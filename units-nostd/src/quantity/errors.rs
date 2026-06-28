use core::convert::Infallible;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ConversionError<VE = Infallible, NE = Infallible, DE = Infallible, OE = Infallible> {
    ValueConversionError(VE),
    NumeratorConversionError(NE),
    DenominatorConversionError(DE),
    OffsetConversionError(OE),
    NumCastFailed,
}

impl<VE, NE, DE, OE> Display for ConversionError<VE, NE, DE, OE>
where
    VE: Display,
    NE: Display,
    DE: Display,
    OE: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ConversionError::ValueConversionError(e) => {
                write!(f, "Value conversion error: ({})", e)
            }
            ConversionError::NumeratorConversionError(e) => {
                write!(f, "Numerator conversion error: ({})", e)
            }
            ConversionError::DenominatorConversionError(e) => {
                write!(f, "Denominator conversion error: ({})", e)
            }
            ConversionError::OffsetConversionError(e) => {
                write!(f, "Offset conversion error: ({})", e)
            }
            ConversionError::NumCastFailed => write!(f, "NumCast failed"),
        }
    }
}
impl<VE, NE, DE, OE> Error for ConversionError<VE, NE, DE, OE>
where
    VE: Error,
    NE: Error,
    DE: Error,
    OE: Error,
{
}

#[cfg(test)]
mod tests {
    use crate::quantity::errors::ConversionError;
    use core::convert::Infallible;

    #[test]
    fn test_error_format() {
        assert_eq!(
            format!(
                "{}",
                ConversionError::<ConversionError>::ValueConversionError(
                    ConversionError::NumCastFailed
                )
            ),
            "Value conversion error: (NumCast failed)"
        );
        assert_eq!(
            format!(
                "{}",
                ConversionError::<Infallible, ConversionError>::NumeratorConversionError(
                    ConversionError::NumCastFailed
                )
            ),
            "Numerator conversion error: (NumCast failed)"
        );
        assert_eq!(format!("{}", ConversionError::<Infallible, Infallible, ConversionError>::DenominatorConversionError(ConversionError::NumCastFailed)), "Denominator conversion error: (NumCast failed)");
        assert_eq!(format!("{}", ConversionError::<Infallible, Infallible, Infallible, ConversionError>::OffsetConversionError(ConversionError::NumCastFailed)), "Offset conversion error: (NumCast failed)");
    }
}
