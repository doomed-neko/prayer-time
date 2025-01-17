#[derive(Debug, clap::Parser, Clone, Copy)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand, Clone, Copy)]
pub enum Command {
    /// next prayer today
    Next {
        #[arg(short, long)]
        json: bool,
    },
    /// today's prayers
    Today {
        #[arg(short, long)]
        json: bool,
    },
}
