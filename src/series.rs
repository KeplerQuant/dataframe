/// A series of elements.
///
/// This struct is generic over an arbitrary type T.
#[derive(Default, Debug, Clone)]
pub struct Series<T>(Vec<T>);

impl<T: Copy + PartialOrd> Series<T> {
    /// Constructs a new, empty `Series<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// let series: Series<i32> = Series::new(vec![1, 2, 3]);
    /// ```
    pub fn new(data: Vec<T>) -> Self {
        Self(data)
    }

    /// Returns the value at a specific position in the series.
    pub fn value(&self, index: usize) -> T {
        self.0[index]
    }

    /// Returns all values in the series.
    pub fn values(&self) -> Vec<T> {
        self.0.to_vec()
    }

    /// Returns the length of the series.
    pub fn length(&self) -> usize {
        self.0.len()
    }

    /// Replaces the value at a specific position in the series.
    pub fn replace(&mut self, index: usize, value: T) -> &mut Self {
        self.0[index] = value;
        self
    }

    /// Slices the series from the offset to the length.
    pub fn slice(&mut self, offset: usize, end: usize) -> &mut Self {
        self.0 = self.0[offset..end].to_vec();
        self
    }

    /// Extends the series with a new value.
    pub fn extend(&mut self, v: T) -> &mut Self {
        self.0.push(v);
        self
    }

    /// Returns the value at a specific position from the end of the series.
    pub fn last(&self, position: usize) -> T {
        self.0[self.0.len() - 1 - position]
    }

    /// Returns the last n values in the series.
    pub fn last_values(&self, size: usize) -> Vec<T> {
        let len = self.0.len();
        if len > size {
            return self.0[len - size..].to_vec();
        }

        self.0.to_vec()
    }

    /// Checks if the series crosses over another series.
    pub fn cross_over(&self, other: &Series<T>) -> bool {
        self.last(0) > other.last(0) && self.last(1) <= other.last(1)
    }

    /// Checks if the series crosses under another series.
    pub fn cross_under(&self, other: &Series<T>) -> bool {
        self.last(0) <= other.last(0) && self.last(1) > other.last(1)
    }

    /// Checks if the series crosses another series.
    pub fn cross(&self, other: &Series<T>) -> bool {
        self.cross_over(other) || self.cross_under(other)
    }
}

/// Implementation of the `AsRef` trait for the `Series` type, allowing a `Series<T>`
/// to be referenced as a slice `[T]`.
impl<T> AsRef<[T]> for Series<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}
