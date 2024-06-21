use crate::{kline::Kline, ticker::Ticker};

/// Represents a DataFrame which is used to hold market data.
#[derive(Debug, Clone)]
pub struct DataFrame {
    pub kline: Kline,
    pub ticker: Ticker,
}
