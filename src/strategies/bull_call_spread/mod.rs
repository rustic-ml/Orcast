use crate::strategies::{OptionStrategy, OrderLeg};

pub struct BullCallSpread {
    pub long_call: String,
    pub short_call: String,
    pub quantity: u32,
    pub long_limit_price: Option<String>,
    pub short_limit_price: Option<String>,
}

impl OptionStrategy for BullCallSpread {
    fn name(&self) -> &'static str { "bull_call_spread" }
    fn legs(&self) -> Vec<OrderLeg> {
        vec![
            OrderLeg { symbol: self.long_call.clone(), qty: self.quantity, side: "buy".into(), order_type: if self.long_limit_price.is_some() { "limit".into() } else { "market".into() }, limit_price: self.long_limit_price.clone() },
            OrderLeg { symbol: self.short_call.clone(), qty: self.quantity, side: "sell".into(), order_type: if self.short_limit_price.is_some() { "limit".into() } else { "market".into() }, limit_price: self.short_limit_price.clone() },
        ]
    }
}


