use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "emt")]
pub enum CliOptions {
    /// Runs tests
    Run(RunOptions),
    /// Hosts a locally connected probe to be run remotely
    Host(HostOptions),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "emt-run")]
pub struct RunOptions {
    /// The link used to interface with a device.
    /// Use either 'probe' for a locally connected device or 'hosted' for remote connections.
    #[structopt(short = "l", long = "link", default_value = "probe")]
    pub link: String,

    /// The domain to use when connecting to a hosted link.
    #[structopt(short = "d", long = "domain", default_value = "localhost")]
    pub domain: String,

    /// The port to use when connecting to a hosted link.
    #[structopt(short = "p", long = "port", default_value = "8080")]
    pub port: u16,

    /// The id of the probe to use (ie. the index in the probe-rs list of connected probes).
    #[structopt(short = "i", long = "probe-id", default_value = "0")]
    pub probe_id: usize,

    /// The target name of the probe as known to probe-rs.
    #[structopt(short = "t", long = "probe-target", default_value = "nrf52")]
    pub probe_target: String,

    /// Skips tests that require human interaction
    #[structopt(short = "n", long)]
    pub no_human_interaction: bool,

    /// An optional ELF binary to flash onto the device
    #[structopt(long = "bin")]
    pub binary: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "emt-host")]
pub struct HostOptions {
    /// The host domain.
    #[structopt(short = "d", long = "domain", default_value = "localhost")]
    pub domain: String,

    /// The host port.
    #[structopt(short = "p", long = "port", default_value = "8080")]
    pub port: u16,

    /// The id of the probe to use (ie. the index in the probe-rs list of connected probes).
    #[structopt(short = "i", long = "probe-id", default_value = "0")]
    pub probe_id: usize,

    /// The target name of the probe as known to probe-rs.
    #[structopt(short = "t", long = "probe-target", default_value = "nrf52")]
    pub probe_target: String,
}
