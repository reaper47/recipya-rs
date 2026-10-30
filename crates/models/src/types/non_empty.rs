use std::fmt;

use crate::Error;

/// A vector that always contains at least one element.
#[derive(Debug, Clone)]
pub struct NonEmptyVec<T>(Vec<T>);

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = Error;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(Error::EmptyInput)
        } else {
            Ok(Self(value))
        }
    }
}

impl<T> AsRef<[T]> for NonEmptyVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T: fmt::Debug> fmt::Display for NonEmptyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_is_empty() {
        let got = NonEmptyVec::try_from(Vec::<i64>::new());

        assert!(got.is_err());
    }

    #[test]
    fn test_vector_contains_elements() {
        let got = NonEmptyVec::try_from(vec![1, 2, 3]).unwrap();

        assert_eq!(got.as_ref(), &[1, 2, 3]);
    }
}
