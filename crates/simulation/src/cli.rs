#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Simulation resolution
    #[arg(long)]
    pub resolution: Option<crate::resolution::Resolution>,

    /// Number of agents to initialize buffers for
    #[arg(long)]
    pub agents: Option<u32>,

    /// Websocket address
    #[arg(long, default_value = "127.0.0.1:32167")]
    pub ws_addr: String,

    /// Start with presentation defaults
    #[arg(long)]
    pub presentation: bool,
}
