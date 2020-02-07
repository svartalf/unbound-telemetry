# Comparison with `kumina/unbound_exporter`

While this exporter tries its best to mimic the [kumina/unbound_exporter](https://github.com/kumina/unbound_exporter) behavior,
there are few notable differences that should be noted.

## Added

A lot of additional metrics from the `unbound`, see their help annotations for details.

### Global metrics

 * `unbound_num_threads`
 * `unbound_memory_stream_wait_count`
 * `unbound_query_ratelimited_total`
 * `unbound_query_tcp_out_total`
 * `unbound_query_tls_resume_total`
 * `unbound_cache_count_total`
 * `unbound_memory_modules_bytes{module="ipsecmod"}`

### Per-thread metrics

 * `unbound_dnscrypt_cert_queries_total`
 * `unbound_dnscrypt_cleartext_queries_total`
 * `unbound_dnscrypt_malformed_queries_total`
 * `unbound_dnscrypt_valid_queries_total`
 * `unbound_queries_ip_ratelimited_total`
 * `unbound_zero_ttl_responses_total`

### Technical metrics

 * `unbound_scrape_duration_seconds`


## Deprecated

 * `unbound_answers_bogus`: use `unbound_answers_bogus_total` instead
 * `unbound_msg_cache_count`: use `unbound_cache_count_total{type="message"}` instead
 * `unbound_rrset_cache_count`: use `unbound_cache_count_total{type="rrset"}` instead


## TODO

 * `unbound_dnscrypt_query_replays_total`
 * `unbound_dnscrypt_valid_queries_total`
 * `unbound_query_aggressive_total`
 * `unbound_query_authzone_down_total`
 * `unbound_query_authzone_up_total`
 * `unbound_query_subnet_cache_total`
 * `unbound_query_subnet_total`
 * `unbound_query_tcp_out_total`
 * `unbound_query_tls_resume_total`
 * `unbound_tcp_usage_current`
 * `unbound_zero_ttl_responses_total`
