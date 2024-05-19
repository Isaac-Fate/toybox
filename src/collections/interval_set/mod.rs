mod error;
pub use error::{ IntervalSetResult, IntervalSetError };

mod endpoint;
pub use endpoint::Endpoint;

mod interval;
use interval::Interval;

use std::ops::{ BitAnd, BitOr };

pub struct IntervalSet<T: PartialOrd + Clone> {
    intervals: Vec<Interval<T>>,
}

impl<T: PartialOrd + Clone> IntervalSet<T> {
    /// Creates a new interval set with an open interval.
    pub fn open(left_value: T, right_value: T) -> IntervalSetResult<Self> {
        let interval = Interval::open(left_value, right_value);
        match interval {
            Ok(interval) => Ok(Self::from(interval)),
            Err(error) => Err(error),
        }
    }

    /// Creates a new interval set with a closed interval.
    pub fn closed(left_value: T, right_value: T) -> IntervalSetResult<Self> {
        let interval = Interval::closed(left_value, right_value);
        match interval {
            Ok(interval) => Ok(Self::from(interval)),
            Err(error) => Err(error),
        }
    }

    /// Creates a new interval set with a left-open right-closed interval.
    pub fn open_closed(left_value: T, right_value: T) -> IntervalSetResult<Self> {
        let interval = Interval::open_closed(left_value, right_value);
        match interval {
            Ok(interval) => Ok(Self::from(interval)),
            Err(error) => Err(error),
        }
    }

    /// Creates a new interval set with a left-closed right-open interval.
    pub fn closed_open(left_value: T, right_value: T) -> IntervalSetResult<Self> {
        let interval = Interval::closed_open(left_value, right_value);
        match interval {
            Ok(interval) => Ok(Self::from(interval)),
            Err(error) => Err(error),
        }
    }

    /// Creates a new interval set with an interval that is unbounded on the left and open on the right.
    pub fn unbounded_open(right_value: T) -> Self {
        Self {
            intervals: vec![Interval::unbounded_open(right_value)],
        }
    }

    /// Creates a new interval set with an interval that is unbounded on the left and closed on the right.
    pub fn unbounded_closed(right_value: T) -> Self {
        Self {
            intervals: vec![Interval::unbounded_closed(right_value)],
        }
    }

    /// Creates a new interval set with an interval that is open on the left and unbounded on the right.
    pub fn open_unbounded(left_value: T) -> Self {
        Self {
            intervals: vec![Interval::open_unbounded(left_value)],
        }
    }

    /// Creates a new interval set with an interval that is closed on the left and unbounded on the right.
    pub fn closed_unbounded(left_value: T) -> Self {
        Self {
            intervals: vec![Interval::closed_unbounded(left_value)],
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        todo!()
    }

    pub fn intersection(&self, other: &Self) -> Self {
        todo!()
    }
}

impl<T: PartialOrd + Clone> From<Interval<T>> for IntervalSet<T> {
    fn from(interval: Interval<T>) -> Self {
        Self { intervals: vec![interval] }
    }
}

impl<T: PartialOrd + Clone> BitAnd for IntervalSet<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
