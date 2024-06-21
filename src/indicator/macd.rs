use anyhow::Result;
use yata::core::IndicatorResult;
use yata::indicators::MACD;
use yata::prelude::*;

use crate::frame::DataFrame;

impl DataFrame {
    pub fn macd(&self) -> Result<Vec<IndicatorResult>> {
        let macd = MACD::default();
        let kline = self.kline.as_ref().unwrap();
        let macd = macd.over(kline.rows())?;

        Ok(macd)
    }
}
