mod contracts;
mod tokens;

use crate::display::MyDisplay;
use crate::prelude::*;
use alloy::primitives::Address;
use alloy::{
    network::{EthereumWallet, ReceiptResponse, TransactionBuilder},
    primitives::{address, utils::format_ether, U256},
    providers::{Network, Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    sol,
    transports::Transport,
};
use contracts::pool_address_provider::PoolAddressProvider;
use contracts::pool_data_provider::PoolDataProvider;
use std::fmt::{self, Write};
use tokens::Tokens;

pub struct AaveV3<P> {
    pub pool_address_provider: PoolAddressProvider<P>,
    pub pool_data_provider: PoolDataProvider<P>,
}

impl<P> AaveV3<P>
where
    P: Provider + Clone,
{
    pub async fn new(provider: P, address: Address) -> Result<Self> {
        let pool_address_provider = PoolAddressProvider::new(provider.clone(), address).await?;

        let pool_data_provider_address = pool_address_provider
            .get_pool_data_provider_address()
            .await?;

        let pool_data_provider =
            PoolDataProvider::new(provider.clone(), pool_data_provider_address).await?;

        Ok(Self {
            pool_address_provider,
            pool_data_provider,
        })
    }

    pub async fn get_all_reserves_tokens(&self) -> Result<Tokens> {
        let mut tokens = Tokens::new();

        for (symbol, address) in self.pool_data_provider.get_all_reserves_tokens().await? {
            tokens.insert(symbol, address);
        }

        Ok(tokens)
    }

    pub async fn get_all_atokens(&self) -> Result<Tokens> {
        let mut tokens = Tokens::new();

        for (symbol, address) in self.pool_data_provider.get_all_atokens().await? {
            tokens.insert(symbol, address);
        }

        Ok(tokens)
    }
}

impl<P> MyDisplay for AaveV3<P> {
    fn to_json(&self) -> Result<String> {
        Ok(String::from("not implemented"))
    }

    fn to_text(&self) -> Result<String> {
        let mut buf = String::new();

        buf.write_str(
            format!(
                "{}: PoolAddressProvider\n",
                self.pool_address_provider.address
            )
            .as_str(),
        )?;

        buf.write_str(format!("{}: PoolDataProvider", self.pool_data_provider.address).as_str())?;

        Ok(buf)
    }
}
