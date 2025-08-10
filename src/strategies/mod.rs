#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderLeg {
    pub symbol: String,
    pub qty: u32,
    pub side: String,       // "buy" | "sell"
    pub order_type: String, // e.g., "market" | "limit"
    pub limit_price: Option<String>,
}

pub trait OptionStrategy {
    fn name(&self) -> &'static str;
    fn legs(&self) -> Vec<OrderLeg>;
}

pub mod long_call;
pub mod long_put;
pub mod covered_call;
pub mod protective_put;
pub mod bull_call_spread;
pub mod bear_put_spread;
pub mod long_straddle;
pub mod long_strangle;


