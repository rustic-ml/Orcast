use crate::strategies::{OptionStrategy, OrderLeg};

pub struct CoveredCall {
    pub option_contract_symbol: String,
    pub quantity: u32,
    pub limit_price: Option<String>,
}

impl OptionStrategy for CoveredCall {
    fn name(&self) -> &'static str { "covered_call" }
    fn legs(&self) -> Vec<OrderLeg> {
        vec![OrderLeg { symbol: self.option_contract_symbol.clone(), qty: self.quantity, side: "sell".into(), order_type: if self.limit_price.is_some() { "limit".into() } else { "market".into() }, limit_price: self.limit_price.clone() }]
    }
}


