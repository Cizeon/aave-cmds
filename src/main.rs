#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod aave;
mod args;
mod commands;
mod contracts;
mod display;
mod error;
mod prelude;

use alloy::{
    network::{EthereumWallet, ReceiptResponse, TransactionBuilder},
    primitives::{address, utils::format_ether, Address, U256},
    providers::{Network, Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    sol,
    transports::Transport,
};
use clap::Parser;
use colored::{control::ShouldColorize, Colorize};
use commands::PortfolioCommand;
use display::MyDisplay;
use reqwest::Url;
use std::sync::Arc;
use std::{process, str::FromStr};

use crate::aave::AaveV3;
use crate::args::{ChainArgs, CmdArgs};
use crate::commands::TokenCommand;
use crate::prelude::*;

static ETHER_V3_ADDRESS_PROVIDER: &str = "0x2f39d218133AFaB8F2B819B1066c7E434Ad94E9e";
static GNOSIS_V3_ADDRESS_PROVIDER: &str = "0x36616cf17557639614c1cdDb356b1B83fc0B2132";
static POLYGON_V3_ADDRESS_PROVIDER: &str = "0xa97684ead0e402dC232d5A977953DF7ECBaB3CDb";
static RMM_ADDRESS_PROVIDER: &str = "0xdAA06Cf7adCEb69fCFDe68d896818b9938984A70";

#[tokio::main]
async fn main() -> Result<()> {
    let args = CmdArgs::parse();

    let mut rpc_url: String;
    let mut pool_address_provider_address: String;

    if args.chain.gnosis {
        rpc_url = String::from("https://rpc.ankr.com/gnosis");
        pool_address_provider_address = String::from(GNOSIS_V3_ADDRESS_PROVIDER);
    } else if args.chain.polygon {
        rpc_url = String::from("https://rpc.ankr.com/polygon");
        pool_address_provider_address = String::from(POLYGON_V3_ADDRESS_PROVIDER);
    } else if args.chain.rmm {
        rpc_url = String::from("https://rpc.ankr.com/gnosis");
        pool_address_provider_address = String::from(RMM_ADDRESS_PROVIDER);
    } else {
        // Ethereum is the default option.
        rpc_url = String::from("https://eth.merkle.io");
        pool_address_provider_address = String::from(ETHER_V3_ADDRESS_PROVIDER);
    }

    if args.rpc_url.is_some() {
        rpc_url = args.rpc_url.unwrap();
    }

    if args.pool_address_provider.is_some() {
        pool_address_provider_address = args.pool_address_provider.unwrap();
    }

    if args.verbose {
        println!("RPC_URL: {}", rpc_url);
        println!("POOL_ADDRESS_PROVIDER: {}", pool_address_provider_address);
    }

    if args.no_color {
        colored::control::set_override(false);
    }

    let rpc_url = Url::parse(rpc_url.as_str()).unwrap_or_else(|e| {
        println!("[-] Invalid RPC URL.");
        process::exit(1)
    });

    let provider = Arc::new(ProviderBuilder::new().on_http(rpc_url));

    // Test the provider.
    provider.get_block_number().await.unwrap_or_else(|e| {
        println!("[-] RPC error: {}", e);
        process::exit(1)
    });

    let pool_address_provider_address = Address::from_str(pool_address_provider_address.as_str())
        .unwrap_or_else(|e| {
            println!("[-] Invalid POOL_ADDRESS_PROVIDER.");
            process::exit(1)
        });

    // Retrieving smart contracts information.
    let aave_v3 = AaveV3::new(provider.clone(), pool_address_provider_address).await?;

    // Execute command.
    let output: Result<Box<dyn MyDisplay>> = match args.command {
        crate::args::Command::Info {} => Ok(Box::new(aave_v3)),
        crate::args::Command::Token {
            list,
            list_atokens,
            get,
        } => {
            let token_info = TokenCommand::new(aave_v3);

            let output;

            if list {
                output = token_info.list_reserves_tokens().await?;
            } else if list_atokens {
                output = token_info.list_atokens().await?;
            } else if get.is_some() {
                unimplemented!()
            } else {
                unimplemented!()
            }

            Ok(output)
        }
        crate::args::Command::Portfolio { wallet } => {
            let portfolio_command = PortfolioCommand::new(aave_v3, wallet.wallet_address);
            Ok(portfolio_command.get_porfolio(provider.clone()).await?)
        }
        _ => unimplemented!(),
    };

    // ShouldColorize::set_override(&self, false);

    // Display output.
    match args.json {
        true => {
            println!("{}", output?.to_json()?);
        }
        false => {
            println!("{}", output?.to_text()?);
        }
    };

    Ok(())
}
