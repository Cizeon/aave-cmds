use crate::display::MyDisplay;
use crate::prelude::*;
use alloy::{
    primitives::{address, Address, U256},
    providers::Provider,
};
use colored::Colorize;
use serde::Serialize;
use std::{fmt::Write, str::FromStr};

#[derive(Clone, Serialize)]
pub struct Asset {
    pub symbol: String,
    pub address: Address,
    pub balance: String,
}

#[derive(Clone, Serialize)]
pub struct GetPortfolioOutput {
    pub assets: Vec<Asset>,
}

impl MyDisplay for GetPortfolioOutput {
    fn to_json(&self) -> Result<String> {
        // Export to JSON string.
        Ok(serde_json::to_string(&self.assets)?)
    }

    fn to_text(&self) -> Result<String> {
        let mut buf = String::new();

        for asset in &self.assets {
            buf.write_str(
                format!("{:>12}: {}\n", asset.symbol.blue(), asset.balance.green()).as_str(),
            )?;
        }

        Ok(buf)
    }
}
