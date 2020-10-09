use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "emt-run")]
pub struct RunOptions {
    /// Skip tests that require human interaction
    #[structopt(short, long)]
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
