use std::fmt::Display;
use super::{ Endpoint, IntervalSetResult, IntervalSetError };

#[derive(Debug, Clone, PartialEq)]
pub struct Interval<T: PartialOrd + Clone> {
    /// Left endpoint.
    left: Endpoint<T>,

    /// Right endpoint.
    right: Endpoint<T>,
}

impl<T: PartialOrd + Clone> Interval<T> {
    /// Creates a new interval by specifying the left and right endpoints.
    pub fn new(left: Endpoint<T>, right: Endpoint<T>) -> IntervalSetResult<Self> {
        match (&left, &right) {
            | (Endpoint::Open(low), Endpoint::Open(high))
            | (Endpoint::Open(low), Endpoint::Closed(high))
            | (Endpoint::Closed(low), Endpoint::Open(high)) => if low < high {
                Ok(Self {
                    left,
                    right,
                })
            } else {
                Err(IntervalSetError::InvalidInterval)
            }

            // Closed interval
            // The left value may be equal to the right value, and
            // in that case, the interval is degenerate
            (Endpoint::Closed(low), Endpoint::Closed(high)) => if low <= high {
                Ok(Self {
                    left,
                    right,
                })
            } else {
                Err(IntervalSetError::InvalidInterval)
            }

            // Left endpoint is unbounded
            // The interval must be valid
            // This arm will also match the case that both endpoints are unbounded
            (Endpoint::Unbounded, _) =>
                Ok(Self {
                    left,
                    right,
                }),

            // Right endpoint is unbounded
            // The interval must be valid
            (_, Endpoint::Unbounded) =>
                Ok(Self {
                    left,
                    right,
                }),
        }
    }

    /// Creates an open interval.
    pub fn open(low: T, high: T) -> IntervalSetResult<Self> {
        // Left value must be strictly less than right value
        if low < high {
            Ok(Self {
                left: Endpoint::Open(low),
                right: Endpoint::Open(high),
            })
        } else {
            Err(IntervalSetError::InvalidInterval)
        }
    }

    /// Creates a closed interval.
    /// The interval may be degenerate.
    pub fn closed(low: T, high: T) -> IntervalSetResult<Self> {
        // Left value must be less than or equal to right value
        if low <= high {
            Ok(Self {
                left: Endpoint::Closed(low),
                right: Endpoint::Closed(high),
            })
        } else {
            Err(IntervalSetError::InvalidInterval)
        }
    }

    /// Creates a left-open right-closed interval.
    pub fn open_closed(low: T, high: T) -> IntervalSetResult<Self> {
        // Left value must be less than right value
        if low < high {
            Ok(Self {
                left: Endpoint::Open(low),
                right: Endpoint::Closed(high),
            })
        } else {
            Err(IntervalSetError::InvalidInterval)
        }
    }

    /// Creates a left-closed right-open interval.
    pub fn closed_open(low: T, high: T) -> IntervalSetResult<Self> {
        // Left value must be less than right value
        if low < high {
            Ok(Self {
                left: Endpoint::Closed(low),
                right: Endpoint::Open(high),
            })
        } else {
            Err(IntervalSetError::InvalidInterval)
        }
    }

    /// Creates an interval that is unbounded on the left and open on the right.
    pub fn unbounded_open(high: T) -> Self {
        Self {
            left: Endpoint::Unbounded,
            right: Endpoint::Open(high),
        }
    }

    /// Creates an interval that is unbounded on the left and closed on the right.
    pub fn unbounded_closed(high: T) -> Self {
        Self {
            left: Endpoint::Unbounded,
            right: Endpoint::Closed(high),
        }
    }

    /// Creates an interval that is open on the left and unbounded on the right.
    pub fn open_unbounded(low: T) -> Self {
        Self {
            left: Endpoint::Open(low),
            right: Endpoint::Unbounded,
        }
    }

    /// Creates an interval that is closed on the left and unbounded on the right.
    pub fn closed_unbounded(low: T) -> Self {
        Self {
            left: Endpoint::Closed(low),
            right: Endpoint::Unbounded,
        }
    }

    /// Creates a universal interval.
    pub fn universe() -> Self {
        Self {
            left: Endpoint::Unbounded,
            right: Endpoint::Unbounded,
        }
    }

    /// Gets the left endpoint.
    pub fn left(&self) -> &Endpoint<T> {
        &self.left
    }

    /// Gets the right endpoint.
    pub fn right(&self) -> &Endpoint<T> {
        &self.right
    }

    /// Gets the low value of the interval.
    /// If the interval is unbounded on the left, returns `None`.
    pub fn low(&self) -> Option<T> {
        match &self.left {
            Endpoint::Open(low) | Endpoint::Closed(low) => Some(low.clone()),
            Endpoint::Unbounded => None,
        }
    }

    /// Gets the high value of the interval.
    /// If the interval is unbounded on the right, returns `None`.
    pub fn high(&self) -> Option<T> {
        match &self.right {
            Endpoint::Open(high) | Endpoint::Closed(high) => Some(high.clone()),
            Endpoint::Unbounded => None,
        }
    }

    /// Checks if the interval is universe, i.e., both endpoints are unbounded.
    pub fn is_universe(&self) -> bool {
        matches!((&self.left, &self.right), (Endpoint::Unbounded, Endpoint::Unbounded))
    }

    /// Checks if the interval is degenerate.
    /// An interval is degenerate if it is closed and
    /// the left endpoint is equal to the right endpoint.
    pub fn is_degenerate(&self) -> bool {
        matches!(
            (&self.left, &self.right), 
            (Endpoint::Closed(low), Endpoint::Closed(high)) if low == high
        )
    }

    /// Checks if the interval is bounded.
    pub fn is_bounded(&self) -> bool {
        matches!(
            (&self.left, &self.right),
            (Endpoint::Open(_) | Endpoint::Closed(_), Endpoint::Open(_) | Endpoint::Closed(_))
        )
    }

    /// Checks if the interval is unbounded.
    pub fn is_unbounded(&self) -> bool {
        return !self.is_bounded();
    }

    /// Checks if the interval is separated from the other interval.
    /// Two intervals A and B are separated if and only if
    /// - closure(A) and B are disjoint, and
    /// - A and closure(B) are disjoint.
    pub fn is_separated_from(&self, other: &Self) -> bool {
        self.is_other_separated_from_this_to_the_left(other) ||
            self.is_other_separated_from_this_to_the_right(other)
    }

    /// Merges two intervals.
    /// Returns an error if the two intervals are separated.
    pub fn merge(&self, other: &Self) -> IntervalSetResult<Self> {
        if self.is_separated_from(other) {
            // Return an error since they are separated, and hence cannot be merged
            Err(IntervalSetError::MergeSeparatedIntervals)
        } else {
            // Merge the two intervals into a new interval
            Ok(Self {
                left: self.smaller_left_endpoint(other),
                right: self.greater_right_endpoint(other),
            })
        }
    }

    /// Checks if the other interval is separated from this interval to the left.
    fn is_other_separated_from_this_to_the_left(&self, other: &Self) -> bool {
        match &self.left {
            Endpoint::Open(this_low) => {
                // They are separate
                // if the high value of the other interval is less than or equal to the low value of this interval
                // when the other interval is open at the right endpoint, or
                // if the high value of the other interval is strictly less than the low value of this interval
                // when the other interval is closed at the right endpoint
                matches!(&other.right, Endpoint::Open(other_high)  if this_low >= other_high) ||
                    matches!(&other.right, Endpoint::Closed(other_high) if this_low > other_high)
            }
            Endpoint::Closed(this_low) => {
                // They are separate
                // if the high value of the other interval is strictly less than the low value of this interval
                matches!(&other.right, Endpoint::Open(other_high) | Endpoint::Closed(other_high) if this_low > other_high)
            }

            Endpoint::Unbounded => false,
        }
    }

    /// Checks if the other interval is separated from this interval to the right.
    fn is_other_separated_from_this_to_the_right(&self, other: &Self) -> bool {
        match &self.right {
            Endpoint::Open(this_high) => {
                // They are separate
                // if the low value of the other interval is greater than or equal to the high value of this interval
                // when the other interval is open at the left endpoint, or
                // if the low value of the other interval is strictly greater than the high value of this interval
                // when the other interval is closed at the left endpoint
                matches!(&other.left, Endpoint::Open(other_low)  if this_high <= other_low) ||
                    matches!(&other.left, Endpoint::Closed(other_low) if this_high < other_low)
            }
            Endpoint::Closed(this_high) => {
                // They are separate
                // if the low value of the other interval is strictly greater than the high value of this interval
                matches!(&other.left, Endpoint::Open(other_low) | Endpoint::Closed(other_low) if this_high < other_low)
            }

            Endpoint::Unbounded => false,
        }
    }

    /// Gets the smaller left endpoint of the two intervals.
    fn smaller_left_endpoint(&self, other: &Self) -> Endpoint<T> {
        // If either interval is unbounded on the left, return unbounded
        if matches!(&self.left, Endpoint::Unbounded) || matches!(&other.left, Endpoint::Unbounded) {
            return Endpoint::Unbounded;
        }

        // Get the low values of the two intervals
        // It is safe to unwrap because we already checked that the intervals are bounded
        let this_low = self.low().unwrap();
        let other_low = other.low().unwrap();

        if this_low < other_low {
            self.left.clone()
        } else if this_low > other_low {
            other.left.clone()
        } else {
            // The low values are equal

            // If either interval is closed on the left, return the closed endpoint
            if
                matches!(&self.left, Endpoint::Closed(_)) ||
                matches!(&other.left, Endpoint::Closed(_))
            {
                Endpoint::Closed(this_low)
            } else {
                // Else, return the open endpoint
                Endpoint::Open(this_low)
            }
        }
    }

    /// Gets the greater left endpoint of the two intervals.
    fn greater_left_endpoint(&self, other: &Self) -> Endpoint<T> {
        // If this interval is unbounded on the left, return the left endpoint of other interval
        if matches!(&self.left, Endpoint::Unbounded) {
            return other.left.clone();
        }

        // If other interval is unbounded on the left, return the left endpoint of this interval
        if matches!(&other.left, Endpoint::Unbounded) {
            return self.left.clone();
        }

        // Get the low values of the two intervals
        // It is safe to unwrap because we already checked that the intervals are bounded
        let this_low = self.low().unwrap();
        let other_low = other.low().unwrap();

        if this_low > other_low {
            self.left.clone()
        } else if this_low < other_low {
            other.left.clone()
        } else {
            // The low values are equal

            // If either interval is open on the left, return the open endpoint
            if matches!(&self.left, Endpoint::Open(_)) || matches!(&other.left, Endpoint::Open(_)) {
                Endpoint::Open(this_low)
            } else {
                // Else, return the closed endpoint
                Endpoint::Closed(this_low)
            }
        }
    }

    /// Gets the less right endpoint of the two intervals.
    fn less_right_endpoint(&self, other: &Self) -> Endpoint<T> {
        // If this interval is unbounded on the right, return the right endpoint of other interval
        if matches!(&self.right, Endpoint::Unbounded) {
            return other.right.clone();
        }

        // If other interval is unbounded on the right, return the right endpoint of this interval
        if matches!(&other.right, Endpoint::Unbounded) {
            return self.right.clone();
        }

        // Get the high values of the two intervals
        // It is safe to unwrap because we already checked that the intervals are bounded
        let this_high = self.high().unwrap();
        let other_high = other.high().unwrap();

        if this_high < other_high {
            self.right.clone()
        } else if this_high > other_high {
            other.right.clone()
        } else {
            // The high values are equal

            // If either interval is open on the right, return the open endpoint
            if
                matches!(&self.right, Endpoint::Open(_)) ||
                matches!(&other.right, Endpoint::Open(_))
            {
                Endpoint::Open(this_high)
            } else {
                // Else, return the closed endpoint
                Endpoint::Closed(this_high)
            }
        }
    }

    /// Gets the greater right endpoint of the two intervals.
    fn greater_right_endpoint(&self, other: &Self) -> Endpoint<T> {
        // If either interval is unbounded on the right, return unbounded
        if
            matches!(&self.right, Endpoint::Unbounded) ||
            matches!(&other.right, Endpoint::Unbounded)
        {
            return Endpoint::Unbounded;
        }

        // Get the high values of the two intervals
        // It is safe to unwrap because we already checked that the intervals are bounded
        let this_high = self.high().unwrap();
        let other_high = other.high().unwrap();

        if this_high > other_high {
            self.right.clone()
        } else if this_high < other_high {
            other.right.clone()
        } else {
            // The high values are equal

            // If either interval is closed on the right, return the closed endpoint
            if
                matches!(&self.right, Endpoint::Closed(_)) ||
                matches!(&other.right, Endpoint::Closed(_))
            {
                Endpoint::Closed(this_high)
            } else {
                // Else, return the open endpoint
                Endpoint::Open(this_high)
            }
        }
    }
}

impl<T: PartialOrd + Clone + Display> Display for Interval<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.left, &self.right) {
            (Endpoint::Open(low), Endpoint::Open(high)) => { write!(f, "({}, {})", low, high) }
            (Endpoint::Open(low), Endpoint::Closed(high)) => { write!(f, "({}, {}]", low, high) }
            (Endpoint::Closed(low), Endpoint::Open(high)) => { write!(f, "[{}, {})", low, high) }
            (Endpoint::Closed(low), Endpoint::Closed(high)) => if low == high {
                write!(f, "[{}]", low)
            } else {
                write!(f, "[{}, {}]", low, high)
            }
            (Endpoint::Unbounded, Endpoint::Open(high)) => { write!(f, "(-∞, {})", high) }
            (Endpoint::Unbounded, Endpoint::Closed(high)) => { write!(f, "(-∞, {}]", high) }
            (Endpoint::Open(low), Endpoint::Unbounded) => { write!(f, "({}, +∞)", low) }
            (Endpoint::Closed(low), Endpoint::Unbounded) => { write!(f, "[{}, +∞)", low) }
            (Endpoint::Unbounded, Endpoint::Unbounded) => { write!(f, "(-∞, +∞)") }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let interval = Interval::<i32>::new(Endpoint::Open(0), Endpoint::Open(1));
        assert!(interval.is_ok());

        let interval = Interval::<i32>::new(Endpoint::Open(0), Endpoint::Open(0));
        assert!(interval.is_err());

        let interval = Interval::<i32>::new(Endpoint::Closed(0), Endpoint::Closed(0));
        assert!(interval.is_ok());

        let interval = Interval::<i32>::new(Endpoint::Open(0), Endpoint::Unbounded);
        assert!(interval.is_ok());
    }

    #[test]
    fn test_display() {
        let interval = Interval::<i32>::new(Endpoint::Open(0), Endpoint::Open(1));
        assert!(interval.is_ok());
        let interval = interval.unwrap();
        println!("{}", interval);
        assert_eq!(interval.to_string(), "(0, 1)");

        let interval = Interval::<i32>::new(Endpoint::Closed(0), Endpoint::Closed(0));
        assert!(interval.is_ok());
        let interval = interval.unwrap();
        println!("{}", interval);
        assert_eq!(interval.to_string(), "[0]");

        let interval = Interval::<i32>::new(Endpoint::Open(0), Endpoint::Unbounded);
        assert!(interval.is_ok());
        let interval = interval.unwrap();
        println!("{}", interval);
        assert_eq!(interval.to_string(), "(0, +∞)");

        let interval = Interval::<i32>::open(0, 1);
        assert!(interval.is_ok());
        let interval = interval.unwrap();
        println!("{}", interval);
        assert_eq!(interval.to_string(), "(0, 1)");

        let interval = Interval::<i32>::closed(0, 0);
        assert!(interval.is_ok());
        let interval = interval.unwrap();
        println!("{}", interval);
        assert_eq!(interval.to_string(), "[0]");

        let interval = Interval::<i32>::open_closed(0, 0);
        assert!(interval.is_err());

        let interval = Interval::<i32>::open_closed(0, 1);
        assert!(interval.is_ok());
        let interval = interval.unwrap();
        println!("{}", interval);
        assert_eq!(interval.to_string(), "(0, 1]");
    }

    #[test]
    fn test_is_degenerate() {
        let interval = Interval::<i32>::new(Endpoint::Closed(0), Endpoint::Closed(0));
        assert!(interval.is_ok());
        let interval = interval.unwrap();
        assert!(interval.is_degenerate());
    }

    #[test]
    fn test_is_separated_from() {
        let a = Interval::<i32>::open(0, 1).unwrap();
        let b = Interval::<i32>::open(1, 2).unwrap();
        assert!(a.is_separated_from(&b));

        let a = Interval::<i32>::open(0, 1).unwrap();
        let b = Interval::<i32>::closed_open(1, 2).unwrap();
        assert!(!a.is_separated_from(&b));

        let a = Interval::<i32>::closed(0, 1).unwrap();
        let b = Interval::<i32>::closed_open(1, 2).unwrap();
        assert!(!a.is_separated_from(&b));

        let a = Interval::<i32>::open(0, 2).unwrap();
        let b = Interval::<i32>::open(1, 3).unwrap();
        assert!(!a.is_separated_from(&b));

        let a = Interval::<i32>::open_unbounded(0);
        let b = Interval::<i32>::open(1, 2).unwrap();
        assert!(!a.is_separated_from(&b));

        let a = Interval::<i32>::universe();
        let b = Interval::<i32>::open_unbounded(1);
        assert!(!a.is_separated_from(&b));

        let a = Interval::<i32>::unbounded_open(0);
        let b = Interval::<i32>::open_unbounded(0);
        assert!(a.is_separated_from(&b));

        let a = Interval::<i32>::unbounded_open(0);
        let b = Interval::<i32>::closed_unbounded(0);
        assert!(!a.is_separated_from(&b));

        let a = Interval::<i32>::open(0, 2).unwrap();
        let b = Interval::<i32>::closed(1, 1).unwrap();
        assert!(!a.is_separated_from(&b));
    }

    #[test]
    fn test_merge() {
        let a = Interval::<i32>::open(0, 1).unwrap();
        let b = Interval::<i32>::open(1, 2).unwrap();
        let c = a.merge(&b);
        assert!(c.is_err());

        let a = Interval::<i32>::open(0, 1).unwrap();
        let b = Interval::<i32>::closed(1, 2).unwrap();
        let c = a.merge(&b);
        assert!(c.is_ok());
        let c = c.unwrap();
        assert_eq!(c, Interval::<i32>::open_closed(0, 2).unwrap());
    }
}
