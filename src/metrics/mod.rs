//! Prometheus metrics format implementation.
//!
//! ## Motivation
//!
//! Currently existing Prometheus crates (`metrics`, `prometheus`, or `opentelemetry`)
//! lacks the ability to collect the measurements from the so-called const metrics
//! (where data is provided from external source and should be set directly as metric value,
//! instead of using something like `Counter::add`).
//!
//! In addition, there is no need to store these values somewhere in a global registry,
//! because they are ephemeral: they are representing system state at the point
//! when request were received and we will not need them later.
//!
//! Since Prometheus text format is quite simple, it is easier to re-implement it
//! and do a quick and dirty writes directly into the output buffer.

use std::io;

mod observe;
mod value;

use self::value::MetricValue;

#[must_use]
#[derive(Default)]
pub struct Measurement(Vec<u8>);

impl Measurement {
    pub fn with_buffer_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn counter(&mut self, name: &'static str, help: &'static str) -> MetricGuard<'_, Vec<u8>> {
        MetricGuard::new(&mut self.0, name, "counter", help)
    }

    pub fn gauge(&mut self, name: &'static str, help: &'static str) -> MetricGuard<'_, Vec<u8>> {
        MetricGuard::new(&mut self.0, name, "gauge", help)
    }

    pub fn histogram(&mut self, name: &'static str, help: &'static str) -> HistogramGuard<'_, Vec<u8>> {
        HistogramGuard::new(&mut self.0, name, help)
    }

    pub fn drain(self) -> Vec<u8> {
        self.0
    }
}

#[must_use]
pub struct MetricGuard<'t, T>
where
    T: io::Write + 't,
{
    w: &'t mut T,
    name: &'static str,
    // metric kind and help text
    header: Option<(&'static str, &'static str)>,
}

impl<'t, T> MetricGuard<'t, T>
where
    T: io::Write + 't,
{
    pub fn new(w: &'t mut T, name: &'static str, kind: &'static str, help: &'static str) -> Self {
        Self {
            w,
            name,
            header: Some((kind, help)),
        }
    }

    pub fn set<V>(&mut self, value: V) -> io::Result<&mut Self>
    where
        V: MetricValue,
    {
        self.ensure_header()?;

        self.w.write_fmt(format_args!("unbound_{} ", self.name))?;
        value.write(&mut self.w)?;
        self.w.write_all(b"\n")?;

        Ok(self)
    }

    pub fn set_with_label<L, V>(&mut self, key: &'static str, label: L, value: V) -> io::Result<&mut Self>
    where
        L: MetricValue,
        V: MetricValue,
    {
        self.ensure_header()?;

        self.w.write_fmt(format_args!("unbound_{}{{{}=\"", self.name, key))?;
        label.write(&mut self.w)?;
        self.w.write_all(b"\"} ")?;
        value.write(&mut self.w)?;
        self.w.write_all(b"\n")?;

        Ok(self)
    }

    pub fn needs_header(&mut self, value: bool) -> &mut Self {
        if !value {
            let _ = self.header.take();
        }

        self
    }

    fn ensure_header(&mut self) -> io::Result<()> {
        match self.header.take() {
            Some((kind, help)) => self.w.write_fmt(format_args!(
                "# TYPE unbound_{name} {kind}\n# HELP unbound_{name} {help}\n",
                name = self.name,
                kind = kind,
                help = help
            )),
            None => Ok(()),
        }
    }
}

#[must_use]
pub struct HistogramGuard<'t, T>
where
    T: io::Write + 't,
{
    w: &'t mut T,
    name: &'static str,
    // metric help text
    header: Option<&'static str>,
}

impl<'t, T> HistogramGuard<'t, T>
where
    T: io::Write + 't,
{
    pub fn new(w: &'t mut T, name: &'static str, help: &'static str) -> Self {
        Self {
            w,
            name,
            header: Some(help),
        }
    }

    pub fn bucket<L, V>(&mut self, le: L, value: V) -> io::Result<&mut Self>
    where
        L: MetricValue,
        V: MetricValue,
    {
        self.ensure_header()?;

        self.w.write_fmt(format_args!("unbound_{}_bucket{{le=\"", self.name))?;
        le.write(&mut self.w)?;
        self.w.write_all(b"\"} ")?;
        value.write(&mut self.w)?;
        self.w.write_all(b"\n")?;

        Ok(self)
    }

    pub fn sum<V>(&mut self, value: V) -> io::Result<&mut Self>
    where
        V: MetricValue,
    {
        self.ensure_header()?;

        self.w.write_fmt(format_args!("unbound_{}_sum ", self.name))?;
        value.write(&mut self.w)?;
        self.w.write_all(b"\n")?;

        Ok(self)
    }

    pub fn count<V>(&mut self, value: V) -> io::Result<&mut Self>
    where
        V: MetricValue,
    {
        self.ensure_header()?;

        self.w.write_fmt(format_args!("unbound_{}_count ", self.name))?;
        value.write(&mut self.w)?;
        self.w.write_all(b"\n")?;

        Ok(self)
    }

    fn ensure_header(&mut self) -> io::Result<()> {
        match self.header.take() {
            Some(help) => self.w.write_fmt(format_args!(
                "# TYPE unbound_{name} histogram\n# HELP unbound_{name} {help}\n",
                name = self.name,
                help = help
            )),
            None => Ok(()),
        }
    }
}
