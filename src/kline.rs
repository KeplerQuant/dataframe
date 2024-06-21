use crate::{KlineRow, Series};
use yata::prelude::OHLCV;

/// Represents a Kline which is used to hold market data.
#[derive(Debug, Clone)]
pub struct Kline {
    close: Series<f64>,
    open: Series<f64>,
    high: Series<f64>,
    low: Series<f64>,
    volume: Series<f64>,
    start_time: Series<i64>,
    end_time: Series<i64>,
    rows: Vec<KlineRow>,
}

impl Kline {
    /// Creates a Kline from a single Row.
    pub fn from_row(row: KlineRow) -> Self {
        Self {
            close: Series::new(vec![row.close()]),
            open: Series::new(vec![row.open()]),
            high: Series::new(vec![row.high()]),
            low: Series::new(vec![row.low()]),
            volume: Series::new(vec![row.volume()]),
            start_time: Series::new(vec![row.start_time()]),
            end_time: Series::new(vec![row.end_time()]),
            rows: vec![row],
        }
    }

    /// Returns a reference to the rows of the Kline.
    pub fn rows(&self) -> &Vec<KlineRow> {
        self.rows.as_ref()
    }

    /// Returns the height of the Kline, which is the length of the time series.
    pub fn height(&self) -> usize {
        self.start_time.length()
    }

    /// Returns the length of the Kline, which is the number of rows.
    pub fn length(&self) -> usize {
        self.rows.len()
    }

    /// Slices the Kline in-place from `offset` to `end`.
    pub fn slice(&mut self, offset: usize, end: usize) -> &mut Self {
        self.close.slice(offset, end);
        self.open.slice(offset, end);
        self.high.slice(offset, end);
        self.low.slice(offset, end);
        self.volume.slice(offset, end);
        self.start_time.slice(offset, end);
        self.end_time.slice(offset, end);
        self.rows = self.rows[offset..end].to_vec();

        self
    }

    /// Replaces the ith row of the Kline with the provided row.
    pub fn replace(&mut self, i: usize, row: KlineRow) -> &mut Self {
        self.close.replace(i, row.close());
        self.open.replace(i, row.open());
        self.high.replace(i, row.high());
        self.low.replace(i, row.low());
        self.volume.replace(i, row.volume());
        self.start_time.replace(i, row.start_time());
        self.end_time.replace(i, row.end_time());
        self.rows[i] = row;

        self
    }

    /// Extends the Kline with a new row at the end.
    pub fn extend(&mut self, row: KlineRow) -> &mut Self {
        self.close.extend(row.close());
        self.open.extend(row.open());
        self.high.extend(row.high());
        self.low.extend(row.low());
        self.volume.extend(row.volume());
        self.start_time.extend(row.start_time());
        self.end_time.extend(row.end_time());
        self.rows.push(row);

        self
    }

    /// Returns a reference to the closing prices of the Kline.
    pub fn close(&self) -> &Series<f64> {
        &self.close
    }

    /// Returns a reference to the opening prices of the Kline.
    pub fn open(&self) -> &Series<f64> {
        &self.open
    }

    /// Returns a reference to the highest prices of the Kline.
    pub fn high(&self) -> &Series<f64> {
        &self.high
    }

    /// Returns a reference to the lowest prices of the Kline.
    pub fn low(&self) -> &Series<f64> {
        &self.low
    }

    /// Returns a reference to the volumes of the Kline.
    pub fn volume(&self) -> &Series<f64> {
        &self.volume
    }

    /// Returns a reference to the start times of the Kline.
    pub fn start_time(&self) -> &Series<i64> {
        &self.start_time
    }

    /// Returns a reference to the end times of the Kline.
    pub fn end_time(&self) -> &Series<i64> {
        &self.end_time
    }
}
