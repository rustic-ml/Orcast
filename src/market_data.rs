use crate::error::{OrcastError, Result};

#[derive(Debug, Clone)]
pub struct OptionChainRequest {
    pub underlying: String,
}

pub async fn get_option_chain(_req: OptionChainRequest) -> Result<()> {
    Err(OrcastError::NotImplemented("option data REST client"))
}


