use std::str::FromStr;
use std::time::Duration;
use std::u64;

mod errors;

pub use self::errors::ParseError;
use super::{Class, Opcode, Rcode, Rtype, Statistics, Thread};
use crate::statistics::Histogram;

/// Parser for [`Statistics`] from the string representation.
///
/// This representation can be obtained from the Unix or TLS socket.
/// Alternatively, it can be received from `unbound-control stats_noreset`
/// (or other `stats_*` commands), but these data sources will not be supported.
#[derive(Debug)]
pub struct Parser {
    stats: Statistics,
}

impl Parser {
    pub fn new() -> Parser {
        Parser::default()
    }

    pub fn parse(mut self, s: &str) -> Result<Statistics, ParseError> {
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match self.feed_line(line) {
                Ok(..) => {}
                Err(ParseError::UnknownKey { .. }) => {
                    log::warn!("Unable to parse '{}', unknown key", line);
                }
                Err(e) => return Err(e),
            }
        }

        *self.stats.histogram.average_mut() = self.stats.total.recursion_time_avg;

        self.finish()
    }

    // For now assuming that all data was provided correctly
    pub fn finish(self) -> Result<Statistics, ParseError> {
        Ok(self.stats)
    }

    pub fn feed_line(&mut self, line: &str) -> Result<(), ParseError> {
        let mut parts = line.splitn(2, '=');
        let key = parts.next().ok_or_else(|| ParseError::MissingKey)?;
        let value = parts
            .next()
            .ok_or_else(|| ParseError::MissingValue { key: key.into() })?;
        let mut key_parts = key.splitn(2, '.');
        let key_prefix = key_parts.next().ok_or_else(|| ParseError::InvalidFormat)?;
        let key_postfix = key_parts.next().ok_or_else(|| ParseError::InvalidFormat)?;

        match key_prefix {
            "total" => Self::thread(&mut self.stats.total, key_postfix, value)?,
            prefix if prefix.starts_with("thread") => {
                // TODO: Unnecessary allocation
                let thread_id = prefix
                    .chars()
                    .skip("thread".len())
                    .collect::<String>()
                    .parse::<usize>()?;

                if cfg!(fuzzing) && thread_id > 255 {
                    return Err(ParseError::InvalidFormat);
                }

                self.stats.threads.resize_with(thread_id + 1, Default::default);
                let mut thread = self
                    .stats
                    .threads
                    .get_mut(thread_id)
                    .expect("Can't happen, resize_with will handle that");

                Self::thread(&mut thread, key_postfix, value)?
            }
            "histogram" => Self::histogram(&mut self.stats.histogram, key_postfix, value)?,
            _ => Self::other(&mut self.stats, key, value)?,
        }

        Ok(())
    }

    fn thread(thread: &mut Thread, key: &str, value: &str) -> Result<(), ParseError> {
        match key {
            "num.queries" => thread.num_queries.parse(value),
            "num.queries_ip_ratelimited" => thread.num_queries_ip_ratelimited.parse(value),
            "num.cachehits" => thread.num_cache_hits.parse(value),
            "num.cachemiss" => thread.num_cache_miss.parse(value),
            "num.prefetch" => thread.num_prefetch.parse(value),

            // Metric name before unbound version 1.10.1
            "num.zero_ttl" => thread.num_zero_ttl.parse(value),
            // Metric name after unbound version 1.10.1
            // see https://github.com/NLnetLabs/unbound/commit/f7fe95ad7bae690781f9b78ca252a44fc072ca33
            "num.expired" => thread.num_zero_ttl.parse(value),

            "num.recursivereplies" => thread.num_recursive_replies.parse(value),
            "num.dnscrypt.crypted" => thread.num_dnscrypt_crypted.parse(value),
            "num.dnscrypt.cert" => thread.num_dnscrypt_cert.parse(value),
            "num.dnscrypt.cleartext" => thread.num_dnscrypt_cleartext.parse(value),
            "num.dnscrypt.malformed" => thread.num_dnscrypt_malformed.parse(value),
            "requestlist.avg" => thread.requestlist_avg.parse(value),
            "requestlist.max" => thread.requestlist_max.parse(value),
            "requestlist.overwritten" => thread.requestlist_overwritten.parse(value),
            "requestlist.exceeded" => thread.requestlist_exceeded.parse(value),
            "requestlist.current.all" => thread.requestlist_current_all.parse(value),
            "requestlist.current.user" => thread.requestlist_current_user.parse(value),
            "recursion.time.avg" => thread.recursion_time_avg.parse(value),
            "recursion.time.median" => thread.recursion_time_median.parse(value),
            "tcpusage" => thread.tcp_usage.parse(value),
            _ => Err(ParseError::UnknownKey { key: key.into() }),
        }
    }

    fn other(stats: &mut Statistics, key: &str, value: &str) -> Result<(), ParseError> {
        match key {
            "time.now" => stats.time.now.parse(value),
            "time.up" => stats.time.up.parse(value),
            "time.elapsed" => stats.time.elapsed.parse(value),
            "mem.cache.rrset" => stats.cache.rrset.parse(value),
            "mem.cache.message" => stats.cache.message.parse(value),
            "mem.mod.iterator" => stats.modules.iterator.parse(value),
            "mem.mod.validator" => stats.modules.validator.parse(value),
            "mem.mod.respip" => stats.modules.respip.parse(value),
            "mem.mod.subnet" => stats.modules.subnet.parse(value),
            "mem.cache.dnscrypt_shared_secret" => stats.cache.dnscrypt_shared_secret.parse(value),
            "mem.cache.dnscrypt_nonce" => stats.cache.dnscrypt_nonce.parse(value),
            "mem.streamwait" => stats.mem_streamwait.parse(value),
            "num.query.type.other" => stats.query_types_other.parse(value),
            key if key.starts_with("num.query.type.") => {
                let mut parts = key.rsplitn(2, '.');
                let raw_code = parts.next().ok_or_else(|| ParseError::InvalidFormat)?;
                let code = Rtype::from_str(raw_code).map_err(|_| ParseError::ParseStr(raw_code.into()))?;
                let value = value.parse::<u64>()?;
                let _ = stats.query_types.insert(code, value);
                Ok(())
            }
            "num.query.class.other" => stats.query_classes_other.parse(value),
            key if key.starts_with("num.query.class.") => {
                let mut parts = key.rsplitn(2, '.');
                let raw_code = parts.next().ok_or_else(|| ParseError::InvalidFormat)?;
                let code = Class::from_str(raw_code).map_err(|_| ParseError::ParseStr(raw_code.into()))?;
                let value = value.parse::<u64>()?;
                let _ = stats.query_classes.insert(code, value);
                Ok(())
            }
            key if key.starts_with("num.query.opcode.") => {
                let mut parts = key.rsplitn(2, '.');
                let raw_code = parts.next().ok_or_else(|| ParseError::InvalidFormat)?;
                let code = Opcode::from_str(raw_code).map_err(|_| ParseError::ParseStr(raw_code.into()))?;
                let value = value.parse::<u64>()?;
                let _ = stats.query_opcodes.insert(code, value);
                Ok(())
            }
            "num.query.tcp" => stats.num_query_tcp.parse(value),
            "num.query.tcpout" => stats.num_query_tcp_out.parse(value),
            "num.query.tls" => stats.num_query_tls.parse(value),
            "num.query.tls.resume" => stats.num_query_tls_resume.parse(value),
            "num.query.ipv6" => stats.num_query_ipv6.parse(value),
            "num.query.flags.QR" => stats.flags.qr.parse(value),
            "num.query.flags.AA" => stats.flags.aa.parse(value),
            "num.query.flags.TC" => stats.flags.tc.parse(value),
            "num.query.flags.RD" => stats.flags.rd.parse(value),
            "num.query.flags.RA" => stats.flags.ra.parse(value),
            "num.query.flags.Z" => stats.flags.z.parse(value),
            "num.query.flags.AD" => stats.flags.ad.parse(value),
            "num.query.flags.CD" => stats.flags.cd.parse(value),
            "num.query.edns.present" => stats.num_query_edns_present.parse(value),
            "num.query.edns.DO" => stats.num_query_edns_do.parse(value),
            // `rcode.nodata` is ignored, same to `kumina/unbound_exporter`
            "num.answer.rcode.nodata" => Ok(()),
            key if key.starts_with("num.answer.rcode.") => {
                let mut parts = key.rsplitn(2, '.');
                let raw_code = parts.next().ok_or_else(|| ParseError::InvalidFormat)?;
                let code = Rcode::from_str(raw_code).map_err(|_| ParseError::ParseStr(raw_code.into()))?;
                let value = value.parse::<u64>()?;
                let _ = stats.answer_rcodes.insert(code, value);
                Ok(())
            }
            "num.query.ratelimited" => stats.num_query_rate_limited.parse(value),
            "num.answer.secure" => stats.num_answer_secure.parse(value),
            "num.answer.bogus" => stats.num_answer_bogus.parse(value),
            "num.rrset.bogus" => stats.num_rrset_bogus.parse(value),
            key if key.starts_with("num.query.aggressive.") => {
                let mut parts = key.rsplitn(2, '.');
                let raw_code = parts.next().ok_or_else(|| ParseError::InvalidFormat)?;
                let code = Rcode::from_str(raw_code).map_err(|_| ParseError::ParseStr(raw_code.into()))?;
                let value = value.parse::<u64>()?;
                let _ = stats.query_aggressive.insert(code, value);
                Ok(())
            }
            "unwanted.queries" => stats.num_unwanted_queries.parse(value),
            "unwanted.replies" => stats.num_unwanted_replies.parse(value),
            "msg.cache.count" => stats.cache_count.message.parse(value),
            "rrset.cache.count" => stats.cache_count.rrset.parse(value),
            "infra.cache.count" => stats.cache_count.infra.parse(value),
            "key.cache.count" => stats.cache_count.key.parse(value),
            "dnscrypt_shared_secret.cache.count" => stats.cache_count.dnscrypt_shared_secret.parse(value),
            "dnscrypt_nonce.cache.count" => stats.cache_count.dnscrypt_nonce.parse(value),
            "num.query.dnscrypt.shared_secret.cachemiss" => {
                stats.num_query_dnscrypt_shared_secret_cache_miss.parse(value)
            }
            "num.query.dnscrypt.replay" => stats.num_query_dnscrypt_replay.parse(value),
            "num.query.authzone.up" => stats.num_query_authzone_up.parse(value),
            "num.query.authzone.down" => stats.num_query_authzone_down.parse(value),
            "num.query.subnet" => stats.num_query_subnet.parse(value),
            "num.query.subnet_cache" => stats.num_query_subnet_cache.parse(value),
            _ => Err(ParseError::UnknownKey { key: key.into() }),
        }
    }

    fn histogram(hist: &mut Histogram, key: &str, value: &str) -> Result<(), ParseError> {
        let mut parts = key.splitn(4, '.').skip(3);
        let time = parts.next().ok_or(ParseError::InvalidFormat)?.parse::<f64>()?;

        let duration = Duration::checked_from_secs_f64(time).ok_or(ParseError::InvalidFormat)?;
        let value = value.parse()?;

        hist.push(duration, value);

        Ok(())
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            stats: Statistics::default(),
        }
    }
}

trait Field: Sized {
    fn parse(&mut self, s: &str) -> Result<(), ParseError>;
}

impl Field for u64 {
    fn parse(&mut self, s: &str) -> Result<(), ParseError> {
        *self = s.parse().map_err(ParseError::from)?;
        Ok(())
    }
}

impl Field for f64 {
    fn parse(&mut self, s: &str) -> Result<(), ParseError> {
        *self = s.parse().map_err(ParseError::from)?;
        Ok(())
    }
}

impl Field for Duration {
    fn parse(&mut self, s: &str) -> Result<(), ParseError> {
        let value = s.parse::<f64>()?;

        match Duration::checked_from_secs_f64(value) {
            Some(duration) => {
                *self = duration;
                Ok(())
            }
            None => Err(ParseError::InvalidFormat),
        }
    }
}

pub trait DurationExt {
    /// Following is a rip-off of the `Duration::from_secs_f64` method,
    /// since there is is no `Duration::checked_from_secs_f64`
    /// and we can't afford to panic
    #[inline]
    fn checked_from_secs_f64(secs: f64) -> Option<Duration> {
        const NANOS_PER_SEC: u128 = 1_000_000_000;
        const MAX_NANOS_F64: f64 = ((u64::MAX as u128 + 1) * NANOS_PER_SEC) as f64;

        let nanos = secs * (NANOS_PER_SEC as f64);
        if !nanos.is_finite() {
            return None;
        }
        if nanos >= MAX_NANOS_F64 {
            return None;
        }
        if nanos < 0.0 {
            return None;
        }
        let nanos = nanos as u128;
        Some(Duration::new(
            (nanos / NANOS_PER_SEC) as u64,
            (nanos % NANOS_PER_SEC) as u32,
        ))
    }
}

impl DurationExt for Duration {}

#[cfg(test)]
mod tests;
