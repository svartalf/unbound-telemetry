# unbound-telemetry

![Logo](.github/logo.png)

[![Coverage Status](https://github.com/svartalf/unbound-telemetry/workflows/Continuous%20integration/badge.svg)](https://github.com/svartalf/prometheus-unbound-exporter/actions?workflow=Continuous+integration)
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.39+-green.svg)

> [Unbound DNS resolver](https://www.nlnetlabs.nl/projects/unbound/about/) metrics exporter for [Prometheus](https://prometheus.io)

## Features

 * Communicates with `unbound` via TLS, UDS socket or shared memory
 * Compatible with [kumina/unbound_exporter](https://github.com/kumina/unbound_exporter); your dashboard should just work
 * Small binary size (~2 Mb after `strip`) and memory footprint (~10 Kb)
 * Takes ~10 ms to respond with gathered metrics
 * Blazing fast!

## Platform support

This project is developed, manually and automatically tested with Linux.

Following platforms are tested in the CI environment and expected to work:

 * Windows
 * macOS

It is expected that FreeBSD, NetBSD and OpenBSD will work too, but
there are no any manual or automatic checks for them exist.

Note that communication via UDS socket or shared memory is not supported for Windows.

## Installation

### From sources

1. [Rust](https://www.rust-lang.org/) language compiler version >= 1.39 is required
2. Clone the repository
3. Run the following command
    ```bash
    $ cargo build --release
    ```
4. Get the compiled executable from the `./target/release/unbound-telemetry`

## Usage

HTTP interface is available at http://0.0.0.0:9167 by default and can be changed via CLI arguments.

### TLS socket

First of all, enable `remote-control` option in the [`unbound.conf`](https://nlnetlabs.nl/documentation/unbound/unbound.conf/),
configure control interface address and TLS if needed.

Run the following command to see possible flags and options:

```bash
$ unbound-telemetry tls --help
```

### Unix domain socket

Similar to [TLS socket](#tls-socket), you need to enable `remote-control` option first.

Run the following command to see possible flags and options:

```bash
$ unbound-telemetry uds --help
```

### Shared memory

Enable `shm-enable` option in the [`unbound.conf`](https://nlnetlabs.nl/documentation/unbound/unbound.conf/)
and run the following command:

```bash
$ unbound-telemetry shm --help
```

### Monitoring

`/healthcheck` URL can be used for automated monitoring;
in case when exporter is not able to access the `unbound` instance,
`HTTP 500` error will be returned, response body will contain plain text error description.

## Grafana

[This Grafana dashboard](https://grafana.com/grafana/dashboards/11705) can be used
to show all metrics provided by this exporter.

## License

`unbound-telemetry` is released under the MIT License.

## Donations

If you appreciate my work and want to support me or this project, you can do it [here](https://svartalf.info/donate).
