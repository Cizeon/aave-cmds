use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CmdArgs {
    #[command(flatten)]
    pub chain: ChainArgs,

    #[arg(long, env = "ETH_RPC_URL")]
    pub rpc_url: Option<String>,

    #[arg(long, env = "POOL_ADDRESS_PROVIDER")]
    pub pool_address_provider: Option<String>,

    #[arg(long, default_value_t = false)]
    pub json: bool,

    #[arg(long, default_value_t = false)]
    pub no_color: bool,

    #[arg(long, default_value_t = false)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Args)]
#[group(multiple = false)]
pub struct ChainArgs {
    #[arg(long)]
    pub ether: bool,

    #[arg(long)]
    pub polygon: bool,

    #[arg(long)]
    pub gnosis: bool,

    #[arg(long)]
    pub rmm: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Info {},
    Token {
        #[arg(long, default_value_t = false)]
        list: bool,

        #[arg(long, default_value_t = false)]
        list_atokens: bool,

        #[arg(long)]
        get: Option<String>,
    },
    Portfolio {
        #[command(flatten)]
        wallet: WalletArgs,
    },
    Stake {},
}

#[derive(Debug, Clone, Args)]
#[group(multiple = false)]
pub struct WalletArgs {
    #[arg(long, env = "WALLET_ADDRESS")]
    pub wallet_address: String,
}
