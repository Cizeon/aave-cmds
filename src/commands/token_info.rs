use crate::aave::AaveV3;
use crate::display::MyDisplay;
use crate::prelude::*;
use alloy::providers::Provider;

pub struct TokenInfo<P> {
    aave_v3: AaveV3<P>,
}

impl<P> TokenInfo<P>
where
    P: Provider + Clone,
{
    pub fn new(aave_v3: AaveV3<P>) -> Self {
        Self { aave_v3 }
    }

    // Retrieves all reserve tokens.
    pub async fn list_tokens(&self) -> Result<Box<dyn MyDisplay>> {
        let tokens = self.aave_v3.get_reserve_tokens().await?;
        Ok(Box::new(tokens))
    }

    // Get information about given token.
    pub async fn get_token(&self, token_symbol: String) -> Result<()> {
        let tokens = self.aave_v3.pool_data_provider.get_all_tokens().await?;

        if tokens.contains_key(&token_symbol) {
            println!("{}: {}\n", tokens[&token_symbol], token_symbol);
        }

        Ok(())
    }
}
