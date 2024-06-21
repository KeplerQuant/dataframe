/// Represents a book ticker row of market data.
#[derive(Debug, Copy, Clone)]
pub struct TickerRow {
    bid_price: f64,
    bid_qty: f64,
    ask_price: f64,
    ask_qty: f64,
}

impl TickerRow {
    pub fn new(bid_price: f64, bid_qty: f64, ask_price: f64, ask_qty: f64) -> Self {
        Self {
            bid_price,
            bid_qty,
            ask_price,
            ask_qty,
        }
    }

    pub fn bid_price(&self) -> f64 {
        self.bid_price
    }

    pub fn bid_qty(&self) -> f64 {
        self.bid_qty
    }

    pub fn ask_price(&self) -> f64 {
        self.ask_price
    }

    pub fn ask_qty(&self) -> f64 {
        self.ask_qty
    }
}
