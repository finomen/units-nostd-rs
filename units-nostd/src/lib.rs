//! This library provides an embedded `no_std` library for type-safe unit conversions and calculations in no_std environments. This is nightly-only crate.
//!
//! # Base units
//!
//! ## Length
//! ```
//! assert_eq!(units::length::Meters::new(10).value(), 10);
//! assert_eq!(format!("{}", units::length::Meters::new(10)), "10m");
//! ```
//! Scaled variants:
//! ```
//! assert_eq!(format!("{}", units::length::KiloMeters::new(10)), "10km");
//! assert_eq!(format!("{}", units::length::DeciMeters::new(10)), "10dm");
//! assert_eq!(format!("{}", units::length::CentiMeters::new(10)), "10cm");
//! assert_eq!(format!("{}", units::length::MilliMeters::new(10)), "10mm");
//! assert_eq!(format!("{}", units::length::MicroMeters::new(10)), "10μm");
//! assert_eq!(format!("{}", units::length::NanoMeters::new(10)), "10nm");
//! ```
//!
//! ## Time
//! ```
//! assert_eq!(units::time::Seconds::new(10).value(), 10);
//! assert_eq!(format!("{}", units::time::Seconds::new(10)), "10s");
//! ```
//! Scaled variants:
//! ```
//! assert_eq!(format!("{}", units::time::Minutes::new(10)), "10min");
//! assert_eq!(format!("{}", units::time::Hours::new(10)), "10h");
//! ```
//!
//! ## Mass
//! ```
//! assert_eq!(units::mass::Grams::new(10).value(), 10);
//! assert_eq!(format!("{}", units::mass::Grams::new(10)), "10g");
//! ```
//! Scaled variants:
//! ```
//! assert_eq!(format!("{}", units::mass::KiloGrams::new(10)), "10kg");
//! assert_eq!(format!("{}", units::mass::MilliGrams::new(10)), "10mg");
//! assert_eq!(format!("{}", units::mass::MicroGrams::new(10)), "10μg");
//! ```
//!
//! ## Temperature
//! ```
//! assert_eq!(units::temperature::Kelvins::new(10).value(), 10);
//! assert_eq!(format!("{}", units::temperature::Kelvins::new(10)), "10K");
//! assert_eq!(format!("{}", units::temperature::MilliKelvins::new(10)), "10mK");
//! assert_eq!(format!("{}", units::temperature::DegreesCelsius::new(10)), "10℃");
//! ```
//!
//! ## Electric current
//! ```
//! assert_eq!(units::electric_current::Amperes::new(10).value(), 10);
//! assert_eq!(format!("{}", units::electric_current::Amperes::new(10)), "10A");
//! ```
//!
//! ## Luminous intensity
//! ```
//! assert_eq!(units::luminous_intensity::Candelas::new(10).value(), 10);
//! assert_eq!(format!("{}", units::luminous_intensity::Candelas::new(10)), "10cd");
//! ```
//!
//! ## Amount of substance
//! ```
//! assert_eq!(units::amount_of_substance::Moles::new(10).value(), 10);
//! assert_eq!(format!("{}", units::amount_of_substance::Moles::new(10)), "10mol");
//! ```
//!
//! ## Angle
//! ```
//! assert_eq!(units::angle::Radians::new(10).value(), 10);
//! assert_eq!(format!("{}", units::angle::Radians::new(10)), "10rad");
//! ```
//!
//! # Derived units
//!
//! Each derived unit is gated behind its own feature; the examples below are
//! compiled and run only when that feature is enabled (and on [docs.rs], which
//! builds with every feature on).
//!
//! [docs.rs]: https://docs.rs/units-nostd
//!
#![cfg_attr(feature = "area", doc = "## Area")]
#![cfg_attr(feature = "area", doc = "```")]
#![cfg_attr(
    feature = "area",
    doc = "assert_eq!(units::area::MetersSquared::new(10).value(), 10);"
)]
#![cfg_attr(
    feature = "area",
    doc = "assert_eq!(format!(\"{}\", units::area::MetersSquared::new(10)), \"10m²\");"
)]
#![cfg_attr(
    feature = "area",
    doc = "assert_eq!(format!(\"{}\", units::area::KiloMetersSquared::new(10)), \"10km²\");"
)]
#![cfg_attr(
    feature = "area",
    doc = "assert_eq!(format!(\"{}\", units::area::DeciMetersSquared::new(10)), \"10dm²\");"
)]
#![cfg_attr(
    feature = "area",
    doc = "assert_eq!(format!(\"{}\", units::area::CentiMetersSquared::new(10)), \"10cm²\");"
)]
#![cfg_attr(
    feature = "area",
    doc = "assert_eq!(format!(\"{}\", units::area::MilliMetersSquared::new(10)), \"10mm²\");"
)]
#![cfg_attr(feature = "area", doc = "```")]
//!
#![cfg_attr(feature = "volume", doc = "## Volume")]
#![cfg_attr(feature = "volume", doc = "```")]
#![cfg_attr(
    feature = "volume",
    doc = "assert_eq!(units::volume::MetersCubic::new(10).value(), 10);"
)]
#![cfg_attr(
    feature = "volume",
    doc = "assert_eq!(format!(\"{}\", units::volume::MetersCubic::new(10)), \"10m³\");"
)]
#![cfg_attr(
    feature = "volume",
    doc = "assert_eq!(format!(\"{}\", units::volume::DeciMetersCubic::new(10)), \"10dm³\");"
)]
#![cfg_attr(
    feature = "volume",
    doc = "assert_eq!(format!(\"{}\", units::volume::CentiMetersCubic::new(10)), \"10cm³\");"
)]
#![cfg_attr(
    feature = "volume",
    doc = "assert_eq!(format!(\"{}\", units::volume::MilliMetersCubic::new(10)), \"10mm³\");"
)]
#![cfg_attr(feature = "volume", doc = "```")]
//!
#![cfg_attr(feature = "acceleration", doc = "## Acceleration")]
#![cfg_attr(feature = "acceleration", doc = "```")]
#![cfg_attr(
    feature = "acceleration",
    doc = "assert_eq!(format!(\"{}\", units::acceleration::MetersPerSecondPerSecond::new(10)), \"10m/s²\");"
)]
#![cfg_attr(feature = "acceleration", doc = "```")]
//!
#![cfg_attr(feature = "velocity", doc = "## Velocity")]
#![cfg_attr(feature = "velocity", doc = "```")]
#![cfg_attr(
    feature = "velocity",
    doc = "assert_eq!(format!(\"{}\", units::velocity::MetersPerSecond::new(10)), \"10㎧\");"
)]
#![cfg_attr(
    feature = "velocity",
    doc = "assert_eq!(format!(\"{}\", units::velocity::KilometersPerHour::new(10)), \"10kph\");"
)]
#![cfg_attr(feature = "velocity", doc = "```")]
//!
#![cfg_attr(feature = "wave_number", doc = "## Wave number")]
#![cfg_attr(feature = "wave_number", doc = "```")]
#![cfg_attr(
    feature = "wave_number",
    doc = "assert_eq!(format!(\"{}\", units::wave_number::ReciprocalMeter::new(10)), \"10m⁻¹\");"
)]
#![cfg_attr(feature = "wave_number", doc = "```")]
//!
#![cfg_attr(feature = "mass_density", doc = "## Mass density")]
#![cfg_attr(feature = "mass_density", doc = "```")]
#![cfg_attr(
    feature = "mass_density",
    doc = "assert_eq!(format!(\"{}\", units::mass_density::KilogramPerCubicMeter::new(10)), \"10kg/m³\");"
)]
#![cfg_attr(feature = "mass_density", doc = "```")]
//!
#![cfg_attr(feature = "specific_volume", doc = "## Specific volume")]
#![cfg_attr(feature = "specific_volume", doc = "```")]
#![cfg_attr(
    feature = "specific_volume",
    doc = "assert_eq!(format!(\"{}\", units::specific_volume::CubicMeterPerKilogram::new(10)), \"10m³/kg\");"
)]
#![cfg_attr(feature = "specific_volume", doc = "```")]
//!
#![cfg_attr(feature = "current_density", doc = "## Current density")]
#![cfg_attr(feature = "current_density", doc = "```")]
#![cfg_attr(
    feature = "current_density",
    doc = "assert_eq!(format!(\"{}\", units::current_density::AmperePerSquareMeter::new(10)), \"10A/m²\");"
)]
#![cfg_attr(feature = "current_density", doc = "```")]
//!
#![cfg_attr(
    feature = "magnetic_field_strength",
    doc = "## Magnetic field strength"
)]
#![cfg_attr(feature = "magnetic_field_strength", doc = "```")]
#![cfg_attr(
    feature = "magnetic_field_strength",
    doc = "assert_eq!(format!(\"{}\", units::magnetic_field_strength::AmperePerMeter::new(10)), \"10A/m\");"
)]
#![cfg_attr(feature = "magnetic_field_strength", doc = "```")]
//!
#![cfg_attr(
    feature = "amount_of_substance_concentration",
    doc = "## Amount of substance concentration"
)]
#![cfg_attr(feature = "amount_of_substance_concentration", doc = "```")]
#![cfg_attr(
    feature = "amount_of_substance_concentration",
    doc = "assert_eq!(format!(\"{}\", units::amount_of_substance_concentration::MolPerCubicMeter::new(10)), \"10mol/m³\");"
)]
#![cfg_attr(feature = "amount_of_substance_concentration", doc = "```")]
//!
#![cfg_attr(feature = "luminance", doc = "## Luminance")]
#![cfg_attr(feature = "luminance", doc = "```")]
#![cfg_attr(
    feature = "luminance",
    doc = "assert_eq!(format!(\"{}\", units::luminance::CandelaPerSquareMeter::new(10)), \"10cd/m²\");"
)]
#![cfg_attr(feature = "luminance", doc = "```")]
//!
#![cfg_attr(feature = "frequency", doc = "## Frequency")]
#![cfg_attr(feature = "frequency", doc = "```")]
#![cfg_attr(
    feature = "frequency",
    doc = "assert_eq!(format!(\"{}\", units::frequency::Hertz::new(10)), \"10Hz\");"
)]
#![cfg_attr(feature = "frequency", doc = "```")]
//!
#![cfg_attr(feature = "force", doc = "## Force")]
#![cfg_attr(feature = "force", doc = "```")]
#![cfg_attr(
    feature = "force",
    doc = "assert_eq!(format!(\"{}\", units::force::Newtons::new(10)), \"10N\");"
)]
#![cfg_attr(feature = "force", doc = "```")]
//!
#![cfg_attr(feature = "energy", doc = "## Energy")]
#![cfg_attr(feature = "energy", doc = "```")]
#![cfg_attr(
    feature = "energy",
    doc = "assert_eq!(format!(\"{}\", units::energy::Joules::new(10)), \"10J\");"
)]
#![cfg_attr(feature = "energy", doc = "```")]
//!
#![cfg_attr(feature = "electric_charge", doc = "## Electric charge")]
#![cfg_attr(feature = "electric_charge", doc = "```")]
#![cfg_attr(
    feature = "electric_charge",
    doc = "assert_eq!(format!(\"{}\", units::electric_charge::Coulombs::new(10)), \"10C\");"
)]
#![cfg_attr(feature = "electric_charge", doc = "```")]
//!
#![cfg_attr(feature = "power", doc = "## Power")]
#![cfg_attr(feature = "power", doc = "```")]
#![cfg_attr(
    feature = "power",
    doc = "assert_eq!(format!(\"{}\", units::power::Watts::new(10)), \"10W\");"
)]
#![cfg_attr(feature = "power", doc = "```")]
//!
#![cfg_attr(feature = "potential_difference", doc = "## Potential difference")]
#![cfg_attr(feature = "potential_difference", doc = "```")]
#![cfg_attr(
    feature = "potential_difference",
    doc = "assert_eq!(format!(\"{}\", units::potential_difference::Volts::new(10)), \"10V\");"
)]
#![cfg_attr(feature = "potential_difference", doc = "```")]
//!
#![cfg_attr(feature = "capacitance", doc = "## Capacitance")]
#![cfg_attr(feature = "capacitance", doc = "```")]
#![cfg_attr(
    feature = "capacitance",
    doc = "assert_eq!(format!(\"{}\", units::capacitance::Farad::new(10)), \"10F\");"
)]
#![cfg_attr(feature = "capacitance", doc = "```")]
//!
#![cfg_attr(feature = "electrical_resistance", doc = "## Electrical resistance")]
#![cfg_attr(feature = "electrical_resistance", doc = "```")]
#![cfg_attr(
    feature = "electrical_resistance",
    doc = "assert_eq!(format!(\"{}\", units::electrical_resistance::Ohms::new(10)), \"10Ω\");"
)]
#![cfg_attr(feature = "electrical_resistance", doc = "```")]
//!
#![cfg_attr(feature = "magnetic_flux", doc = "## Magnetic flux")]
#![cfg_attr(feature = "magnetic_flux", doc = "```")]
#![cfg_attr(
    feature = "magnetic_flux",
    doc = "assert_eq!(format!(\"{}\", units::magnetic_flux::Weber::new(10)), \"10Wb\");"
)]
#![cfg_attr(feature = "magnetic_flux", doc = "```")]
//!
#![cfg_attr(feature = "magnetic_flux_density", doc = "## Magnetic flux density")]
#![cfg_attr(feature = "magnetic_flux_density", doc = "```")]
#![cfg_attr(
    feature = "magnetic_flux_density",
    doc = "assert_eq!(format!(\"{}\", units::magnetic_flux_density::Tesla::new(10)), \"10T\");"
)]
#![cfg_attr(feature = "magnetic_flux_density", doc = "```")]
//!
#![cfg_attr(feature = "inductance", doc = "## Inductance")]
#![cfg_attr(feature = "inductance", doc = "```")]
#![cfg_attr(
    feature = "inductance",
    doc = "assert_eq!(format!(\"{}\", units::inductance::Henry::new(10)), \"10H\");"
)]
#![cfg_attr(feature = "inductance", doc = "```")]
//!
#![cfg_attr(feature = "pressure", doc = "## Pressure")]
#![cfg_attr(feature = "pressure", doc = "```")]
#![cfg_attr(
    feature = "pressure",
    doc = "assert_eq!(units::pressure::Pascals::new(10).value(), 10);"
)]
#![cfg_attr(
    feature = "pressure",
    doc = "assert_eq!(format!(\"{}\", units::pressure::Pascals::new(10)), \"10Pa\");"
)]
#![cfg_attr(
    feature = "pressure",
    doc = "assert_eq!(format!(\"{}\", units::pressure::MegaPascals::new(10)), \"10MPa\");"
)]
#![cfg_attr(
    feature = "pressure",
    doc = "assert_eq!(format!(\"{}\", units::pressure::KiloPascals::new(10)), \"10kPa\");"
)]
#![cfg_attr(
    feature = "pressure",
    doc = "assert_eq!(format!(\"{}\", units::pressure::HectoPascals::new(10)), \"10hPa\");"
)]
#![cfg_attr(
    feature = "pressure",
    doc = "assert_eq!(format!(\"{}\", units::pressure::MilliPascals::new(10)), \"10mPa\");"
)]
#![cfg_attr(feature = "pressure", doc = "```")]
//!
#![cfg_attr(feature = "dynamic_viscosity", doc = "## Dynamic viscosity")]
#![cfg_attr(feature = "dynamic_viscosity", doc = "```")]
#![cfg_attr(
    feature = "dynamic_viscosity",
    doc = "assert_eq!(format!(\"{}\", units::dynamic_viscosity::PascalSeconds::new(10)), \"10Pa·s\");"
)]
#![cfg_attr(feature = "dynamic_viscosity", doc = "```")]
//!
#![cfg_attr(feature = "surface_tension", doc = "## Surface tension")]
#![cfg_attr(feature = "surface_tension", doc = "```")]
#![cfg_attr(
    feature = "surface_tension",
    doc = "assert_eq!(format!(\"{}\", units::surface_tension::NewtonsPerMeter::new(10)), \"10N/m\");"
)]
#![cfg_attr(feature = "surface_tension", doc = "```")]
//!
#![cfg_attr(feature = "angular_velocity", doc = "## Angular velocity")]
#![cfg_attr(feature = "angular_velocity", doc = "```")]
#![cfg_attr(
    feature = "angular_velocity",
    doc = "assert_eq!(format!(\"{}\", units::angular_velocity::RadiansPerSecond::new(10)), \"10rad/s\");"
)]
#![cfg_attr(feature = "angular_velocity", doc = "```")]
//!
#![cfg_attr(feature = "angular_acceleration", doc = "## Angular acceleration")]
#![cfg_attr(feature = "angular_acceleration", doc = "```")]
#![cfg_attr(
    feature = "angular_acceleration",
    doc = "assert_eq!(format!(\"{}\", units::angular_acceleration::RadiansPerSecond::new(10)), \"10rad/s²\");"
)]
#![cfg_attr(feature = "angular_acceleration", doc = "```")]
//!
#![cfg_attr(feature = "heat_flux_density", doc = "## Heat flux density")]
#![cfg_attr(feature = "heat_flux_density", doc = "```")]
#![cfg_attr(
    feature = "heat_flux_density",
    doc = "assert_eq!(format!(\"{}\", units::heat_flux_density::WattsPerSquareMeter::new(10)), \"10W/m²\");"
)]
#![cfg_attr(feature = "heat_flux_density", doc = "```")]
//!
#![cfg_attr(feature = "entropy", doc = "## Entropy")]
#![cfg_attr(feature = "entropy", doc = "```")]
#![cfg_attr(
    feature = "entropy",
    doc = "assert_eq!(format!(\"{}\", units::entropy::JoulesPerKelvin::new(10)), \"10J/K\");"
)]
#![cfg_attr(feature = "entropy", doc = "```")]
//!
#![cfg_attr(feature = "specific_heat_capacity", doc = "## Specific heat capacity")]
#![cfg_attr(feature = "specific_heat_capacity", doc = "```")]
#![cfg_attr(
    feature = "specific_heat_capacity",
    doc = "assert_eq!(format!(\"{}\", units::specific_heat_capacity::JoulesPerKilogramKelvin::new(10)), \"10J/(kg·K)\");"
)]
#![cfg_attr(feature = "specific_heat_capacity", doc = "```")]
//!
#![cfg_attr(feature = "specific_energy", doc = "## Specific energy")]
#![cfg_attr(feature = "specific_energy", doc = "```")]
#![cfg_attr(
    feature = "specific_energy",
    doc = "assert_eq!(format!(\"{}\", units::specific_energy::JoulesPerKilogram::new(10)), \"10J/kg\");"
)]
#![cfg_attr(feature = "specific_energy", doc = "```")]
//!
#![cfg_attr(feature = "thermal_conductivity", doc = "## Thermal conductivity")]
#![cfg_attr(feature = "thermal_conductivity", doc = "```")]
#![cfg_attr(
    feature = "thermal_conductivity",
    doc = "assert_eq!(format!(\"{}\", units::thermal_conductivity::WattsPerSquareMeterKelvin::new(10)), \"10W/(m²·K)\");"
)]
#![cfg_attr(feature = "thermal_conductivity", doc = "```")]
//!
#![cfg_attr(
    feature = "electric_field_strength",
    doc = "## Electric field strength"
)]
#![cfg_attr(feature = "electric_field_strength", doc = "```")]
#![cfg_attr(
    feature = "electric_field_strength",
    doc = "assert_eq!(format!(\"{}\", units::electric_field_strength::VoltsPerMeter::new(10)), \"10V/m\");"
)]
#![cfg_attr(feature = "electric_field_strength", doc = "```")]
//!
#![cfg_attr(
    feature = "electric_charge_density",
    doc = "## Electric charge density"
)]
#![cfg_attr(feature = "electric_charge_density", doc = "```")]
#![cfg_attr(
    feature = "electric_charge_density",
    doc = "assert_eq!(format!(\"{}\", units::electric_charge_density::CoulombsPerCubicMeter::new(10)), \"10C/m³\");"
)]
#![cfg_attr(feature = "electric_charge_density", doc = "```")]
//!
#![cfg_attr(feature = "electric_flux_density", doc = "## Electric flux density")]
#![cfg_attr(feature = "electric_flux_density", doc = "```")]
#![cfg_attr(
    feature = "electric_flux_density",
    doc = "assert_eq!(format!(\"{}\", units::electric_flux_density::CoulombsPerSquareMeter::new(10)), \"10C/m²\");"
)]
#![cfg_attr(feature = "electric_flux_density", doc = "```")]
//!
#![cfg_attr(feature = "molar_energy", doc = "## Molar energy")]
#![cfg_attr(feature = "molar_energy", doc = "```")]
#![cfg_attr(
    feature = "molar_energy",
    doc = "assert_eq!(format!(\"{}\", units::molar_energy::JoulesPerMole::new(10)), \"10J/mol\");"
)]
#![cfg_attr(feature = "molar_energy", doc = "```")]
//!
#![cfg_attr(feature = "molar_entropy", doc = "## Molar entropy")]
#![cfg_attr(feature = "molar_entropy", doc = "```")]
#![cfg_attr(
    feature = "molar_entropy",
    doc = "assert_eq!(format!(\"{}\", units::molar_entropy::JoulesPerMoleKelvin::new(10)), \"10J/(mol·K)\");"
)]
#![cfg_attr(feature = "molar_entropy", doc = "```")]
//!
#![cfg_attr(not(test), no_std)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features, unused, clippy::neg_multiply)]
#![feature(const_ops)]
#![feature(const_cmp)]
#![feature(adt_const_params)]
#![feature(const_param_ty_trait)]
#![feature(min_adt_const_params)]
#![feature(const_trait_impl)]
#![feature(derive_const)]
#![feature(inherent_associated_types)]
extern crate core;

mod prefixes;
pub mod quantity;
mod scale;

use scale::ONE;

use core::fmt::{Formatter, Pointer};

use quantity::Quantity;
use quantity::si::SiCompoundUnitWrapper;
pub use scale::Scale;

macro_rules! pow_impl {
    ($unit:ty, $pow:literal) => {
        paste::paste! {
            Quantity<T, {$unit::SCALE.pow($pow)}, crate::quantity::si::SiCompoundUnitWrapper<{$unit::UNIT.pow($pow)}>>
        }
    };
}

macro_rules! div_impl {
    ($unit1:ty, $unit2:ty) => {
        paste::paste! {
            Quantity<T, {$unit1::SCALE / $unit2::SCALE}, crate::quantity::si::SiCompoundUnitWrapper<{$unit1::UNIT / $unit2::UNIT}>>
        }
    };
}

macro_rules! mul_impl {
    ($unit1:ty, $unit2:ty) => {
        paste::paste! {
            Quantity<T, {$unit1::SCALE * $unit2::SCALE}, crate::quantity::si::SiCompoundUnitWrapper<{$unit1::UNIT * $unit2::UNIT}>>
        }
    };
}

macro_rules! scaled_impl {
    ($unit:ty, $scale:expr) => {
        paste::paste! {
            Quantity<T, {<$unit>::SCALE * $scale}, crate::quantity::si::SiCompoundUnitWrapper<{<$unit>::UNIT}>>
        }
    };
}

macro_rules! pow {
    ($unit:tt, $pow:literal) => {
        pow_impl!($unit::<()>, $pow)
    };
}

macro_rules! scale {
    ($unit:tt, $scale:expr) => {
        scaled_impl!($unit::<()>, $scale)
    };
}

macro_rules! div {
    ($a:tt, $b:tt) => {
        div_impl!($a::<()>, $b::<()>)
    };
}

macro_rules! mul {
    ($a:tt, $b:tt) => {
        mul_impl!($a::<()>, $b::<()>)
    };
}

macro_rules! named_unit_ex {
    ($alias:ident, $unit:ty, $symbol:expr, $prefix:expr) => {
        #[doc = concat!("Representation of ", $symbol)]
        pub type $alias<T> = $unit;

        impl<T> core::fmt::Display for $alias<T>
        where
            T: core::fmt::Display,
            T: Copy,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}{}{}", self.value(), $prefix, $symbol)
            }
        }

        #[cfg(test)]
        paste::paste! {
            #[test]
            fn [< test_ $alias:snake _display >]() {
                assert_eq!(
                    format!("{}", $alias::new(42_i32)),
                    format!("42{}{}", $prefix, $symbol)
                );
            }
        }
    };
}

macro_rules! named_unit {
    ($alias:ident, $unit:ty, $symbol:expr) => {
        named_unit_ex!($alias, $unit, $symbol, "");
    };
}

macro_rules! unit_with_prefix {
    ($alias:ident, $pow:literal, $symbol:expr, { $( $variant:ident ),* $(,)? }) => {
        $(
            paste::paste! {
                named_unit_ex!([< $variant:camel $alias:camel >], scale!($alias, $crate::prefixes::[<$variant _SCALE>].pow($pow)), $symbol, $crate::prefixes::[<$variant _SYMBOL>]);
            }
        )*
    }
}

macro_rules! scalable_unit {
    ($alias:ident, $unit:ty, $symbol:expr, $pow:literal, { $( $variant:ident ),* $(,)? }) => {
        named_unit!($alias, $unit, $symbol);
        unit_with_prefix!($alias, $pow, $symbol, {$($variant),*});
    }
}

mod base_units {
    use crate::quantity::si::{SiCompoundUnit, SiCompoundUnitWrapper};
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::scale::ONE;

    pub(crate) type Unitless<T> =
        Quantity<T, ONE, SiCompoundUnitWrapper<{ SiCompoundUnit::zero() }>>;

    pub(crate) type Meters<T> = Quantity<T, ONE, crate::quantity::si::Meter<1>>;
    pub(crate) type Seconds<T> = Quantity<T, ONE, crate::quantity::si::Second<1>>;
    pub(crate) type Grams<T> = Quantity<T, ONE, crate::quantity::si::Gram<1>>;
    pub(crate) type Kelvins<T> = Quantity<T, ONE, crate::quantity::si::Kelvin<1>>;
    pub(crate) type Ampere<T> = Quantity<T, ONE, crate::quantity::si::Ampere<1>>;
    pub(crate) type Candela<T> = Quantity<T, ONE, crate::quantity::si::Candela<1>>;
    pub(crate) type Mole<T> = Quantity<T, ONE, crate::quantity::si::Mole<1>>;
    pub(crate) type Radians<T> = Quantity<T, ONE, crate::quantity::si::Radians<1>>;
}

named_unit!(Unitless, base_units::Unitless<T>, "");

#[cfg(feature = "length")]
pub mod length {
    use crate::base_units;
    use crate::quantity::{Quantity, QuantityInfo};

    scalable_unit!(Meters, base_units::Meters<T>, "m", 1, {KILO, DECI, CENTI, MILLI, MICRO, NANO});
}

#[cfg(feature = "angle")]
pub mod angle {
    use crate::prefixes;
    use crate::scale::ONE;
    use crate::{Quantity, base_units};

    named_unit!(Radians, base_units::Radians<T>, "rad");
}

#[cfg(feature = "time")]
pub mod time {
    use crate::base_units;
    use crate::prefixes;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::scale::{ONE, Scale};

    named_unit!(Seconds, base_units::Seconds<T>, "s");
    named_unit!(Minutes, scale!(Seconds, Scale::new(60, 1)), "min");
    named_unit!(Hours, scale!(Minutes, Scale::new(60, 1)), "h");
}

#[cfg(feature = "mass")]
pub mod mass {
    use crate::base_units;
    use crate::prefixes;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::scale::ONE;

    scalable_unit!(Grams, base_units::Grams<T>, "g", 1, {KILO, MILLI, MICRO});
}

#[cfg(feature = "temperature")]
pub mod temperature {
    use crate::base_units;
    use crate::quantity::errors::ConversionError;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::quantity::{UnitConvert, UnitTryConvert, si};
    use crate::scale::ONE;
    use crate::{Scale, prefixes};
    use core::convert::Infallible;
    use core::ops::{Add, Div, Mul, Sub};
    use num::{NumCast, ToPrimitive};

    scalable_unit!(Kelvins, base_units::Kelvins<T>, "K", 1, { MILLI });

    /// Unit for degrees Celsius
    /// ```
    /// let res : Result<units::temperature::MilliKelvins<u64>, _> = units::temperature::DegreesCelsius::new(0).try_convert();
    /// assert_eq!(res, Ok(units::temperature::MilliKelvins::<u64>::new(273150)));
    /// ```
    ///
    /// ```
    /// let res : Result<units::temperature::MilliKelvins<u64>, _> = units::temperature::DegreesCelsius::new(20).try_convert();
    /// assert_eq!(res, Ok(units::temperature::MilliKelvins::<u64>::new(293150)));
    /// ```
    ///
    /// ```
    /// let res : Result<units::temperature::DegreesCelsius<u64>, _> = units::temperature::MilliKelvins::new(293150).try_convert();
    /// assert_eq!(res, Ok(units::temperature::DegreesCelsius::<u64>::new(20)));
    /// ```
    ///
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Celsius;

    const ZERO_CELSIUS: MilliKelvins<u64> = MilliKelvins::new(273_150);

    impl<T, V, const S1: Scale, const S2: Scale> UnitConvert<Celsius, T, V, S1, S2> for si::Kelvin<1>
    where
        V: Sub<V, Output = V>
            + Div<V, Output = V>
            + Mul<V, Output = V>
            + From<T>
            + From<u64>
            + Copy,
    {
        fn convert(value: T) -> V {
            let vk = <si::Kelvin<1> as UnitConvert<si::Kelvin<1>, T, V, S1, S2>>::convert(value);
            let offset: Quantity<V, S2, si::Kelvin<1>> = ZERO_CELSIUS.convert();
            vk - offset.value()
        }
    }

    impl<T, V, const S1: Scale, const S2: Scale, E1> UnitTryConvert<Celsius, T, V, S1, S2>
        for si::Kelvin<1>
    where
        V: Sub<V, Output = V> + Div<V, Output = V> + Mul<V, Output = V> + NumCast + Copy,
        si::Kelvin<1>: UnitTryConvert<si::Kelvin<1>, T, V, S1, S2, Error = E1>,
        E1: core::error::Error,
    {
        type Error = ConversionError<
            E1,
            Infallible,
            Infallible,
            ConversionError<ConversionError, ConversionError, ConversionError, Infallible>,
        >;
        fn try_convert(value: T) -> Result<V, Self::Error> {
            let vk =
                <si::Kelvin<1> as UnitTryConvert<si::Kelvin<1>, T, V, S1, S2>>::try_convert(value)
                    .map_err(ConversionError::ValueConversionError)?;
            let offset: Quantity<V, S2, si::Kelvin<1>> = ZERO_CELSIUS
                .try_convert()
                .map_err(ConversionError::OffsetConversionError)?;
            Ok(vk - offset.value())
        }
    }

    impl<T, V, const S1: Scale, const S2: Scale> UnitConvert<si::Kelvin<1>, T, V, S1, S2> for Celsius
    where
        V: Add<V, Output = V>
            + Div<V, Output = V>
            + Mul<V, Output = V>
            + From<T>
            + From<u64>
            + Copy,
    {
        fn convert(value: T) -> V {
            let vk = <si::Kelvin<1> as UnitConvert<si::Kelvin<1>, T, V, S1, S2>>::convert(value);
            let offset: Quantity<V, S2, si::Kelvin<1>> = ZERO_CELSIUS.convert();
            vk + offset.value()
        }
    }

    impl<T, V, const S1: Scale, const S2: Scale, E1> UnitTryConvert<si::Kelvin<1>, T, V, S1, S2>
        for Celsius
    where
        V: Add<V, Output = V> + Div<V, Output = V> + Mul<V, Output = V> + Copy + NumCast,
        T: ToPrimitive,
        si::Kelvin<1>: UnitTryConvert<si::Kelvin<1>, T, V, S1, S2, Error = E1>,
        E1: core::error::Error,
    {
        type Error = ConversionError<
            E1,
            Infallible,
            Infallible,
            ConversionError<ConversionError, ConversionError, ConversionError, Infallible>,
        >;
        fn try_convert(value: T) -> Result<V, Self::Error> {
            let vk =
                <si::Kelvin<1> as UnitTryConvert<si::Kelvin<1>, T, V, S1, S2>>::try_convert(value)
                    .map_err(ConversionError::ValueConversionError)?;
            let offset: Quantity<V, S2, si::Kelvin<1>> = ZERO_CELSIUS
                .try_convert()
                .map_err(ConversionError::OffsetConversionError)?;
            Ok(vk + offset.value())
        }
    }

    impl<T, V, const S1: Scale, const S2: Scale> UnitConvert<Celsius, T, V, S1, S2> for Celsius
    where
        V: Add<V, Output = V>
            + Div<V, Output = V>
            + Mul<V, Output = V>
            + From<T>
            + From<u64>
            + Copy,
    {
        fn convert(value: T) -> V {
            <si::Kelvin<1> as UnitConvert<si::Kelvin<1>, T, V, S1, S2>>::convert(value)
        }
    }

    impl<T, V, const S1: Scale, const S2: Scale, E1> UnitTryConvert<Celsius, T, V, S1, S2> for Celsius
    where
        V: Add<V, Output = V> + Div<V, Output = V> + Mul<V, Output = V> + NumCast + Copy,
        T: ToPrimitive,
        si::Kelvin<1>: UnitTryConvert<si::Kelvin<1>, T, V, S1, S2, Error = E1>,
        E1: core::error::Error,
    {
        type Error = <si::Kelvin<1> as UnitTryConvert<si::Kelvin<1>, T, V, S1, S2>>::Error;
        fn try_convert(value: T) -> Result<V, Self::Error> {
            <si::Kelvin<1> as UnitTryConvert<si::Kelvin<1>, T, V, S1, S2>>::try_convert(value)
        }
    }

    named_unit!(DegreesCelsius, Quantity<T, ONE, Celsius>, "℃");
}

#[cfg(feature = "electric_current")]
pub mod electric_current {
    use crate::prefixes;
    use crate::scale::ONE;
    use crate::{Quantity, base_units};

    named_unit!(Amperes, base_units::Ampere<T>, "A");
}

#[cfg(feature = "luminous_intensity")]
pub mod luminous_intensity {
    use crate::prefixes;
    use crate::scale::ONE;
    use crate::{Quantity, base_units};

    named_unit!(Candelas, base_units::Candela<T>, "cd");
}

#[cfg(feature = "amount_of_substance")]
pub mod amount_of_substance {
    use crate::prefixes;
    use crate::scale::ONE;
    use crate::{Quantity, base_units};

    named_unit!(Moles, base_units::Mole<T>, "mol");
}

#[cfg(feature = "area")]
pub mod area {
    use crate::length::Meters;
    use crate::prefixes;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::scale::ONE;

    scalable_unit!(MetersSquared, pow!(Meters, 2), "m²", 2, {KILO, DECI, CENTI, MILLI});
}

#[cfg(feature = "volume")]
pub mod volume {
    use crate::length::Meters;
    use crate::prefixes;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::scale::ONE;

    scalable_unit!(MetersCubic, pow!(Meters, 3), "m³", 3, {DECI, CENTI, MILLI});
}

#[cfg(feature = "acceleration")]
pub mod acceleration {
    use crate::length::Meters;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::scale::{ONE, Scale};
    use crate::time::Seconds;

    type S2<T> = pow!(Seconds, 2);
    named_unit!(MetersPerSecondPerSecond, div!(Meters, S2), "m/s²");
}

#[cfg(feature = "velocity")]
pub mod velocity {
    use crate::length::{KiloMeters, Meters};
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::scale::{ONE, Scale};
    use crate::time::{Hours, Seconds};

    named_unit!(MetersPerSecond, div!(Meters, Seconds), "㎧");
    named_unit!(KilometersPerHour, div!(KiloMeters, Hours), "kph");
}

#[cfg(feature = "wave_number")]
pub mod wave_number {
    use crate::length::Meters;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::scale::{ONE, Scale};

    named_unit!(ReciprocalMeter, pow!(Meters, -1), "m⁻¹");
}

#[cfg(feature = "mass_density")]
pub mod mass_density {
    use crate::length::Meters;
    use crate::mass::*;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::scale::{ONE, Scale};
    use crate::volume::*;

    named_unit!(KilogramPerCubicMeter, div!(KiloGrams, MetersCubic), "kg/m³");
    named_unit!(
        GramPerCubicCentieter,
        div!(Grams, CentiMetersCubic),
        "g/cm³"
    );
    named_unit!(
        MicrogramPerCubicMeter,
        div!(MicroGrams, MetersCubic),
        "μg/m³"
    );
}

#[cfg(feature = "specific_volume")]
pub mod specific_volume {
    use crate::length::Meters;
    use crate::mass::KiloGrams;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::scale::{ONE, Scale};
    use crate::volume::MetersCubic;

    named_unit!(CubicMeterPerKilogram, div!(MetersCubic, KiloGrams), "m³/kg");
}

#[cfg(feature = "current_density")]
pub mod current_density {
    use crate::area::MetersSquared;
    use crate::electric_current::Amperes;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::scale::{ONE, Scale};

    named_unit!(AmperePerSquareMeter, div!(Amperes, MetersSquared), "A/m²");
}

#[cfg(feature = "magnetic_field_strength")]
pub mod magnetic_field_strength {
    use crate::electric_current::Amperes;
    use crate::length::Meters;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(AmperePerMeter, div!(Amperes, Meters), "A/m");
}

#[cfg(feature = "amount_of_substance_concentration")]
pub mod amount_of_substance_concentration {
    use crate::amount_of_substance::Moles;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::volume::MetersCubic;

    named_unit!(MolPerCubicMeter, div!(Moles, MetersCubic), "mol/m³");
}

#[cfg(feature = "luminance")]
pub mod luminance {
    use crate::area::MetersSquared;
    use crate::luminous_intensity::Candelas;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(
        CandelaPerSquareMeter,
        div!(Candelas, MetersSquared),
        "cd/m²"
    );
}

#[cfg(feature = "frequency")]
pub mod frequency {
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::time::Seconds;

    named_unit!(Hertz, pow!(Seconds, -1), "Hz");
}

#[cfg(feature = "force")]
pub mod force {
    use crate::acceleration::MetersPerSecondPerSecond;
    use crate::length::Meters;
    use crate::mass::KiloGrams;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(Newtons, mul!(KiloGrams, MetersPerSecondPerSecond), "N");
}

#[cfg(feature = "energy")]
pub mod energy {
    use crate::force::Newtons;
    use crate::length::Meters;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(Joules, mul!(Newtons, Meters), "J");
}

#[cfg(feature = "electric_charge")]
pub mod electric_charge {
    use crate::electric_current::Amperes;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::time::Seconds;

    named_unit!(Coulombs, mul!(Amperes, Seconds), "C");
}

#[cfg(feature = "power")]
pub mod power {
    use crate::energy::Joules;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::time::Seconds;

    named_unit!(Watts, div!(Joules, Seconds), "W");
}

#[cfg(feature = "potential_difference")]
pub mod potential_difference {
    use crate::electric_current::Amperes;
    use crate::power::Watts;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(Volts, div!(Watts, Amperes), "V");
}

#[cfg(feature = "capacitance")]
pub mod capacitance {
    use crate::electric_charge::Coulombs;
    use crate::potential_difference::Volts;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(Farad, div!(Coulombs, Volts), "F");
}

#[cfg(feature = "electrical_resistance")]
pub mod electrical_resistance {
    use crate::electric_current::Amperes;
    use crate::potential_difference::Volts;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(Ohms, div!(Volts, Amperes), "Ω");
}

#[cfg(feature = "magnetic_flux")]
pub mod magnetic_flux {
    use crate::potential_difference::Volts;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::time::Seconds;

    named_unit!(Weber, mul!(Volts, Seconds), "Wb");
}

#[cfg(feature = "magnetic_flux_density")]
pub mod magnetic_flux_density {
    use crate::area::MetersSquared;
    use crate::magnetic_flux::Weber;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(Tesla, mul!(Weber, MetersSquared), "T");
}

#[cfg(feature = "inductance")]
pub mod inductance {
    use crate::electric_current::Amperes;
    use crate::magnetic_flux::Weber;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(Henry, div!(Weber, Amperes), "H");
}

#[cfg(feature = "pressure")]
pub mod pressure {
    use crate::area::MetersSquared;
    use crate::force::Newtons;
    use crate::quantity::{Quantity, QuantityInfo};

    scalable_unit!(Pascals, div!(Newtons, MetersSquared), "Pa", 1, {MEGA,KILO,HECTO,MILLI});
}

#[cfg(feature = "dynamic_viscosity")]
pub mod dynamic_viscosity {
    use crate::pressure::Pascals;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::time::Seconds;

    named_unit!(PascalSeconds, mul!(Pascals, Seconds), "Pa·s");
}

#[cfg(feature = "surface_tension")]
pub mod surface_tension {
    use crate::force::Newtons;
    use crate::length::Meters;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(NewtonsPerMeter, div!(Newtons, Meters), "N/m");
}

#[cfg(feature = "angular_velocity")]
pub mod angular_velocity {
    use crate::angle::Radians;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::time::Seconds;

    named_unit!(RadiansPerSecond, div!(Radians, Seconds), "rad/s");
}

#[cfg(feature = "angular_acceleration")]
pub mod angular_acceleration {
    use crate::angle::Radians;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::time::Seconds;

    type S2<T> = pow!(Seconds, 2);
    named_unit!(RadiansPerSecond, div!(Radians, S2), "rad/s²");
}

#[cfg(feature = "heat_flux_density")]
pub mod heat_flux_density {
    use crate::length::Meters;
    use crate::power::Watts;
    use crate::quantity::{Quantity, QuantityInfo};

    type M2<T> = pow!(Meters, 2);
    named_unit!(WattsPerSquareMeter, div!(Watts, M2), "W/m²");
}

#[cfg(feature = "entropy")]
pub mod entropy {
    use crate::energy::Joules;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::temperature::Kelvins;

    named_unit!(JoulesPerKelvin, div!(Joules, Kelvins), "J/K");
}

#[cfg(feature = "specific_heat_capacity")]
pub mod specific_heat_capacity {
    use crate::energy::Joules;
    use crate::mass::KiloGrams;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::temperature::Kelvins;

    type KgK<T> = mul!(KiloGrams, Kelvins);
    named_unit!(JoulesPerKilogramKelvin, div!(Joules, KgK), "J/(kg·K)");
}

#[cfg(feature = "specific_energy")]
pub mod specific_energy {
    use crate::energy::Joules;
    use crate::mass::KiloGrams;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(JoulesPerKilogram, div!(Joules, KiloGrams), "J/kg");
}

#[cfg(feature = "thermal_conductivity")]
pub mod thermal_conductivity {
    use crate::energy::Joules;
    use crate::length::Meters;
    use crate::power::Watts;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::temperature::Kelvins;

    type M2<T> = pow!(Meters, 2);
    type M2K<T> = mul!(M2, Kelvins);
    named_unit!(WattsPerSquareMeterKelvin, div!(Watts, M2K), "W/(m²·K)");
}

#[cfg(feature = "electric_field_strength")]
pub mod electric_field_strength {
    use crate::length::Meters;
    use crate::potential_difference::Volts;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(VoltsPerMeter, div!(Volts, Meters), "V/m");
}

#[cfg(feature = "electric_charge_density")]
pub mod electric_charge_density {
    use crate::electric_charge::Coulombs;
    use crate::length::Meters;
    use crate::quantity::{Quantity, QuantityInfo};

    type M3<T> = pow!(Meters, 3);
    named_unit!(CoulombsPerCubicMeter, div!(Coulombs, M3), "C/m³");
}

#[cfg(feature = "electric_flux_density")]
pub mod electric_flux_density {
    use crate::electric_charge::Coulombs;
    use crate::length::Meters;
    use crate::quantity::{Quantity, QuantityInfo};

    type M2<T> = pow!(Meters, 2);
    named_unit!(CoulombsPerSquareMeter, div!(Coulombs, M2), "C/m²");
}

#[cfg(feature = "molar_energy")]
pub mod molar_energy {
    use crate::amount_of_substance::Moles;
    use crate::energy::Joules;
    use crate::quantity::{Quantity, QuantityInfo};

    named_unit!(JoulesPerMole, div!(Joules, Moles), "J/mol");
}

#[cfg(feature = "molar_entropy")]
pub mod molar_entropy {
    use crate::amount_of_substance::Moles;
    use crate::energy::Joules;
    use crate::quantity::{Quantity, QuantityInfo};
    use crate::temperature::Kelvins;

    type MK<T> = mul!(Moles, Kelvins);
    named_unit!(JoulesPerMoleKelvin, div!(Joules, MK), "J/(mol·K)");
}

#[cfg(all(
    test,
    feature = "length",
    feature = "time",
    feature = "mass",
    feature = "area",
    feature = "velocity",
    feature = "acceleration",
    feature = "force",
    feature = "frequency",
))]
mod tests {
    use super::*;
    use crate::acceleration::MetersPerSecondPerSecond;
    use crate::area::CentiMetersSquared;
    use crate::force::Newtons;
    use crate::length::{Meters, MicroMeters};
    use crate::mass::KiloGrams;

    #[test]
    fn it_works() {
        let m = length::Meters::new(10);
        let s = time::Seconds::new(2);
        let m2 = m * m;
        let mps = m / s;
        let kph = velocity::KilometersPerHour::new(15);
        assert_eq!(format!("{}", m), "10m");
        assert_eq!(format!("{}", MicroMeters::new(10)), "10μm");
        assert_eq!(format!("{}", m2), "100m²");
        assert_eq!(format!("{}", mps), "5㎧");
        assert_eq!(format!("{}", kph), "15kph");
        assert_eq!(format!("{}", Unitless::new(4) / s), "2Hz");

        let kph2: velocity::KilometersPerHour<i32> =
            velocity::MetersPerSecond::new(20).try_convert().unwrap();

        assert_eq!(format!("{}", kph2), "72kph");
        assert_eq!(format!("{}", kph2 + kph2), "144kph");

        let m1 = Meters::new(1);
        let sm1 = m1 * m1;
        let cm2: CentiMetersSquared<i32> = sm1.try_convert().unwrap();

        assert_eq!(format!("{}", cm2), "10000cm²");

        assert_eq!(
            format!("{}", KiloGrams::new(10) * MetersPerSecondPerSecond::new(10)),
            "100N"
        );
    }
}
