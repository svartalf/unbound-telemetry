#![no_main]
use std::str::FromStr;

use libfuzzer_sys::fuzz_target;

use unbound_telemetry::Statistics;

fuzz_target!(|data: String| {
    let _ = Statistics::from_str(&data);
});
