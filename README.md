# Prometheus DHCP Exporter

## Intro
A Rust Prometheus exporter for `dhcpd_pools`. This tool exports the infmation provided by the [dhcpd-pools](https://sourceforge.net/projects/dhcpd-pools/) utility in a format that [Prometheus](https://prometheus.io/) can understand. It's a Rust-only clone of this exporter: [https://github.com/atonkyra/dhcp-stats-prometheus](https://github.com/atonkyra/dhcp-stats-prometheus). 
There is really no need to rewrite this is Rust but I was looking for an excuse to write some Rust. The advantage is this tool does not have a dependency on Python and of course being Rust the memory and CPU footprint is minimal (which is always a good thing).

## Prerequisites 

* You need [Rust](https://www.rust-lang.org/) to compile this code. Simply follow the instructions on Rust's website to install the toolchain.
* You need the [dhcpd-pools](https://sourceforge.net/projects/dhcpd-pools/) tool. You can compile it yourself or get using your distro package manager. The tool must be in the `PATH` environment variable.

## Compilation

To compile the latest master version:

```bash
git clone https://github.com/MindFlavor/prometheus_dhcp_exporter.git
cd prometheus_dhcp_exporter
cargo install --path .
```

If you want the latest release you can simply use:

```bash
cargo install prometheus_dhcp_exporter
```

## Usage

Start the binary with `-h` to get the complete syntax. There are just two parameters though:

1. `-v` Verbose mode
2. `-p <port>` Specify an alternative port (the default is 9979). 

Once started, the tool will listen on the specified port (or the default one, 9979, if not specified) and return a Prometheus valid response at the url `/metrics`. So to check if the tool is working properly simply browse the `http://localhost:9979` (or whichever port you choose).

Now add the exporter to the Prometheus exporters as usual. I recommend to start it as a service. 
