use crate::prelude::*;
use alloy::{
    primitives::{utils::format_units, Address, U256},
    providers::Provider,
    sol,
};
use std::fmt;

sol!(
    #[sol(rpc)]
    interface IERC20 {
        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);
        function totalSupply() external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address to, uint256 value) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 value) external returns (bool);
        function transferFrom(address from, address to, uint256 value) external returns (bool);
        function symbol() returns (string);
        function decimals() returns (uint8);
    }
);

pub struct ERC20<P> {
    pub address: Address,
    contract: IERC20::IERC20Instance<(), P>,
}

impl<P> ERC20<P>
where
    P: Provider + Clone,
{
    pub async fn new(provider: P, address: Address) -> Result<Self> {
        let contract = IERC20::new(address.clone(), provider.clone());
        Ok(Self { address, contract })
    }

    pub async fn symbol(&self) -> Result<String> {
        Ok(self.contract.symbol().call().await?._0)
    }

    pub async fn decimals(&self) -> Result<u8> {
        Ok(self.contract.decimals().call().await?._0)
    }

    pub async fn balance_of(&self, address: Address) -> Result<U256> {
        Ok(self.contract.balanceOf(address).call().await?._0)
    }

    pub async fn balance_of_formatted(&self, address: Address) -> Result<String> {
        let decimals = self.decimals().await?;
        let balance = self.balance_of(address).await?;
        Ok(format_units(balance, decimals)?)
    }
}
