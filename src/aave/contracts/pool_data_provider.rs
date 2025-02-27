use crate::prelude::*;
use alloy::{primitives::Address, providers::Provider, sol};
use std::fmt;

sol!(
    #[sol(rpc)]
    "interfaces/IPoolDataProvider.sol"
);

pub struct PoolDataProvider<P> {
    provider: P,
    pub address: Address,
    pub contract: IPoolDataProvider::IPoolDataProviderInstance<(), P>,
}

impl<P> PoolDataProvider<P>
where
    P: Provider + Clone,
{
    pub async fn new(provider: P, address: Address) -> Result<Self> {
        let contract = IPoolDataProvider::new(address.clone(), provider.clone());
        // println!("tokens: {:?}", tokens);

        Ok(Self {
            provider,
            address,
            contract,
        })
    }
}

impl<P> fmt::Display for PoolDataProvider<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PoolDataProvider: {}", self.address)
    }
}
