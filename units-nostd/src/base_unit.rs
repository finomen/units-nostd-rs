use crate::Rational;
use crate::normalized_unit::NormalizedUnitAdd;
use crate::unit::{Unit, ops};
use core::fmt::{Display, Formatter};

pub const trait BaseUnit:
    ops::Mul + ops::Div + Default + ops::Pow + ops::Scale + Display
{
    const SYMBOL: &'static str;
}

macro_rules! unit {
    ($name:ident, $symbol:expr) => {
        paste::paste! {
            #[derive(Debug, Clone, Copy, core::marker::ConstParamTy)]
            #[derive_const(PartialEq, Eq)]
            pub struct [<$name:camel>];
            const impl BaseUnit for [<$name:camel>] {
                const SYMBOL: &'static str = $symbol;
            }

            const impl $crate::unit::NamedUnit for  [<$name:camel>] {}

            impl core::fmt::Display for [<$name:camel>] {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_str(Self::SYMBOL)
                }
            }

            const impl Default for [<$name:camel>] {
                fn default() -> Self {
                    Self{}
                }
            }

            #[cfg(test)]
            mod [<$name:snake tests>] {
                #[test]
                fn [< test_ $name:snake _display >]() {
                    assert_eq!(
                        format!("{}", super::[<$name:camel>]::default()),
                        $symbol
                    );
                }
            }
        }
    };
}

pub(crate) use unit;

#[cfg(feature = "length")]
unit!(Meters, "m");
#[cfg(feature = "time")]
unit!(Seconds, "s");
#[cfg(feature = "mass")]
unit!(Grams, "g");
#[cfg(feature = "temperature")]
unit!(Kelvins, "K");
#[cfg(feature = "electric_current")]
unit!(Amperes, "A");
#[cfg(feature = "luminous_intensity")]
unit!(Candela, "cd");
#[cfg(feature = "amount_of_substance")]
unit!(Moles, "mol");
#[cfg(feature = "angle")]
unit!(Radians, "rad");

const impl<T> Unit for T
where
    T: [const] BaseUnit + 'static + Copy,
{
    type Normalized = <() as NormalizedUnitAdd<T, 1>>::Result;
    const SYMBOL_SCALE: Rational = Rational::new(1, 1);
}

#[derive(Debug, Clone, Copy, core::marker::ConstParamTy)]
#[derive_const(Default, PartialEq, Eq)]
pub struct Unitless;

impl Display for Unitless {
    fn fmt(&self, _f: &mut Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

const impl Unit for Unitless {
    type Normalized = ();
    const SYMBOL_SCALE: Rational = Rational::new(1, 1);
}

#[cfg(test)]
mod tests {
    use crate::base_unit::Unitless;

    #[test]
    fn test_unitless() {
        assert_eq!(format!("{}", Unitless), "");
    }
}
