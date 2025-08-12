#[derive(Debug, Clone, Default)]
pub struct OptionFeatures {
    pub iv_rank_3m: Option<f64>,
    pub iv_rank_6m: Option<f64>,
    pub iv_rank_1y: Option<f64>,
    pub iv_to_realized_ratio: Option<f64>,
    pub term_structure_slope: Option<f64>,
    pub call_put_skew_25d: Option<f64>,
    pub min_oi_atm_window: Option<u64>,
    pub opt_volume_atm_window: Option<u64>,
    pub max_spread_bps_atm: Option<f64>,
    pub days_to_earnings: Option<u32>,
    pub historical_post_earn_move: Option<f64>,
    pub iv_crush_risk: Option<f64>,
    pub unusual_activity_score: Option<f64>,
}

pub trait OptionDataProvider {
    fn fetch_features(&self, _symbol: &str) -> OptionFeatures {
        OptionFeatures::default()
    }
}

pub struct NoopOptionDataProvider;
impl OptionDataProvider for NoopOptionDataProvider {}


