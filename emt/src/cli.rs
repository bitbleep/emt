use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "emt-run")]
pub struct RunOptions {
    /// Skip tests that require human interaction
    #[structopt(short, long)]
    no_human_interaction: bool,
}
