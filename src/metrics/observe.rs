use std::io;

use super::Measurement;
use crate::statistics::{Bucket, Statistics};

impl Measurement {
    #[allow(unused_results)] // `.set` and `.set_with_label` are returning a lot of references.
    pub fn observe(mut s: Statistics) -> io::Result<Self> {
        // Roughly equal to the response body size plus some extra capacity
        let mut w = Measurement::with_buffer_capacity(16_834);

        // Common
        w.gauge("num_threads", "The number of threads to create to serve clients")
            .set(s.threads.len())?;

        // Time
        w.counter("time_up_seconds_total", "Uptime since server boot in seconds")
            .set(s.time.up)?;
        w.counter("time_now_seconds", "Current time in seconds since UNIX epoch")
            .set(s.time.now)?;
        w.counter("time_elapsed_seconds", "Time since last statistics printout in seconds")
            .set(s.time.elapsed)?;

        // Memory caches
        w.gauge("memory_caches_bytes", "Memory in bytes in use by caches")
            .set_with_label("cache", "rrset", s.cache.rrset)?
            .set_with_label("cache", "message", s.cache.message)?
            .set_with_label("cache", "dnscrypt_shared_secret", s.cache.dnscrypt_shared_secret)?
            .set_with_label("cache", "dnscrypt_nonce", s.cache.dnscrypt_nonce)?;

        // Memory modules
        w.gauge("memory_modules_bytes", "Memory in bytes in use by modules")
            .set_with_label("module", "iterator", s.modules.iterator)?
            .set_with_label("module", "validator", s.modules.validator)?
            .set_with_label("module", "respip", s.modules.respip)?
            .set_with_label("module", "subnet", s.modules.subnet)?;
        // TODO:
        // .set_with_label("module", "ipsecmod", s.modules.ipsecmod)?

        // Mem buffers
        w.gauge(
            "memory_stream_wait_count",
            "The number of bytes in the stream wait buffers",
        )
        .set(s.mem_streamwait)?;

        w.counter(
            "query_tcp_total",
            "Total number of queries that were made using TCP towards the server",
        )
        .set(s.num_query_tcp)?;
        w.counter(
            "query_tcp_out_total",
            "Total number of queries that were made using TCP outwards the server",
        )
        .set(s.num_query_tcp_out)?;
        w.counter(
            "query_tls_total",
            "Total number of queries that were made using TLS towards the server",
        )
        .set(s.num_query_tls)?;
        w.counter(
            "query_tls_resume_total",
            "Total number of queries that were made using TLS resumption",
        )
        .set(s.num_query_tls_resume)?;
        w.counter(
            "query_ipv6_total",
            "Total number of queries that were made using IPv6 toward the server",
        )
        .set(s.num_query_ipv6)?;

        // Query EDNS numbers
        w.counter(
            "query_edns_DO_total",
            "Total number of queries that had an EDNS OPT record with the DO (DNSSEC OK) bit set present",
        )
        .set(s.num_query_edns_do)?;
        w.counter(
            "query_edns_present_total",
            "Total number of queries that had an EDNS OPT record present",
        )
        .set(s.num_query_edns_present)?;
        // Query iteration numbers
        w.counter(
            "query_ratelimited_total",
            "Total number of queries that had been rate limited",
        )
        .set(s.num_query_rate_limited)?;

        // Query validation numbers
        w.counter("answers_secure_total", "Total amount of answers that were secure (AD)")
            .set(s.num_answer_secure)?;
        // Deprecated version to maintain compatibility
        w.counter(
            "answers_bogus",
            "Total amount of answers that were bogus (withheld as SERVFAIL)",
        )
        .set(s.num_answer_bogus)?;
        w.counter(
            "answers_bogus_total",
            "Total amount of answers that were bogus (withheld as SERVFAIL)",
        )
        .set(s.num_answer_bogus)?;

        w.counter(
            "rrset_bogus_total",
            "Total number of rrsets marked bogus by the validator",
        )
        .set(s.num_rrset_bogus)?;

        // Cache count (deprecated, exposed only to maintain compatibility with `kumina/unbound_exporter`)
        w.gauge("msg_cache_count", "The number of messages cached")
            .set(s.cache_count.message)?;
        w.gauge("rrset_cache_count", "The number of rrset cached")
            .set(s.cache_count.rrset)?;

        // Cache count (new version)
        w.gauge("cache_count_total", "The number of cached entries")
            .set_with_label("type", "message", s.cache_count.message)?
            .set_with_label("type", "rrset", s.cache_count.rrset)?
            .set_with_label("type", "key", s.cache_count.key)?
            .set_with_label("type", "infra", s.cache_count.infra)?
            .set_with_label("type", "dnscrypt_nonce", s.cache_count.dnscrypt_nonce)?
            .set_with_label("type", "dnscrypt_shared_secret", s.cache_count.dnscrypt_shared_secret)?;

        w.counter(
            "unwanted_queries_total",
            "Total number of queries that were refused or dropped because they failed the access control settings.",
        )
        .set(s.num_unwanted_queries)?;
        w.counter(
            "unwanted_replies_total",
            "Total number of replies that were unwanted or unsolicited",
        )
        .set(s.num_unwanted_replies)?;

        let mut answer_rcodes = w.counter(
            "answer_rcodes_total",
            "Total number of answers to queries, from cache or from recursion, by response code.",
        );
        for (rcode, value) in s.answer_rcodes.iter() {
            answer_rcodes.set_with_label("rcode", rcode.as_str(), value)?;
        }
        let mut query_opcodes = w.counter(
            "query_opcodes_total",
            "Total number of queries with a given query opcode",
        );
        for (opcode, value) in s.query_opcodes.iter() {
            query_opcodes.set_with_label("opcode", opcode.as_str(), value)?;
        }
        let mut query_types = w.counter("query_types_total", "Total number of queries with a given query type");
        query_types.set_with_label("type", "other", s.query_types_other)?;
        for (rtype, value) in s.query_types.iter() {
            query_types.set_with_label("type", rtype.as_str(), value)?;
        }
        let mut query_classes = w.counter(
            "query_classes_total",
            "Total number of queries with a given query class",
        );
        query_classes.set_with_label("class", "other", s.query_classes_other)?;
        for (class, value) in s.query_classes.iter() {
            query_classes.set_with_label("class", class.as_str(), value)?;
        }
        w.counter(
            "query_flags_total",
            "Total number of queries that had a given flag set in the header",
        )
        .set_with_label("flag", "QR", s.flags.qr)?
        .set_with_label("flag", "AA", s.flags.aa)?
        .set_with_label("flag", "TC", s.flags.tc)?
        .set_with_label("flag", "RD", s.flags.rd)?
        .set_with_label("flag", "RA", s.flags.ra)?
        .set_with_label("flag", "Z", s.flags.z)?
        .set_with_label("flag", "AD", s.flags.ad)?
        .set_with_label("flag", "CD", s.flags.cd)?;

        // Histogram
        let mut hist = w.histogram("response_time_seconds", "Query response time in seconds");
        hist.sum(s.histogram.sum())?.count(s.histogram.count())?;
        for bucket in s.histogram.buckets() {
            match bucket {
                Bucket::Le(le, value) => hist.bucket(le, value)?,
                Bucket::Inf(value) => hist.bucket("+Inf", value)?,
            };
        }

        // threads
        for (idx, thread) in s.threads.iter().enumerate() {
            let add_header = idx == 0;

            // Queries
            w.counter("queries_total", "Total number of queries received")
                .needs_header(add_header)
                .set_with_label("thread", idx, thread.num_queries)?;
            w.counter(
                "queries_ip_ratelimited_total",
                "Total number of queries rate limited by IP",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.num_queries_ip_ratelimited)?;
            w.counter(
                "cache_hits_total",
                "Total number of queries that were successfully answered using a cache lookup.",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.num_cache_hits)?;
            w.counter(
                "cache_misses_total",
                "Total number of cache queries that needed recursive processing.",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.num_cache_miss)?;
            w.counter("prefetches_total", "Total number of cache prefetches performed")
                .needs_header(add_header)
                .set_with_label("thread", idx, thread.num_prefetch)?;

            // Deprecated since unbound version 1.10.1
            w.counter(
                "zero_ttl_responses_total",
                "Total number of replies with ttl zero, because they served an expired cache entry.",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.num_zero_ttl)?;
            // Added since unbound version 1.10.1
            w.counter(
                "expired_responses_total",
                "Total number of replies that served an expired cache entry.",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.num_zero_ttl)?;

            // TODO:!
            //            w.counter("recursive_replies_total","Total number of replies sent to queries that needed recursive processing")
            //                .set_with_label("thread", idx, thread.mesh_replies_sent)?;

            // DNSCrypt
            w.counter(
                "dnscrypt_valid_queries_total",
                "Total number of queries that were encrypted and successfully decapsulated by dnscrypt",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.num_dnscrypt_crypted)?;
            w.counter(
                "dnscrypt_cert_queries_total",
                "Total number of queries that were requesting dnscrypt certificates",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.num_dnscrypt_cert)?;
            w.counter("dnscrypt_cleartext_queries_total", "Total number of queries received on dnscrypt port that were cleartext and not a request for certificates")
                .needs_header(add_header)
                .set_with_label("thread", idx,thread.num_dnscrypt_cleartext)?;
            w.counter(
                "dnscrypt_malformed_queries_total",
                "Total number of requests that were neither cleartext, not valid dnscrypt messages",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.num_dnscrypt_malformed)?;

            // Request list
            w.gauge(
                "request_list_current_all",
                "Current size of the request list, including internally generated queries",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.requestlist_current_all)?;
            w.gauge(
                "request_list_current_user",
                "Current size of the request list, only counting the requests from client queries",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.requestlist_current_user)?;
            // TODO:
            w.counter(
                "request_list_exceeded_total",
                "Number of queries that were dropped because the request list was full",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.requestlist_exceeded)?;
            w.counter(
                "request_list_overwritten_total",
                "Total number of requests in the request list that were overwritten by newer entries",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.requestlist_overwritten)?;

            // Recursion
            w.gauge("recursion_time_seconds_avg", "Average time it took to answer queries that needed recursive processing (does not include in-cache requests)")
                .needs_header(add_header)
                .set_with_label("thread", idx, thread.recursion_time_avg)?;
            w.gauge(
                "recursion_time_seconds_median",
                "The median of the time it took to answer queries that needed recursive processing",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.recursion_time_median)?;

            // TCP usage
            w.gauge(
                "tcp_usage_current",
                "Number of the currently held TCP buffers for incoming connections",
            )
            .needs_header(add_header)
            .set_with_label("thread", idx, thread.tcp_usage)?;
        }

        Ok(w)
    }
}
