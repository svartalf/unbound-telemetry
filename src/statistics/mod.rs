use std::collections::HashMap;
use std::str;
use std::time::Duration;

use domain::base::iana::{Class, Opcode, Rcode, Rtype};

mod histogram;
mod parser;

pub use self::histogram::{Bucket, Histogram};
pub use self::parser::ParseError;

/// Statistics snapshot received from some data source.
///
/// It is decoupled from any data layout or format exposed by `unbound`
/// and mostly exists only to make sure that all keys are provided by all the data sources.
#[derive(Debug, Default)]
pub struct Statistics {
    pub total: Thread,
    pub threads: Vec<Thread>,
    pub time: Time,
    pub cache: Cache,
    pub modules: Modules,
    pub cache_count: CacheCounter,
    pub http: Http,
    pub flags: Flags,
    pub query_opcodes: HashMap<Opcode, u64>,
    pub query_types: HashMap<Rtype, u64>,
    // All other `Rtype` entries higher than `UB_STATS_QTYPE_NUM` (declared in `unbound.h`)
    // are summed together into one metric value.
    // As they are not representing any specific `Rtype`, storing them separately in here too.
    pub query_types_other: u64,
    pub query_classes: HashMap<Class, u64>,
    // See `query_types_other` comment for motivation to have this separate field.
    pub query_classes_other: u64,
    pub answer_rcodes: HashMap<Rcode, u64>,
    pub query_aggressive: HashMap<Rcode, u64>,
    pub histogram: Histogram,
    pub mem_streamwait: u64,
    pub num_query_tcp: u64,
    pub num_query_tcp_out: u64,
    pub num_query_tls: u64,
    pub num_query_tls_resume: u64,
    pub num_query_ipv6: u64,
    pub num_query_edns_present: u64,
    pub num_query_edns_do: u64,
    pub num_query_rate_limited: u64,
    pub num_query_https: u64,
    pub num_answer_secure: u64,
    pub num_answer_bogus: u64,
    pub num_rrset_bogus: u64,
    pub num_unwanted_queries: u64,
    pub num_unwanted_replies: u64,
    pub num_query_dnscrypt_shared_secret_cache_miss: u64,
    pub num_query_dnscrypt_replay: u64,
    pub num_query_authzone_up: u64,
    pub num_query_authzone_down: u64,
    pub num_query_subnet: u64,
    pub num_query_subnet_cache: u64,
}

impl str::FromStr for Statistics {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = self::parser::Parser::new();
        parser.parse(s)
    }
}

/// Thread related data.
#[derive(Debug, Default)]
pub struct Thread {
    // Num
    pub num_queries: u64,
    pub num_queries_ip_ratelimited: u64,
    pub num_cache_hits: u64,
    pub num_cache_miss: u64,
    pub num_prefetch: u64,
    pub num_zero_ttl: u64,
    pub num_recursive_replies: u64,
    pub num_dnscrypt_crypted: u64,
    pub num_dnscrypt_cert: u64,
    pub num_dnscrypt_cleartext: u64,
    pub num_dnscrypt_malformed: u64,
    pub requestlist_avg: f64,
    pub requestlist_max: u64,
    pub requestlist_overwritten: u64,
    pub requestlist_exceeded: u64,
    pub requestlist_current_all: u64,
    pub requestlist_current_user: u64,
    pub recursion_time_avg: f64,
    pub recursion_time_median: f64,
    pub tcp_usage: u64,
    pub answer_rcode: HashMap<Rcode, u64>,
}

#[derive(Debug, Default)]
pub struct Time {
    pub now: Duration,
    pub up: Duration,
    pub elapsed: Duration,
}

#[derive(Debug, Default)]
pub struct Cache {
    pub rrset: u64,
    pub message: u64,
    pub dnscrypt_shared_secret: u64,
    pub dnscrypt_nonce: u64,
}

#[derive(Debug, Default)]
pub struct Modules {
    pub iterator: u64,
    pub validator: u64,
    pub respip: u64,
    pub subnet: u64,
}

#[derive(Debug, Default)]
pub struct CacheCounter {
    pub message: u64,
    pub rrset: u64,
    pub infra: u64,
    pub key: u64,
    pub dnscrypt_shared_secret: u64,
    pub dnscrypt_nonce: u64,
}

#[derive(Debug, Default)]
pub struct Flags {
    pub qr: u64,
    pub aa: u64,
    pub tc: u64,
    pub rd: u64,
    pub ra: u64,
    pub z: u64,
    pub ad: u64,
    pub cd: u64,
}

#[derive(Debug, Default)]
pub struct Http {
    pub query_buffer: u64,
    pub response_buffer: u64,
}
