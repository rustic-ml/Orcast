use crate::strategies::{OptionStrategy, OrderLeg};

pub struct LongStrangle {
    pub call_symbol: String,
    pub put_symbol: String,
    pub quantity: u32,
    pub call_limit_price: Option<String>,
    pub put_limit_price: Option<String>,
}

impl OptionStrategy for LongStrangle {
    fn name(&self) -> &'static str { "long_strangle" }
    fn legs(&self) -> Vec<OrderLeg> {
        vec![
            OrderLeg { symbol: self.call_symbol.clone(), qty: self.quantity, side: "buy".into(), order_type: if self.call_limit_price.is_some() { "limit".into() } else { "market".into() }, limit_price: self.call_limit_price.clone() },
            OrderLeg { symbol: self.put_symbol.clone(), qty: self.quantity, side: "buy".into(), order_type: if self.put_limit_price.is_some() { "limit".into() } else { "market".into() }, limit_price: self.put_limit_price.clone() },
        ]
    }
}


