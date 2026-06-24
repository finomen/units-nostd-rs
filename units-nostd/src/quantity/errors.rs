use core::error::Error;
use core::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ConversionError<SE, VE> {
    ScaleFailed(SE),
    ValueConversionFailed(VE),
}

impl<SE, VE> Display for ConversionError<SE, VE>
where
    SE: Display,
    VE: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ConversionError::ScaleFailed(e) => write!(f, "Scale conversion failed: {}", e),
            ConversionError::ValueConversionFailed(e) => {
                write!(f, "Value conversion failed: {}", e)
            }
        }
    }
}
impl<SE, VE> Error for ConversionError<SE, VE>
where
    SE: Display,
    VE: Display,
    SE: Debug,
    VE: Debug,
{
}
