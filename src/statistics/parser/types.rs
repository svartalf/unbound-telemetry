use std::time::Duration;

use domain::base::iana::Rcode;

use crate::statistics::ParseError;

pub(crate) trait Field: Sized {
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
        const MAX_NANOS_F64: f64 = ((u64::max_value() as u128 + 1) * NANOS_PER_SEC) as f64;

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

/// `Rcode` enum from `domain` crate does not implement `FromStr`,
/// but we need it to parse textual representation from `unbound`.
///
/// Considering that text format is known, current implementation
/// is pretty simple and even skips the case-sensitive checks.
pub(crate) fn parse_rcode(s: &str) -> Result<Rcode, ParseError> {
    match s {
        "NOERROR" => Ok(Rcode::NoError),
        "FORMERR" => Ok(Rcode::FormErr),
        "SERVFAIL" => Ok(Rcode::ServFail),
        "NXDOMAIN" => Ok(Rcode::NXDomain),
        "NOTIMPL" => Ok(Rcode::NotImp),
        "REFUSED" => Ok(Rcode::Refused),
        "YXDOMAIN" => Ok(Rcode::YXDomain),
        "YXRRSET" => Ok(Rcode::YXRRSet),
        "NXRRSET" => Ok(Rcode::NXRRSet),
        "NOTAUTH" => Ok(Rcode::NotAuth),
        "NOTZONE" => Ok(Rcode::NotZone),
        _ => Err(ParseError::ParseStr(s.to_owned())),
    }
}
