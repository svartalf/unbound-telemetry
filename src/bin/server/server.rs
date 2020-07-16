use std::error::Error;
use std::io;
use std::sync::Arc;
use std::time;

use hyper::service::{make_service_fn, service_fn};
use hyper::{header::HeaderValue, Body, Method, Request, Response, Server, StatusCode};
use unbound_telemetry::{Measurement, RemoteControlSource, Source, TextTransport, TlsTransport};
#[cfg(unix)]
use unbound_telemetry::{SharedMemorySource, UdsTransport};

use crate::cli;

static INDEX_BODY: &str = include_str!("../../../assets/index.html");

struct Context {
    config: cli::Arguments,
    source: Box<dyn Source + Send + Sync + 'static>,
}

async fn handler(req: Request<Body>, context: Arc<Context>) -> hyper::Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        // Landing page
        (&Method::GET, "/") => Ok(Response::new(Body::from(INDEX_BODY))),
        // Observing statistics
        (&Method::GET, path) if path == context.config.common().path => {
            let start = time::Instant::now();
            let observation = context.source.observe().await;
            let elapsed = time::Instant::now().duration_since(start);

            let mut response = observation
                .and_then(|statistics| {
                    let mut m = Measurement::observe(statistics)?;

                    // These two metrics are not related directly to the unbound,
                    // but we want to provide some extra data
                    m.counter("up", "This Unbound instance is up and running").set(1)?;
                    m.gauge("scrape_duration_seconds", "Time spent on metrics scraping")
                        .set(elapsed)?;

                    Ok(m.drain())
                })
                .map(|body| Response::new(Body::from(body)))
                .or_else::<hyper::Error, _>(|e| Ok(render_error(e)))
                .unwrap_or_else(|_| unreachable!("Err variant is excluded by the combinators chain"));

            response
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("text/plain"));

            Ok(response)
        }

        // Healthcheck
        (method, "/healthcheck") if method == Method::HEAD || method == Method::GET => {
            match context.source.healthcheck().await {
                Ok(_) => {
                    log::debug!("Health check completed successfully");
                    Ok(Response::new(Body::empty()))
                }
                Err(e) => Ok(render_error(e)),
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

fn render_error<T: Error + std::fmt::Debug>(e: T) -> Response<Body> {
    log::error!("Unable to observe unbound statistics: {}", e);

    let body = Body::from(format!("# Unable to observe unbound statistics: {}", e));
    let mut response = Response::new(body);
    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

    response
}

fn build_source(config: &cli::Arguments) -> io::Result<Box<dyn Source + Send + Sync + 'static>> {
    let source = match config {
        cli::Arguments::Tcp {
            ca: Some(ca),
            cert: Some(cert),
            key: Some(key),
            interface,
            ..
        } => {
            let transport = TlsTransport::new(ca, cert, key, interface.clone())?;
            let source = RemoteControlSource::new(transport);
            Box::new(source) as Box<_>
        }
        cli::Arguments::Tcp {
            ca: None,
            cert: None,
            key: None,
            interface,
            ..
        } => {
            let transport = TextTransport::new(interface.clone())?;
            let source = RemoteControlSource::new(transport);

            Box::new(source) as Box<_>
        }
        cli::Arguments::Tcp { .. } => unreachable!("CLI validation should handle this case"),
        #[cfg(unix)]
        cli::Arguments::Uds { socket, .. } => {
            let transport = UdsTransport::new(socket);
            let source = RemoteControlSource::new(transport);
            Box::new(source) as Box<_>
        }
        #[cfg(unix)]
        cli::Arguments::Shm { shm_key, .. } => Box::new(SharedMemorySource::new(*shm_key)) as Box<_>,
    };

    Ok(source)
}
