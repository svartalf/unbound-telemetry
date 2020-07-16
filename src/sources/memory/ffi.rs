/// Structs which are exposed from unbound via shared memory.
///
/// These are Rust versions of the C structs defined at `libunbound/unbound.h`

// Following constants were copied from the `libunbound/unbound.h`
const UB_STATS_QTYPE_NUM: usize = 256;
const UB_STATS_QCLASS_NUM: usize = 256;
const UB_STATS_RCODE_NUM: usize = 16;
const UB_STATS_OPCODE_NUM: usize = 16;
const UB_STATS_BUCKET_NUM: usize = 40;

#[repr(C)]
#[derive(Debug)]
pub struct Time {
    pub now_sec: libc::c_longlong,
    pub now_usec: libc::c_longlong,
    pub up_sec: libc::c_longlong,
    pub up_usec: libc::c_longlong,
    pub elapsed_sec: libc::c_longlong,
    pub elapsed_usec: libc::c_longlong,
}

#[repr(C)]
#[derive(Debug)]
pub struct Memory {
    pub msg: libc::c_longlong,
    pub rrset: libc::c_longlong,
    pub val: libc::c_longlong,
    pub iter: libc::c_longlong,
    pub subnet: libc::c_longlong,
    pub ipsecmod: libc::c_longlong,
    pub respip: libc::c_longlong,
    pub dnscrypt_shared_secret: libc::c_longlong,
    pub dnscrypt_nonce: libc::c_longlong,
}

/// This struct is shared via the shm segment (`shm-key` from `unbound.conf`),
/// maps to `ub_shm_stat_info` from `libunbound/unbound.h`
#[repr(C)]
#[derive(Debug)]
pub struct ShmStatInfo {
    pub num_threads: libc::c_int,
    pub time: Time,
    pub memory: Memory,
}

/// Maps to `ub_server_stats` from `libunbound/unbound.h`
#[repr(C)]
pub struct ServerStats {
    pub num_queries: libc::c_longlong,
    pub num_queries_ip_ratelimited: libc::c_longlong,
    pub num_queries_missed_cache: libc::c_longlong,
    pub num_queries_prefetch: libc::c_longlong,
    pub sum_query_list_size: libc::c_longlong,
    pub max_query_list_size: libc::c_longlong,
    pub extended: libc::c_int,
    pub qtype: [libc::c_longlong; UB_STATS_QTYPE_NUM],
    pub qtype_big: libc::c_longlong,
    pub qclass: [libc::c_longlong; UB_STATS_QCLASS_NUM],
    pub qclass_big: libc::c_longlong,
    pub qopcode: [libc::c_longlong; UB_STATS_OPCODE_NUM],
    pub qtcp: libc::c_longlong,
    pub qtcp_outgoing: libc::c_longlong,
    pub qtls: libc::c_longlong,
    pub qipv6: libc::c_longlong,
    pub qbit_qr: libc::c_longlong,
    pub qbit_aa: libc::c_longlong,
    pub qbit_tc: libc::c_longlong,
    pub qbit_rd: libc::c_longlong,
    pub qbit_ra: libc::c_longlong,
    pub qbit_z: libc::c_longlong,
    pub qbit_ad: libc::c_longlong,
    pub qbit_cd: libc::c_longlong,
    pub qedns: libc::c_longlong,
    pub qedns_do: libc::c_longlong,
    pub ans_rcode: [libc::c_longlong; UB_STATS_RCODE_NUM],
    pub ans_rcode_nodata: libc::c_longlong,
    pub ans_secure: libc::c_longlong,
    pub ans_bogus: libc::c_longlong,
    pub rrset_bogus: libc::c_longlong,
    pub queries_ratelimited: libc::c_longlong,
    pub unwanted_replies: libc::c_longlong,
    pub unwanted_queries: libc::c_longlong,
    pub tcp_accept_usage: libc::c_longlong,

    // TODO: Field was renamed in unbound 1.10.1
    //
    // - /** answers served from expired cache */
    // - long long zero_ttl_responses;
    // + /** expired answers served from cache */
    // + long long ans_expired;
    //
    // See https://github.com/NLnetLabs/unbound/commit/f7fe95ad7bae690781f9b78ca252a44fc072ca33
    pub zero_ttl_responses: libc::c_longlong,

    pub hist: [libc::c_longlong; UB_STATS_BUCKET_NUM],
    pub msg_cache_count: libc::c_longlong,
    pub rrset_cache_count: libc::c_longlong,
    pub infra_cache_count: libc::c_longlong,
    pub key_cache_count: libc::c_longlong,
    pub num_query_dnscrypt_crypted: libc::c_longlong,
    pub num_query_dnscrypt_cert: libc::c_longlong,
    pub num_query_dnscrypt_cleartext: libc::c_longlong,
    pub num_query_dnscrypt_crypted_malformed: libc::c_longlong,
    pub num_query_dnscrypt_secret_missed_cache: libc::c_longlong,
    pub shared_secret_cache_count: libc::c_longlong,
    pub num_query_dnscrypt_replay: libc::c_longlong,
    pub nonce_cache_count: libc::c_longlong,
    pub num_query_authzone_up: libc::c_longlong,
    pub num_query_authzone_down: libc::c_longlong,
    pub num_neg_cache_noerror: libc::c_longlong,
    pub num_neg_cache_nxdomain: libc::c_longlong,
    pub num_query_subnet: libc::c_longlong,
    pub num_query_subnet_cache: libc::c_longlong,
    pub mem_stream_wait: libc::c_longlong,
    pub qtls_resume: libc::c_longlong,
}

/// This struct is shared via the shm segment (`shm-key + 1` from `unbound.conf`),
/// maps to `ub_stats_info` from `libunbound/unbound.h`
pub struct StatsInfo {
    pub svr: ServerStats,
    pub mesh_num_states: libc::c_longlong,
    pub mesh_num_reply_states: libc::c_longlong,
    pub mesh_jostled: libc::c_longlong,
    pub mesh_dropped: libc::c_longlong,
    pub mesh_replies_sent: libc::c_longlong,
    pub mesh_replies_sum_wait_sec: libc::c_longlong,
    pub mesh_replies_sum_wait_usec: libc::c_longlong,
    pub mesh_time_median: libc::c_double,
}
