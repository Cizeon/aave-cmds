mod contracts;
mod reserves_tokens;

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
use reserves_tokens::ReservesTokens;
use std::fmt;

pub struct AaveV3<P> {
    pub pool_address_provider: PoolAddressProvider<P>,
    pub pool_data_provider: PoolDataProvider<P>,
}

impl<P> AaveV3<P>
where
    P: Provider + Clone,
{
    pub async fn new(provider: P, address: Address) -> Result<Self> {
        // let pool_address_provider: Address = ;
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

    pub async fn get_reserve_tokens(&self) -> Result<ReservesTokens> {
        let mut reserves_tokens = ReservesTokens::new();

        let tokens = self.pool_data_provider.get_all_tokens().await?;

        for (symbol, address) in tokens {
            reserves_tokens.insert(symbol, address);
        }

        Ok(reserves_tokens)
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
