use crate::prelude::*;
use alloy::{primitives::Address, providers::Provider, sol};
use std::fmt;

sol!(
    #[sol(rpc)]
    "interfaces/IPoolAddressesProvider.sol"
);

pub struct PoolAddressProvider<P> {
    provider: P,
    pub address: Address,
    pub contract: IPoolAddressesProvider::IPoolAddressesProviderInstance<(), P>,
}

impl<P> PoolAddressProvider<P>
where
    P: Provider + Clone,
{
    pub async fn new(provider: P, address: Address) -> Result<Self> {
        let contract = IPoolAddressesProvider::new(address.clone(), provider.clone());

        Ok(Self {
            provider,
            address,
            contract,
        })
    }
}

impl<P> fmt::Display for PoolAddressProvider<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PoolAddressesProvider: {}", self.address)
    }
}
