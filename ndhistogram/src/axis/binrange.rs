use std::{
    convert::TryFrom,
    ops::{Range, RangeBounds, RangeFrom, RangeTo},
};
#[derive(Debug)]
pub enum ContinuousBinRange<T> {
    Underflow { end: T },
    Overflow { start: T },
    Bin { start: T, end: T },
}

impl<T> ContinuousBinRange<T> {
    pub fn underflow(end: T) -> Self {
        Self::Underflow { end }
    }
    pub fn overflow(start: T) -> Self {
        Self::Overflow { start }
    }
    pub fn new(start: T, end: T) -> Self {
        Self::Bin { start, end }
    }
}

impl<T: Copy> ContinuousBinRange<T> {
    pub fn start(&self) -> Option<T> {
        match self {
            ContinuousBinRange::Underflow { end: _ } => None,
            ContinuousBinRange::Overflow { start } => Some(*start),
            ContinuousBinRange::Bin { start, end: _ } => Some(*start),
        }
    }

    pub fn end(&self) -> Option<T> {
        match self {
            ContinuousBinRange::Underflow { end } => Some(*end),
            ContinuousBinRange::Overflow { start: _ } => None,
            ContinuousBinRange::Bin { start: _, end } => Some(*end),
        }
    }
}

impl<T> From<Range<T>> for ContinuousBinRange<T> {
    fn from(other: Range<T>) -> Self {
        ContinuousBinRange::Bin {
            start: other.start,
            end: other.end,
        }
    }
}

impl<T> From<RangeTo<T>> for ContinuousBinRange<T> {
    fn from(other: RangeTo<T>) -> Self {
        ContinuousBinRange::Underflow { end: other.end }
    }
}

impl<T> From<RangeFrom<T>> for ContinuousBinRange<T> {
    fn from(other: RangeFrom<T>) -> Self {
        ContinuousBinRange::Overflow { start: other.start }
    }
}

impl<T> TryFrom<ContinuousBinRange<T>> for Range<T> {
    type Error = ();

    fn try_from(value: ContinuousBinRange<T>) -> Result<Self, Self::Error> {
        if let ContinuousBinRange::Bin { start, end } = value {
            return Ok(Range { start, end });
        }
        Err(())
    }
}

impl<T> TryFrom<ContinuousBinRange<T>> for RangeTo<T> {
    type Error = ();

    fn try_from(value: ContinuousBinRange<T>) -> Result<Self, Self::Error> {
        if let ContinuousBinRange::Underflow { end } = value {
            return Ok(RangeTo { end });
        }
        Err(())
    }
}

impl<T> TryFrom<ContinuousBinRange<T>> for RangeFrom<T> {
    type Error = ();

    fn try_from(value: ContinuousBinRange<T>) -> Result<Self, Self::Error> {
        if let ContinuousBinRange::Overflow { start } = value {
            return Ok(RangeFrom { start });
        }
        Err(())
    }
}
