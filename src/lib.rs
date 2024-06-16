//! This is the root module of a library for data analysis.
//!
//! This module contains several sub-modules:
//! - `dataframe`: This module contains the `DataFrame` structure and associated functions. A DataFrame represents a table of data with rows and columns.
//! - `row`: This module contains the `Row` structure and associated functions. A Row represents a single row in a DataFrame.
//! - `series`: This module contains the `Series` structure and associated functions. A Series represents a single column in a DataFrame.
//! - `ta`: This module contains technical analysis functions. These are functions that can be used to perform statistical analysis on a DataFrame.
//!
//! The main structures of `DataFrame`, `Row`, and `Series` are re-exported at the crate root for ease of access.

/// This module contains the `DataFrame` structure and associated functions.
mod frame;
/// This module contains the `Row` structure and associated functions.
mod row;
/// This module contains the `Series` structure and associated functions.
mod series;

pub use frame::DataFrame;
pub use row::Row;
pub use series::Series;
pub use yata;
