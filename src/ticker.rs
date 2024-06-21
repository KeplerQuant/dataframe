use crate::{ticker_row::TickerRow, Series};

#[derive(Debug, Clone)]
pub struct Ticker {
    bid_price: Series<f64>,
    bid_qty: Series<f64>,
    ask_price: Series<f64>,
    ask_qty: Series<f64>,
    rows: Vec<TickerRow>,
}

impl Ticker {
    /// Creates a Kline from a single Row.
    pub fn from_row(row: TickerRow) -> Self {
        Self {
            bid_price: Series::new(vec![row.bid_price()]),
            bid_qty: Series::new(vec![row.bid_qty()]),
            ask_price: Series::new(vec![row.ask_price()]),
            ask_qty: Series::new(vec![row.ask_qty()]),
            rows: vec![row],
        }
    }

    /// Returns a reference to the rows of the Kline.
    pub fn rows(&self) -> &Vec<TickerRow> {
        self.rows.as_ref()
    }

    /// Returns the height of the Kline, which is the length of the time series.
    pub fn height(&self) -> usize {
        self.bid_price.length()
    }

    /// Returns the length of the Ticker, which is the number of rows.
    pub fn length(&self) -> usize {
        self.rows.len()
    }

    /// Slices the Kline in-place from `offset` to `end`.
    pub fn slice(&mut self, offset: usize, end: usize) -> &mut Self {
        self.bid_price.slice(offset, end);
        self.bid_qty.slice(offset, end);
        self.ask_price.slice(offset, end);
        self.ask_qty.slice(offset, end);
        self.rows = self.rows[offset..end].to_vec();

        self
    }

    /// Replaces the ith row of the Kline with the provided row.
    pub fn replace(&mut self, i: usize, row: TickerRow) -> &mut Self {
        self.bid_price.replace(i, row.bid_price());
        self.bid_qty.replace(i, row.bid_qty());
        self.ask_price.replace(i, row.ask_price());
        self.ask_qty.replace(i, row.ask_qty());
        self.rows[i] = row;

        self
    }

    /// Extends the Kline with a new row at the end.
    pub fn extend(&mut self, row: TickerRow) -> &mut Self {
        self.bid_price.extend(row.bid_price());
        self.bid_qty.extend(row.bid_qty());
        self.ask_price.extend(row.ask_price());
        self.ask_qty.extend(row.ask_qty());
        self.rows.push(row);

        self
    }
}
