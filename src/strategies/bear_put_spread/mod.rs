use crate::strategies::{OptionStrategy, OrderLeg};

pub struct BearPutSpread {
    pub long_put: String,
    pub short_put: String,
    pub quantity: u32,
    pub long_limit_price: Option<String>,
    pub short_limit_price: Option<String>,
}

impl OptionStrategy for BearPutSpread {
    fn name(&self) -> &'static str { "bear_put_spread" }
    fn legs(&self) -> Vec<OrderLeg> {
        vec![
            OrderLeg { symbol: self.long_put.clone(), qty: self.quantity, side: "buy".into(), order_type: if self.long_limit_price.is_some() { "limit".into() } else { "market".into() }, limit_price: self.long_limit_price.clone() },
            OrderLeg { symbol: self.short_put.clone(), qty: self.quantity, side: "sell".into(), order_type: if self.short_limit_price.is_some() { "limit".into() } else { "market".into() }, limit_price: self.short_limit_price.clone() },
        ]
    }
}


