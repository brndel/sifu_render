use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dim {
    D2,
    D3,
    D4,
}

impl From<Dim> for u8 {
    fn from(value: Dim) -> Self {
        match value {
            Dim::D2 => 2,
            Dim::D3 => 3,
            Dim::D4 => 4,
        }
    }
}

impl From<Dim> for usize {
    fn from(value: Dim) -> Self {
        match value {
            Dim::D2 => 2,
            Dim::D3 => 3,
            Dim::D4 => 4,
        }
    }
}

pub enum ParseDimError {
    Int(ParseIntError),
    InvalidDim(u8),
}

impl Display for ParseDimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseDimError::Int(err) => write!(f, "{}", err),
            ParseDimError::InvalidDim(dim) => write!(
                f,
                "`{}` is not allowed as a dimension

allowed dimensions are
- 1, 2, 3",
                dim
            ),
        }
    }
}

impl From<ParseIntError> for ParseDimError {
    fn from(value: ParseIntError) -> Self {
        Self::Int(value)
    }
}

impl FromStr for Dim {
    type Err = ParseDimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dim = s.parse::<u8>()?;
        match dim {
            2 => Ok(Self::D2),
            3 => Ok(Self::D3),
            4 => Ok(Self::D4),
            _ => Err(ParseDimError::InvalidDim(dim)),
        }
    }
}

impl Display for Dim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", usize::from(*self))
    }
}
