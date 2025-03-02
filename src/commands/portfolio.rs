mod get_portfolio;

use crate::aave::AaveV3;
use crate::contracts::ERC20;
use crate::display::MyDisplay;
use crate::prelude::*;
use alloy::{
    primitives::{address, Address, U256},
    providers::Provider,
};
use get_portfolio::{Asset, GetPortfolioOutput};
use serde_json::to_string;
use std::{fmt::Write, str::FromStr};

pub struct PortfolioCommand<P> {
    aave_v3: AaveV3<P>,
    wallet_address: Address,
}

impl<P> PortfolioCommand<P>
where
    P: Provider + Clone,
{
    pub fn new(aave_v3: AaveV3<P>, wallet_address: String) -> Self {
        let wallet_address = Address::from_str(wallet_address.as_str()).unwrap();

        Self {
            aave_v3,
            wallet_address,
        }
    }

    pub async fn get_porfolio(&self, provider: P) -> Result<Box<dyn MyDisplay>> {
        let tokens = self.aave_v3.get_all_atokens().await?;

        let mut assets = Vec::new();
        for (address, _) in tokens.by_address {
            let erc20 = ERC20::new(provider.clone(), address).await?;

            let balance = erc20.balance_of(self.wallet_address).await?;

            // Only retrieve owned assets.
            if !balance.is_zero() {
                assets.push(Asset {
                    symbol: erc20.symbol().await?,
                    address: address.clone(),
                    balance: erc20.balance_of_formatted(self.wallet_address).await?,
                });
            }
        }

        let output = Box::new(GetPortfolioOutput { assets });

        Ok(output)
    }
}
