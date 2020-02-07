use std::error::Error;
use std::io;
use std::sync::Arc;
use std::time;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use unbound_telemetry::{Measurement, Source, TlsSource};
#[cfg(unix)]
use unbound_telemetry::{SharedMemorySource, UdsSource};

use crate::cli;

static INDEX_BODY: &str = include_str!("../../../assets/index.html");

struct Context {
    config: cli::Arguments,
    source: Box<dyn Source + Send + Sync + 'static>,
}

async fn handler(req: Request<Body>, context: Arc<Context>) -> hyper::Result<Response<Body>> {
    // False positive on `&Method::GET`,
    // see https://github.com/rust-lang/rust/issues/62411#issuecomment-510604193
    #[allow(future_incompatible)]
    match (req.method(), req.uri().path()) {
        // Index page
        (&Method::GET, "/") => Ok(Response::new(Body::from(INDEX_BODY))),
        (&Method::GET, path) if path == context.config.common().path => {
            let start = time::Instant::now();
            let observation = context.source.observe().await;
            let elapsed = time::Instant::now().duration_since(start);

            let body = observation
                .and_then(|statistics| {
                    let mut m = Measurement::observe(statistics)?;
                    m.gauge("scrape_duration_seconds", "Time spent on metrics scraping")
                        .set(elapsed)?;

                    Ok(m.drain())
                })
                .map(Body::from)
                .or_else::<hyper::Error, _>(|e| Ok(render_error(e)))
                .expect("Err variant is excluded by the combinators chain");

            Ok(Response::new(body))
        }
        // Healthcheck endpoint
        (method, "/healthcheck") if method == Method::HEAD || method == Method::GET => {
            match context.source.healthcheck().await {
                Ok(_) => {
                    log::debug!("Health check completed successfully");
                    Ok(Response::new(Body::empty()))
                }
                Err(e) => Ok(Response::new(render_error(e))),
            }
        }

        // Fallback for any other requests, results in plain 404
        _ => {
            let mut resp = Response::new(Body::empty());
            *resp.status_mut() = StatusCode::NOT_FOUND;
            Ok(resp)
        }
    }
}

pub async fn serve(config: cli::Arguments) -> Result<(), Box<dyn Error + Send + Sync>> {
    let server_config = (*config.common()).clone();
    let source = build_source(&config)?;

    let context = Arc::new(Context { config, source });
    let service = make_service_fn(move |_| {
        let handler_context = context.clone();

        async {
            let f = service_fn(move |req| handler(req, handler_context.clone()));

            Ok::<_, hyper::Error>(f)
        }
    });

    let ctrl_c = tokio::signal::ctrl_c();
    let server = Server::bind(&server_config.bind)
        .serve(service)
        .with_graceful_shutdown(async {
            let _ = ctrl_c.await;
        });
    log::info!("Listening on {}", server_config.bind);
    server.await?;

    Ok(())
}

fn render_error<T: Error + std::fmt::Debug>(e: T) -> Body {
    log::error!("Unable to observe unbound statistics: {}", e);

    let body = format!("# Unable to observe unbound statistics: {}", e);
    Body::from(body)
}

fn build_source(config: &cli::Arguments) -> io::Result<Box<dyn Source + Send + Sync + 'static>> {
    let source = match config {
        cli::Arguments::Tls {
            ca,
            cert,
            key,
            interface,
            ..
        } => {
            let source = TlsSource::new(ca, cert, key, interface.clone())?;
            Box::new(source) as Box<_>
        }
        #[cfg(unix)]
        cli::Arguments::Uds { socket, .. } => Box::new(UdsSource::new(socket)) as Box<_>,
        #[cfg(unix)]
        cli::Arguments::Shm { shm_key, .. } => Box::new(SharedMemorySource::new(*shm_key)) as Box<_>,
    };

    Ok(source)
}
