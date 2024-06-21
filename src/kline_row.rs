use yata::prelude::OHLCV;

/// Represents a kline row of market data.
#[derive(Debug, Copy, Clone)]
pub struct KlineRow {
    close: f64,      // Closing price
    open: f64,       // Opening price
    high: f64,       // Highest price in the period
    low: f64,        // Lowest price in the period
    volume: f64,     // Trading volume
    start_time: i64, // Start time of the period
    end_time: i64,   // End time of the period
}

impl KlineRow {
    /// Creates a new `Row` with the given parameters.
    ///
    /// # Parameters
    ///
    /// - `close`: The closing price.
    /// - `open`: The opening price.
    /// - `high`: The highest price in the period.
    /// - `low`: The lowest price in the period.
    /// - `volume`: The trading volume.
    /// - `start_time`: The start time of the period.
    /// - `end_time`: The end time of the period.
    pub fn new(
        close: f64,
        open: f64,
        high: f64,
        low: f64,
        volume: f64,
        start_time: i64,
        end_time: i64,
    ) -> Self {
        Self {
            close,
            open,
            high,
            low,
            volume,
            start_time,
            end_time,
        }
    }

    /// Returns the start time of the period.
    pub fn start_time(&self) -> i64 {
        self.start_time
    }

    /// Returns the end time of the period.
    pub fn end_time(&self) -> i64 {
        self.end_time
    }
}

/// `Row` implements `OHLCV` to provide open, high, low, close, and volume values.
impl OHLCV for KlineRow {
    fn open(&self) -> f64 {
        self.open
    }

    fn high(&self) -> f64 {
        self.high
    }

    fn low(&self) -> f64 {
        self.low
    }

    fn close(&self) -> f64 {
        self.close
    }

    fn volume(&self) -> f64 {
        self.volume
    }
}
