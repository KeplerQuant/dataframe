use super::{row::Row, series::Series};
use yata::prelude::OHLCV;

/// Represents a DataFrame which is used to hold market data.
#[derive(Debug)]
pub struct DataFrame {
    close: Series<f64>,
    open: Series<f64>,
    high: Series<f64>,
    low: Series<f64>,
    volume: Series<f64>,
    start_time: Series<i64>,
    end_time: Series<i64>,
    rows: Vec<Row>,
}

impl DataFrame {
    /// Creates a DataFrame from a single Row.
    pub fn from_row(row: Row) -> Self {
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

    /// Returns a reference to the rows of the DataFrame.
    pub fn rows(&self) -> &Vec<Row> {
        self.rows.as_ref()
    }

    /// Returns the height of the DataFrame, which is the length of the time series.
    pub fn height(&self) -> usize {
        self.start_time.length()
    }

    /// Returns the length of the DataFrame, which is the number of rows.
    pub fn length(&self) -> usize {
        self.rows.len()
    }

    /// Slices the DataFrame in-place from `offset` to `end`.
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

    /// Replaces the ith row of the DataFrame with the provided row.
    pub fn replace(&mut self, i: usize, row: Row) -> &mut Self {
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

    /// Extends the DataFrame with a new row at the end.
    pub fn extend(&mut self, row: Row) -> &mut Self {
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

    /// Returns a reference to the closing prices of the DataFrame.
    pub fn close(&self) -> &Series<f64> {
        &self.close
    }

    /// Returns a reference to the opening prices of the DataFrame.
    pub fn open(&self) -> &Series<f64> {
        &self.open
    }

    /// Returns a reference to the highest prices of the DataFrame.
    pub fn high(&self) -> &Series<f64> {
        &self.high
    }

    /// Returns a reference to the lowest prices of the DataFrame.
    pub fn low(&self) -> &Series<f64> {
        &self.low
    }

    /// Returns a reference to the volumes of the DataFrame.
    pub fn volume(&self) -> &Series<f64> {
        &self.volume
    }

    /// Returns a reference to the start times of the DataFrame.
    pub fn start_time(&self) -> &Series<i64> {
        &self.start_time
    }

    /// Returns a reference to the end times of the DataFrame.
    pub fn end_time(&self) -> &Series<i64> {
        &self.end_time
    }
}
