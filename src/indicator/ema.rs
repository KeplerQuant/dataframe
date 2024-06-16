use anyhow::Result;
use yata::methods::EMA;
use yata::prelude::*;

use crate::frame::DataFrame;
use crate::series::Series;

impl DataFrame {
    pub fn ema(&self, inputs: &Series<f64>, span: u8) -> Result<Series<f64>> {
        let ema = EMA::new_over(span, inputs)?;
        Ok(Series::new(ema))
    }
}
