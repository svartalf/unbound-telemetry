use std::error::Error;

use structopt::StructOpt;

mod cli;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config = self::cli::Arguments::from_args();
    simple_logger::init_with_level(config.common().log_level)?;
    self::server::serve(config).await?;

    Ok(())
}
