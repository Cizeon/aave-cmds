use crate::aave::AaveV3;
use crate::display::MyDisplay;
use crate::prelude::*;
use alloy::providers::Provider;

pub struct TokenCommand<P> {
    aave_v3: AaveV3<P>,
}

impl<P> TokenCommand<P>
where
    P: Provider + Clone,
{
    pub fn new(aave_v3: AaveV3<P>) -> Self {
        Self { aave_v3 }
    }

    // Retrieves all reserves tokens.
    pub async fn list_reserves_tokens(&self) -> Result<Box<dyn MyDisplay>> {
        let tokens = self.aave_v3.get_all_reserves_tokens().await?;
        Ok(Box::new(tokens))
    }

    // Retrieves all reserves tokens.
    pub async fn list_atokens(&self) -> Result<Box<dyn MyDisplay>> {
        let tokens = self.aave_v3.get_all_atokens().await?;
        Ok(Box::new(tokens))
    }

    // Get information about given token.
    pub async fn get_reserves_token(&self, token_symbol: String) -> Result<()> {
        let tokens = self
            .aave_v3
            .pool_data_provider
            .get_all_reserves_tokens()
            .await?;

        if tokens.contains_key(&token_symbol) {
            println!("{}: {}\n", tokens[&token_symbol], token_symbol);
        }

        Ok(())
    }
}
