use crate::scale::Scale;

macro_rules! metric_prefix {
    ($alias:ident, $pow:literal, $symbol:literal) => {
        paste::paste! {
            pub(crate) const [<$alias _SCALE>] : Scale = Scale::new(10, 1).pow($pow);
            pub(crate) const [<$alias _SYMBOL>] : &'static str  = $symbol;
        }
    };
}

metric_prefix!(PETA, 15, "P");
metric_prefix!(TERA, 12, "T");
metric_prefix!(GIGA, 9, "G");
metric_prefix!(MEGA, 6, "M");
metric_prefix!(KILO, 3, "k");
metric_prefix!(HECTO, 2, "h");
metric_prefix!(DECA, 1, "da");
metric_prefix!(DECI, -1, "d");
metric_prefix!(CENTI, -2, "c");
metric_prefix!(MILLI, -3, "m");
metric_prefix!(MICRO, -6, "μ");
metric_prefix!(NANO, -9, "n");
metric_prefix!(PICO, -12, "p");
metric_prefix!(FEMTO, -15, "f");
