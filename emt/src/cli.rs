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

    /// Skips tests that require human interaction
    #[structopt(short = "n", long)]
    pub no_human_interaction: bool,
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
}
