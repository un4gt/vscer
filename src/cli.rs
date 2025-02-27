use clap::{ArgGroup, Args, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct VscerCli {
    /// extension id, eg: `charliermarsh.ruff`
    #[arg(long, short = 'i')]
    pub ext_id: String,
    #[command(flatten)]
    pub mode: Opt,
}

#[derive(Args, Debug)]
#[command(group = ArgGroup::new("mode").required(false).multiple(false))]
pub struct Opt {
    /// number of latest versions to display, default is 10
    #[arg(long, short = 'n', group = "mode")]
    pub n_latest: Option<u16>,
    /// specified version to download
    #[arg(long, short = 's', group = "mode")]
    pub spec_version: Option<String>,
}
