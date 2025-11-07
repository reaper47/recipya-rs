use schemars::JsonSchema;
use serde::Deserialize;

/// Data type: Number.
///
/// Usage guidelines:
///
/// - Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.
/// - Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl Default for Number {
    fn default() -> Self {
        Number::Integer(0)
    }
}

impl From<i32> for Number {
    fn from(v: i32) -> Self {
        Number::Integer(v as i64)
    }
}

impl From<i64> for Number {
    fn from(v: i64) -> Self {
        Number::Integer(v)
    }
}

impl From<f32> for Number {
    fn from(v: f32) -> Self {
        Number::Float(v as f64)
    }
}

impl From<f64> for Number {
    fn from(v: f64) -> Self {
        Number::Float(v)
    }
}

impl Number {
    pub fn as_f64(&self) -> f64 {
        match self {
            Number::Integer(i) => *i as f64,
            Number::Float(f) => *f,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Number::Integer(i) => Some(*i),
            Number::Float(f) => {
                if f.fract() == 0.0 && f.is_finite() {
                    Some(*f as i64)
                } else {
                    None
                }
            }
        }
    }
}
