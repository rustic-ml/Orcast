use crate::strategies::{OptionStrategy, OrderLeg};

pub struct LongCall {
    pub contract_symbol: String,
    pub quantity: u32,
    pub limit_price: Option<String>,
}

impl OptionStrategy for LongCall {
    fn name(&self) -> &'static str { "long_call" }
    fn legs(&self) -> Vec<OrderLeg> {
        vec![OrderLeg { symbol: self.contract_symbol.clone(), qty: self.quantity, side: "buy".into(), order_type: if self.limit_price.is_some() { "limit".into() } else { "market".into() }, limit_price: self.limit_price.clone() }]
    }
}


