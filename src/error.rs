use crate::internal_prelude::*;
use core::fmt;

/// A unified error type for anything returned by a method in the time crate.
///
/// This can be used when you either don't know or don't care about the exact
/// error returned. `Result<_, time::Error>` will work in these situations.
// Boxing the `ComponentRangeError` reduces the size of `Error` from 72 bytes to
// 16.
#[allow(clippy::missing_docs_in_private_items)] // variants only
#[cfg_attr(supports_non_exhaustive, non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ConversionRange,
    ComponentRange(Box<ComponentRangeError>),
    Parse(ParseError),
    IndeterminateOffset,
    #[cfg(not(supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

impl fmt::Display for Error {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            e @ Error::ConversionRange | e @ Error::IndeterminateOffset => e.fmt(f),
            Error::ComponentRange(e) => e.fmt(f),
            Error::Parse(e) => e.fmt(f),
            #[cfg(not(supports_non_exhaustive))]
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

#[cfg(std)]
impl std::error::Error for Error {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            err @ Error::ConversionRange | err @ Error::IndeterminateOffset => Some(err),
            Error::ComponentRange(box_err) => Some(box_err.as_ref()),
            Error::Parse(err) => Some(err),
            #[cfg(not(supports_non_exhaustive))]
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

/// An error type indicating that a conversion failed because the target type
/// could not store the initial value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConversionRangeError;

impl fmt::Display for ConversionRangeError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}

#[cfg(std)]
impl std::error::Error for ConversionRangeError {}

impl From<ConversionRangeError> for Error {
    #[inline(always)]
    fn from(_: ConversionRangeError) -> Self {
        Error::ConversionRange
    }
}

/// An error type indicating that a component provided to a method was out of
/// range, causing a failure.
// i64 is the narrowest type fitting all use cases. This eliminates the need
// for a type parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentRangeError {
    /// Name of the component.
    pub(crate) name: &'static str,
    /// Minimum allowed value, inclusive.
    pub(crate) minimum: i64,
    /// Maximum allowed value, inclusive.
    pub(crate) maximum: i64,
    /// Value that was provided.
    pub(crate) value: i64,
    /// The minimum and/or maximum is only valid with the following values.
    pub(crate) given: Vec<(&'static str, i64)>,
}

impl fmt::Display for ComponentRangeError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} must be in the range {}..={}",
            self.name, self.minimum, self.maximum
        )?;

        let mut iter = self.given.iter();
        if let Some((name, value)) = iter.next() {
            write!(f, " given {}={}", name, value)?;
            for (name, value) in iter {
                write!(f, ", {}={}", name, value)?;
            }
        }

        write!(f, " (was {})", self.value)
    }
}

impl From<ComponentRangeError> for Error {
    #[inline(always)]
    fn from(original: ComponentRangeError) -> Self {
        Error::ComponentRange(Box::new(original))
    }
}

#[cfg(std)]
impl std::error::Error for ComponentRangeError {}

impl From<ParseError> for Error {
    #[inline(always)]
    fn from(original: ParseError) -> Self {
        Error::Parse(original)
    }
}

/// The system's UTC offset could not be determined at the given datetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IndeterminateOffsetError;

impl fmt::Display for IndeterminateOffsetError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The system's UTC offset could not be determined")
    }
}

#[cfg(std)]
impl std::error::Error for IndeterminateOffsetError {}

impl From<IndeterminateOffsetError> for Error {
    #[inline(always)]
    fn from(_: IndeterminateOffsetError) -> Self {
        Error::IndeterminateOffset
    }
}
