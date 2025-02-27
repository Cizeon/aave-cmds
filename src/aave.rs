mod contracts;

use crate::prelude::*;
use alloy::primitives::Address;
use contracts::pool_address_provider::PoolAddressProvider;
use contracts::pool_data_provider::PoolDataProvider;
use std::fmt;

use alloy::{
    network::{EthereumWallet, ReceiptResponse, TransactionBuilder},
    primitives::{address, utils::format_ether, U256},
    providers::{Network, Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    sol,
    transports::Transport,
};

pub struct AaveV3<P> {
    provider: P,
    pub pool_address_provider: PoolAddressProvider<P>,
    pub pool_data_provider: PoolDataProvider<P>,
}

impl<P> AaveV3<P>
where
    P: Provider + Clone,
{
    pub async fn new(provider: P, address: Address) -> Result<Self> {
        // let pool_address_provider: Address = ;
        let pool_address_provider = PoolAddressProvider::new(provider.clone(), address)
            .await
            .unwrap();

        let address = pool_address_provider
            .contract
            .getPoolDataProvider()
            .call()
            .await?
            ._0;

        let pool_data_provider = PoolDataProvider::new(provider.clone(), address)
            .await
            .unwrap();

        Ok(Self {
            provider,
            pool_address_provider,
            pool_data_provider,
        })
    }

    pub async fn list_tokens(&self) -> Result<()> {
        println!("[+] List tokens");

        let tokens = self
            .pool_data_provider
            .contract
            .getAllReservesTokens()
            .call()
            .await?
            ._0;

        for token in tokens {
            println!("{} - {}", token.tokenAddress, token.symbol);
        }

        Ok(())
    }

    /* Get information about given token.  */
    pub async fn get_token(&self, token_symbol: String) -> Result<()> {
        let tokens = self
            .pool_data_provider
            .contract
            .getAllReservesTokens()
            .call()
            .await?
            ._0;

        for token in tokens {
            if token.symbol == token_symbol {
                println!("[+] {}: {}\n", token.tokenAddress, token.symbol);
            }
            // println!("{} - {}", token.tokenAddress, token.symbol);
        }

        Ok(())
    }
}

impl<P> fmt::Display for AaveV3<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.pool_address_provider, self.pool_data_provider
        )
    }
}
