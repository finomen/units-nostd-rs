use crate::quantity::QuantityInfo;
use crate::{Rational, unit};

pub(crate) type U<U1> = <U1 as QuantityInfo>::Unit;
pub(crate) type Mul<U1, U2> = <U1 as unit::ops::Mul>::Mul<U2>;
pub(crate) type Div<U1, U2> = <U1 as unit::ops::Div>::Div<U2>;
pub(crate) type Scale<U1, const S: Rational> = <U1 as unit::ops::Scale>::Scale<S>;
pub(crate) type Pow<U1, const P: i32> = <U1 as unit::ops::Pow>::Pow<P>;
pub(crate) type Alias<U1, const S: &'static str> = <U1 as unit::ops::Alias>::Alias<S>;
