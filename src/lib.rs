#![deny(
    unused,
    unused_imports,
    unused_features,
    bare_trait_objects,
    future_incompatible,
    nonstandard_style,
    dead_code,
    deprecated,
    broken_intra_doc_links
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results
)]

#[macro_use]
mod macros;
mod metrics;
mod sources;
pub mod statistics;

pub use self::metrics::Measurement;
pub use self::sources::{RemoteControlSource, Source, TextTransport, TlsTransport};
#[cfg(unix)]
pub use self::sources::{SharedMemorySource, UdsTransport};
pub use self::statistics::{ParseError, Statistics};
