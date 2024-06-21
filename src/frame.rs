use crate::{kline::Kline, ticker::Ticker, KlineRow, TickerRow};

/// Represents a DataFrame which is used to hold market data.
#[derive(Debug, Clone)]
pub struct DataFrame {
    pub kline: Option<Kline>,
    pub ticker: Option<Ticker>,
}

impl DataFrame {
    pub fn from_kline_row(row: KlineRow) -> Self {
        Self {
            kline: Some(Kline::from_row(row)),
            ticker: None,
        }
    }

    pub fn from_ticker_row(row: TickerRow) -> Self {
        Self {
            kline: None,
            ticker: Some(Ticker::from_row(row)),
        }
    }
}
