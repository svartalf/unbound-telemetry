use std::num;
use std::time::Duration;

#[derive(Debug)]
struct InnerBucket {
    le: Duration,
    count: u64,
}

#[derive(Debug)]
pub struct Histogram {
    buckets: Vec<InnerBucket>,
    average: f64,
}

impl Histogram {
    pub fn new(total_recursion_time_avg: f64) -> Self {
        Self {
            buckets: Vec::with_capacity(40),
            average: total_recursion_time_avg,
        }
    }

    pub fn average_mut(&mut self) -> &mut f64 {
        &mut self.average
    }

    pub fn push(&mut self, le: Duration, count: u64) {
        self.buckets.push(InnerBucket { le, count })
    }

    pub fn sum(&self) -> f64 {
        self.average * self.count() as f64
    }

    pub fn count(&self) -> u64 {
        self.buckets
            .iter()
            .map(|bucket| num::Wrapping(bucket.count))
            .sum::<num::Wrapping<u64>>()
            .0
    }

    pub fn buckets(&mut self) -> Buckets<'_> {
        Buckets::new(&mut self.buckets)
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self {
            buckets: Vec::with_capacity(40),
            average: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Buckets<'h> {
    buckets: &'h mut [InnerBucket],
    total: num::Wrapping<u64>,
    current: usize,
    inf_yielded: bool,
}

impl<'h> Buckets<'h> {
    fn new(buckets: &'h mut [InnerBucket]) -> Buckets<'h> {
        buckets.sort_unstable_by_key(|bucket| bucket.le);

        Buckets {
            buckets,
            total: num::Wrapping(0u64),
            current: 0,
            inf_yielded: false,
        }
    }
}

impl<'h> Iterator for Buckets<'h> {
    type Item = Bucket;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.buckets.len() {
            let bucket = &self.buckets[self.current];
            self.current += 1;
            self.total += num::Wrapping(bucket.count);

            Some(Bucket::Le(bucket.le, self.total.0))
        } else if !self.inf_yielded {
            self.inf_yielded = true;

            Some(Bucket::Inf(self.total.0))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Bucket {
    Le(Duration, u64),
    Inf(u64),
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{Bucket, Histogram};

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_histogram() {
        let source = vec![
            // Roughly in the same format what `unbound-control stats_noreset` command outputs
            (Duration::from_secs_f64(000_000.000_001), 0),
            (Duration::from_secs_f64(000_000.000_002), 0),
            (Duration::from_secs_f64(000_000.000_004), 0),
            (Duration::from_secs_f64(000_000.000_008), 0),
            (Duration::from_secs_f64(000_000.000_016), 0),
            (Duration::from_secs_f64(000_000.000_032), 0),
            (Duration::from_secs_f64(000_000.000_064), 0),
            (Duration::from_secs_f64(000_000.000_128), 0),
            (Duration::from_secs_f64(000_000.000_256), 0),
            (Duration::from_secs_f64(000_000.000_512), 0),
            (Duration::from_secs_f64(000_000.001_024), 0),
            (Duration::from_secs_f64(000_000.002_048), 0),
            (Duration::from_secs_f64(000_000.004_096), 0),
            (Duration::from_secs_f64(000_000.008_192), 0),
            (Duration::from_secs_f64(000_000.016_384), 0),
            (Duration::from_secs_f64(000_000.032_768), 0),
            (Duration::from_secs_f64(000_000.065_536), 2),
            (Duration::from_secs_f64(000_000.131_072), 1),
            (Duration::from_secs_f64(000_000.262_144), 1),
            (Duration::from_secs_f64(000_000.524_288), 2),
            (Duration::from_secs_f64(000_001.000_000), 1),
            (Duration::from_secs_f64(000_002.000_000), 0),
            (Duration::from_secs_f64(000_004.000_000), 0),
            (Duration::from_secs_f64(000_008.000_000), 0),
            (Duration::from_secs_f64(000_016.000_000), 0),
            (Duration::from_secs_f64(000_032.000_000), 0),
            (Duration::from_secs_f64(000_064.000_000), 0),
            (Duration::from_secs_f64(000_128.000_000), 0),
            (Duration::from_secs_f64(000_256.000_000), 0),
            (Duration::from_secs_f64(000_512.000_000), 0),
            (Duration::from_secs_f64(001_024.000_000), 0),
            (Duration::from_secs_f64(002_048.000_000), 0),
            (Duration::from_secs_f64(004_096.000_000), 0),
            (Duration::from_secs_f64(008_192.000_000), 0),
            (Duration::from_secs_f64(016_384.000_000), 0),
            (Duration::from_secs_f64(032_768.000_000), 0),
            (Duration::from_secs_f64(065_536.000_000), 0),
            (Duration::from_secs_f64(131_072.000_000), 0),
            (Duration::from_secs_f64(262_144.000_000), 0),
            (Duration::from_secs_f64(524_288.000_000), 0),
        ];
        assert_eq!(source.len(), 40);

        let mut h = Histogram::new(0.280_601);
        for (le, value) in source.into_iter() {
            h.push(le, value);
        }

        approx::assert_relative_eq!(h.sum(), 1.964_207);
        assert_eq!(h.count(), 7);

        let mut buckets = h.buckets();

        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.000_001), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.000_002), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.000_004), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.000_008), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.000_016), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.000_032), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.000_064), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.000_128), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.000_256), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.000_512), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.001_024), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.002_048), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.004_096), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.008_192), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.016_384), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.032_768), 0))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.065_536), 2))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.131_072), 3))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.262_144), 4))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_000.524_288), 6))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_001.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_002.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_004.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_008.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_016.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_032.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_064.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_128.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_256.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(000_512.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(001_024.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(002_048.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(004_096.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(008_192.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(016_384.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(032_768.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(065_536.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(131_072.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(262_144.000_000), 7))
        );
        assert_eq!(
            buckets.next(),
            Some(Bucket::Le(Duration::from_secs_f64(524_288.000_000), 7))
        );
        assert_eq!(buckets.next(), Some(Bucket::Inf(7)));
        assert_eq!(buckets.next(), None);
    }
}
