#![no_main]
use libfuzzer_sys::fuzz_target;

use unbound_telemetry::{Statistics, Measurement};

fuzz_target!(|data: Statistics| {
    let _ = Measurement::observe(data).unwrap();
});
