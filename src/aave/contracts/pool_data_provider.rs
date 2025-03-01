use crate::prelude::*;
use alloy::{primitives::Address, providers::Provider, sol};
use std::collections::HashMap;
use std::fmt;
use IPoolDataProvider::TokenData;
// use IPoolDataProvider::TokenData;

sol!(
    #[sol(rpc)]
    "interfaces/IPoolDataProvider.sol"
);

pub struct PoolDataProvider<P> {
    pub address: Address,
    contract: IPoolDataProvider::IPoolDataProviderInstance<(), P>,
}

impl<P> PoolDataProvider<P>
where
    P: Provider + Clone,
{
    pub async fn new(provider: P, address: Address) -> Result<Self> {
        let contract = IPoolDataProvider::new(address.clone(), provider.clone());

        Ok(Self { address, contract })
    }

    pub async fn get_all_atokens(&self) -> Result<HashMap<String, Address>> {
        let mut symbols: HashMap<String, Address> = HashMap::new();

        let tokens = self.contract.getAllATokens().call().await?._0;
        for token in tokens {
            symbols.insert(token.symbol, token.tokenAddress);
        }

        Ok(symbols)
    }

    pub async fn get_all_reserves_tokens(&self) -> Result<HashMap<String, Address>> {
        let mut symbols: HashMap<String, Address> = HashMap::new();

        let tokens = self.contract.getAllReservesTokens().call().await?._0;
        for token in tokens {
            symbols.insert(token.symbol, token.tokenAddress);
        }

        Ok(symbols)
    }
}

impl<P> fmt::Display for PoolDataProvider<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PoolDataProvider: {}", self.address)
    }
}
