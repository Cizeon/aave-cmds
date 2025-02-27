use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(long, value_enum, default_value_t=Chains::Ether)]
    pub chain: Chains,

    #[arg(long, env = "ETH_RPC_URL")]
    pub rpc_url: Option<String>,

    #[arg(long, env = "POOL_ADDRESS_PROVIDER")]
    pub pool_address_provider: Option<String>,

    #[arg(long, env = "WALLET_ADDRESS")]
    pub wallet_address: Option<String>,

    #[arg(long, default_value_t = false)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clone, Debug, clap::ValueEnum)]
// #[group(required = false, multiple = false)]
pub enum Chains {
    Ether,
    Polygon,
    Gnosis,
    Rmm,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Token {
        #[arg(long, default_value_t = false)]
        list: bool,

        #[arg(long)]
        get: Option<String>,
    },
    Portfolio {},
}
