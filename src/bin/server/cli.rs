use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(structopt::StructOpt, Debug, Clone)]
pub struct Common {
    /// Address to listen on for HTTP interface
    #[structopt(short = "b", long = "bind", default_value = "0.0.0.0:9167", global = true)]
    pub bind: SocketAddr,

    /// HTTP path to expose metrics
    #[structopt(short = "p", long = "path", default_value = "/metrics", global = true)]
    pub path: String,

    /// Set the log level to run under.
    ///
    /// Possible values are: error, warn, info, debug, trace
    #[structopt(
        name = "log-level",
        short = "l",
        long = "log-level",
        default_value = "info",
        global = true,
        parse(try_from_str)
    )]
    pub log_level: log::Level,
}

#[derive(structopt::StructOpt, Debug)]
#[structopt(name = "unbound-telemetry")]
pub enum Arguments {
    /// Fetch unbound statistics from the shared memory region.
    ///
    /// Available for UNIX systems only.
    #[cfg(unix)]
    Shm {
        /// Shared memory key (`shm-key` value from the `unbound.conf`)
        #[structopt(name = "KEY", short = "k", long = "shm-key", default_value = "11777")]
        shm_key: libc::key_t,
        #[structopt(flatten)]
        common: Common,
    },
    /// Fetch unbound statistics from the remote control Unix socket.
    ///
    /// Available for UNIX systems only.
    #[cfg(unix)]
    Uds {
        /// Local socket path.
        #[structopt(name = "socket", long = "control-interface")]
        socket: PathBuf,
        #[structopt(flatten)]
        common: Common,
    },
    /// Fetch unbound statistics from the remote control TCP socket.
    ///
    /// Certificate and key options can be omitted
    /// if TLS is disabled for remote control socket.
    Tcp {
        /// Server certificate file.
        #[structopt(
            name = "CA_FILE",
            long = "server-cert-file",
            requires_all = &["CERT_FILE", "KEY_FILE"]
        )]
        ca: Option<PathBuf>,

        /// Server control certificate file.
        #[structopt(name = "CERT_FILE", long = "control-cert-file")]
        cert: Option<PathBuf>,

        /// Control client private key.
        #[structopt(name = "KEY_FILE", long = "control-key-file")]
        key: Option<PathBuf>,

        /// TLS socket hostname.
        #[structopt(name = "interface", long = "control-interface", default_value = "127.0.0.1:8953")]
        // Note that at this point we are not using `SocketAddr` type,
        // because we might need to do the DNS resolving later.
        interface: String,

        #[structopt(flatten)]
        common: Common,
    },
}

impl Arguments {
    // Common settings are accessed via method,
    // because neither `clap` or `structopt` right now
    // are not allowing to create "global" arguments,
    // which will be shared among all subcommands.
    pub fn common(&self) -> &Common {
        match self {
            Arguments::Tcp { common, .. } => common,
            #[cfg(unix)]
            Arguments::Shm { common, .. } => common,
            #[cfg(unix)]
            Arguments::Uds { common, .. } => common,
        }
    }
}
