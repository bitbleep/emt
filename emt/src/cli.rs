use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "emt")]
pub enum CliOptions {
    Run(RunOptions),
    Host(HostOptions),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "emt-run")]
pub struct RunOptions {
    #[structopt(short = "l", long = "link", default_value = "probe")]
    pub link: String,

    #[structopt(short = "d", long = "domain", default_value = "localhost")]
    pub domain: String,

    #[structopt(short = "p", long = "port", default_value = "8080")]
    pub port: u16,

    /// Skip tests that require human interaction
    #[structopt(short = "n", long)]
    pub no_human_interaction: bool,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "emt-host")]
pub struct HostOptions {
    #[structopt(short = "d", long = "domain", default_value = "localhost")]
    pub domain: String,

    #[structopt(short = "p", long = "port", default_value = "8080")]
    pub port: u16,
}
