use crate::prelude::*;
use alloy::{primitives::Address, providers::Provider, sol};
use std::fmt;

sol!(
    #[sol(rpc)]
    "interfaces/IPoolAddressesProvider.sol"
);

pub struct PoolAddressProvider<P> {
    pub address: Address,
    contract: IPoolAddressesProvider::IPoolAddressesProviderInstance<(), P>,
}

impl<P> PoolAddressProvider<P>
where
    P: Provider + Clone,
{
    pub async fn new(provider: P, address: Address) -> Result<Self> {
        let contract = IPoolAddressesProvider::new(address.clone(), provider.clone());

        Ok(Self { address, contract })
    }

    pub async fn get_pool_data_provider_address(&self) -> Result<Address> {
        Ok(self.contract.getPoolDataProvider().call().await?._0)
    }
}

impl<P> fmt::Display for PoolAddressProvider<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PoolAddressesProvider: {}", self.address)
    }
}
