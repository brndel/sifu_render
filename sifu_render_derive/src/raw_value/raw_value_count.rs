use proc_macro2::Span;
use syn::spanned::Spanned;

#[derive(Debug, Clone, Copy, Default)]
pub enum RawValueCount {
    #[default]
    N1,
    N2,
    N3,
    N4,
}

impl From<RawValueCount> for usize {
    fn from(value: RawValueCount) -> Self {
        match value {
            RawValueCount::N1 => 1,
            RawValueCount::N2 => 2,
            RawValueCount::N3 => 3,
            RawValueCount::N4 => 4,
        }
    }
}

pub enum RawValueError {
    InvalidNumber(Span),
    Syn(syn::Error),
}

impl From<RawValueError> for syn::Error {
    fn from(value: RawValueError) -> Self {
        match value {
            RawValueError::InvalidNumber(span) => Self::new(
                span,
                "invalid number. The only valid numbers are 1, 2, 3, 4",
            ),
            RawValueError::Syn(error) => error,
        }
    }
}

impl From<syn::Error> for RawValueError {
    fn from(value: syn::Error) -> Self {
        Self::Syn(value)
    }
}

impl TryFrom<syn::LitInt> for RawValueCount {
    type Error = RawValueError;

    fn try_from(value: syn::LitInt) -> Result<Self, Self::Error> {
        let value = value.base10_parse()?;
        match value {
            1 => Ok(Self::N1),
            2 => Ok(Self::N2),
            3 => Ok(Self::N3),
            4 => Ok(Self::N4),
            _ => Err(RawValueError::InvalidNumber(value.span())),
        }
    }
}

impl RawValueCount {
    pub fn vertex_format_part(&self) -> &'static str {
        match self {
            RawValueCount::N1 => "",
            RawValueCount::N2 => "x2",
            RawValueCount::N3 => "x3",
            RawValueCount::N4 => "x4",
        }
    }

    pub fn glsl_type(&self) -> Option<&'static str> {
        match self {
            RawValueCount::N1 => None,
            RawValueCount::N2 => Some("vec2"),
            RawValueCount::N3 => Some("vec3"),
            RawValueCount::N4 => Some("vec4"),
        }
    }
}
