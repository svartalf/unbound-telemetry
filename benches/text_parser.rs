#![feature(test)]

extern crate test;

use std::str::FromStr;
use test::Bencher;

use unbound_telemetry::Statistics;

static STATS: &str = include_str!("../assets/test_text_stats.txt");

#[bench]
fn bench_parser(b: &mut Bencher) {
    b.iter(|| Statistics::from_str(STATS));
}
