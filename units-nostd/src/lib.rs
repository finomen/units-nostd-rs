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
//!
#![doc(test(attr(
    allow(incomplete_features),
    feature(
        generic_const_exprs,
        const_ops,
        const_cmp,
        adt_const_params,
        const_param_ty_trait,
        min_adt_const_params,
        const_trait_impl,
        derive_const,
        inherent_associated_types,
        const_default,
        min_generic_const_args,
        generic_const_parameter_types,
        unsized_const_params,
        const_type_name,
        specialization,
    )
)))]
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
#![cfg_attr(feature = "parts_per", doc = "## Parts-per")]
#![cfg_attr(feature = "parts_per", doc = "```")]
#![cfg_attr(
    feature = "parts_per",
    doc = "assert_eq!(format!(\"{}\", units::parts_per::Percent::new(10)), \"10%\");"
)]
#![cfg_attr(
    feature = "parts_per",
    doc = "assert_eq!(format!(\"{}\", units::parts_per::PerMillie::new(10)), \"10‰\");"
)]
#![cfg_attr(
    feature = "parts_per",
    doc = "assert_eq!(format!(\"{}\", units::parts_per::PerMiriad::new(10)), \"10‱\");"
)]
#![cfg_attr(
    feature = "parts_per",
    doc = "assert_eq!(format!(\"{}\", units::parts_per::PerCentiMillie::new(10)), \"10pcm\");"
)]
#![cfg_attr(
    feature = "parts_per",
    doc = "assert_eq!(format!(\"{}\", units::parts_per::PerMillion::new(10)), \"10ppm\");"
)]
#![cfg_attr(
    feature = "parts_per",
    doc = "assert_eq!(format!(\"{}\", units::parts_per::PerBillion::new(10)), \"10ppb\");"
)]
#![cfg_attr(
    feature = "parts_per",
    doc = "assert_eq!(format!(\"{}\", units::parts_per::PerTrillion::new(10)), \"10ppt\");"
)]
#![cfg_attr(feature = "parts_per", doc = "```")]
//!
#![cfg_attr(not(test), no_std)]
#![allow(incomplete_features, clippy::neg_multiply)]
#![feature(generic_const_exprs)]
#![feature(const_ops)]
#![feature(const_cmp)]
#![feature(adt_const_params)]
#![feature(min_adt_const_params)]
#![feature(const_trait_impl)]
#![feature(derive_const)]
#![feature(inherent_associated_types)]
#![feature(const_default)]
#![feature(min_generic_const_args)]
#![feature(generic_const_parameter_types)]
#![feature(unsized_const_params)]
#![feature(const_type_name)]
#![feature(specialization)]

extern crate alloc;
extern crate core;

mod base_unit;
mod composite_unit;
mod normalized_unit;
mod prefixes;
pub mod quantity;
mod scale;
pub mod unit;

use quantity::Quantity;
pub use scale::Rational;

macro_rules! named_unit_ex {
    ($(#[$meta:meta])* $alias:ident, $unit:ty, $symbol:expr, $prefix:expr) => {
        #[doc = concat!("Representation of ", $symbol)]
        $(#[$meta])*
        pub type $alias<T = ()> = $crate::quantity::Quantity<T, $unit>;

        paste::paste! {
            #[cfg(test)]
            mod [< $alias:snake _tests >] {
                #[test]
                fn [< test_ $alias:snake _display >]() {
                    assert_eq!(
                        format!("{}", super::$alias::new(42_i32)),
                        format!("42{}{}", $prefix, $symbol)
                    );
                }
            }
        }
    };
}

macro_rules! named_unit {
    ($(#[$meta:meta])* $alias:ident, $unit:ty, $symbol:expr) => {
        named_unit_ex!($(#[$meta])* $alias, $unit, $symbol, "");
    };
}

macro_rules! unit_with_prefix {
    ($alias:ident, $unit:ty, $pow:expr, $symbol:expr, { $( $variant:ident ),* $(,)? }) => {
        $(
            paste::paste! {
                named_unit_ex!([< $variant:camel $alias:camel >], $crate::unit::helpers::Scale::<$unit, const {$crate::prefixes::[<$variant _SCALE>].pow($pow)}>, $symbol, $crate::prefixes::[<$variant _SYMBOL>]);
            }
        )*
    }
}

macro_rules! scalable_unit {
    ($alias:ident, $unit:ty, $pow:expr, $symbol:expr, { $( $variant:ident ),* $(,)? }) => {
        named_unit!($alias, $unit, $symbol);
        unit_with_prefix!($alias, $unit, $pow, $symbol, {$($variant),*});
    }
}

pub type Unitless<T> = Quantity<T, base_unit::Unitless>;

#[cfg(feature = "length")]
pub mod length {
    use crate::base_unit;
    scalable_unit!(Meters, base_unit::Meters, 1, "m", {KILO, DECI, CENTI, MILLI, MICRO, NANO});
}

#[cfg(feature = "angle")]
pub mod angle {
    use crate::base_unit;

    named_unit!(Radians, base_unit::Radians, "rad");
}

#[cfg(feature = "time")]
pub mod time {
    use crate::base_unit;
    use crate::scale::Rational;
    use crate::unit::helpers::{Alias, Scale, U};

    named_unit!(Seconds, base_unit::Seconds, "s");

    named_unit!(
        Minutes,
        Alias<Scale<base_unit::Seconds, const { Rational::new(60, 1) }>, "min">,
        "min"
    );
    named_unit!(
        Hours,
        Alias<Scale<U<Minutes::<()>>, const { Rational::new(60, 1) }>, "h">,
        "h"
    );
}

#[cfg(feature = "mass")]
pub mod mass {
    use crate::base_unit;

    scalable_unit!(Grams, base_unit::Grams, 1, "g", {KILO, MILLI, MICRO});
}

#[cfg(feature = "temperature")]
pub mod temperature {
    use crate::Rational;
    use crate::base_unit;
    use crate::base_unit::{BaseUnit, unit};
    use crate::quantity::errors::ConversionError;
    use crate::quantity::errors::ConversionError::{
        DenominatorConversionError, NumeratorConversionError, OffsetConversionError,
        ValueConversionError,
    };
    use crate::quantity::{UnitConvert, UnitTryConvert};
    use crate::unit::helpers::U;
    use crate::unit::{Tag, Unit, UnitTag};
    use core::ops::{Add, Div, Mul, Sub};
    use num::{NumCast, ToPrimitive};

    scalable_unit!(Kelvins, base_unit::Kelvins, 1, "K", { MILLI });

    unit!(Celsius, "℃");

    const ZERO_CELSIUS: MilliKelvins<u64> = MilliKelvins::new(273_150);

    impl<T, V, S1, S2> UnitConvert<T, V>
        for (
            UnitTag<<Celsius as Unit>::Normalized>,
            S1,
            UnitTag<<base_unit::Kelvins as Unit>::Normalized>,
            S2,
        )
    where
        S1: Tag<Rational>,
        S2: Tag<Rational>,
        T: Copy + ToPrimitive,
        V: Copy + NumCast + Mul<Output = V> + Div<Output = V> + Add<Output = V>,
    {
        fn convert(v: T) -> V {
            let rescale = S1::VALUE / S2::VALUE;
            let offset_scale = U::<MilliKelvins>::SYMBOL_SCALE / S2::VALUE;
            let offset = V::from(ZERO_CELSIUS.value()).unwrap()
                * V::from(offset_scale.numerator()).unwrap()
                / V::from(offset_scale.denominator()).unwrap();
            V::from(v).unwrap() * V::from(rescale.numerator()).unwrap()
                / V::from(rescale.denominator()).unwrap()
                + offset
        }
    }

    impl<T, V, S1, S2> UnitTryConvert<T, V>
        for (
            UnitTag<<Celsius as Unit>::Normalized>,
            S1,
            UnitTag<<base_unit::Kelvins as Unit>::Normalized>,
            S2,
        )
    where
        S1: Tag<Rational>,
        S2: Tag<Rational>,
        T: ToPrimitive,
        V: NumCast + Mul<Output = V> + Div<Output = V> + Add<Output = V>,
    {
        type Error =
            ConversionError<ConversionError, ConversionError, ConversionError, ConversionError>;
        fn try_convert(v: T) -> Result<V, Self::Error> {
            let rescale = S1::VALUE / S2::VALUE;

            let offset_scale = U::<MilliKelvins>::SYMBOL_SCALE / S2::VALUE;
            let offset = V::from(ZERO_CELSIUS.value())
                .ok_or(OffsetConversionError(ConversionError::NumCastFailed))?
                * V::from(offset_scale.numerator())
                    .ok_or(OffsetConversionError(ConversionError::NumCastFailed))?
                / V::from(offset_scale.denominator())
                    .ok_or(OffsetConversionError(ConversionError::NumCastFailed))?;

            let v_c: V = V::from(v).ok_or(ValueConversionError(ConversionError::NumCastFailed))?;
            let n_c: V = V::from(rescale.numerator())
                .ok_or(NumeratorConversionError(ConversionError::NumCastFailed))?;
            let d_c: V = V::from(rescale.denominator())
                .ok_or(DenominatorConversionError(ConversionError::NumCastFailed))?;
            Ok(v_c * n_c / d_c + offset)
        }
    }

    impl<T, V, S1, S2> UnitConvert<T, V>
        for (
            UnitTag<<base_unit::Kelvins as Unit>::Normalized>,
            S1,
            UnitTag<<Celsius as Unit>::Normalized>,
            S2,
        )
    where
        S1: Tag<Rational>,
        S2: Tag<Rational>,
        T: ToPrimitive,
        V: NumCast + Mul<Output = V> + Div<Output = V> + Sub<Output = V>,
    {
        fn convert(v: T) -> V {
            let rescale = S1::VALUE / S2::VALUE;
            let offset_scale = U::<MilliKelvins>::SYMBOL_SCALE / S2::VALUE;
            let offset = V::from(ZERO_CELSIUS.value()).unwrap()
                * V::from(offset_scale.numerator()).unwrap()
                / V::from(offset_scale.denominator()).unwrap();
            V::from(v).unwrap() * V::from(rescale.numerator()).unwrap()
                / V::from(rescale.denominator()).unwrap()
                - offset
        }
    }

    impl<T, V, S1, S2> UnitTryConvert<T, V>
        for (
            UnitTag<<base_unit::Kelvins as Unit>::Normalized>,
            S1,
            UnitTag<<Celsius as Unit>::Normalized>,
            S2,
        )
    where
        S1: Tag<Rational>,
        S2: Tag<Rational>,
        T: ToPrimitive,
        V: NumCast + Mul<Output = V> + Div<Output = V> + Sub<Output = V>,
    {
        type Error =
            ConversionError<ConversionError, ConversionError, ConversionError, ConversionError>;
        fn try_convert(v: T) -> Result<V, Self::Error> {
            let rescale = S1::VALUE / S2::VALUE;

            let offset_scale = U::<MilliKelvins>::SYMBOL_SCALE / S2::VALUE;
            let offset = V::from(ZERO_CELSIUS.value())
                .ok_or(OffsetConversionError(ConversionError::NumCastFailed))?
                * V::from(offset_scale.numerator())
                    .ok_or(OffsetConversionError(ConversionError::NumCastFailed))?
                / V::from(offset_scale.denominator())
                    .ok_or(OffsetConversionError(ConversionError::NumCastFailed))?;

            let v_c: V = V::from(v).ok_or(ValueConversionError(ConversionError::NumCastFailed))?;
            let n_c: V = V::from(rescale.numerator())
                .ok_or(NumeratorConversionError(ConversionError::NumCastFailed))?;
            let d_c: V = V::from(rescale.denominator())
                .ok_or(DenominatorConversionError(ConversionError::NumCastFailed))?;

            Ok(v_c * n_c / d_c - offset)
        }
    }

    named_unit!(
        /// Unit for degrees Celsius.
        ///
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
        DegreesCelsius, Celsius, "℃");

    #[cfg(test)]
    mod tests {
        use crate::Rational;
        use crate::quantity::Quantity;
        use crate::unit::helpers::Scale;
        use assert_approx_eq::assert_approx_eq;

        #[test]
        fn test_celsius_scale() {
            type DegCelsius =
                Quantity<i16, Scale<crate::temperature::Celsius, const { Rational::new(1, 200) }>>;
            let celsius: DegCelsius = DegCelsius::new(4800);
            let celsius_f: Result<crate::temperature::DegreesCelsius<f32>, _> =
                celsius.try_convert();
            assert_eq!(celsius_f.unwrap().value(), 24.0);
        }

        #[test]
        fn test_try_convert_kelvin_to_celsius() {
            let celsius_f: Result<crate::temperature::DegreesCelsius<f32>, _> =
                crate::temperature::Kelvins::<i32>::new(293).try_convert();
            assert_approx_eq!(celsius_f.unwrap().value(), 19.85, 1e-3);
        }
        #[test]
        fn test_convert_kelvin_to_celsius() {
            let celsius_f: crate::temperature::DegreesCelsius<f32> =
                crate::temperature::Kelvins::<f32>::new(293.15).convert();
            assert_approx_eq!(celsius_f.value(), 20.0, 1e-3);
        }
        #[test]
        fn test_convert_celsius_to_kelvin() {
            let kelvins_f: crate::temperature::Kelvins<f32> =
                crate::temperature::DegreesCelsius::<f32>::new(20.0).convert();
            assert_approx_eq!(kelvins_f.value(), 293.15, 1e-3);
        }
        #[test]
        fn test_try_convert_celsius_to_kelvin() {
            let kelvins_f: Result<crate::temperature::Kelvins<f32>, _> =
                crate::temperature::DegreesCelsius::<f32>::new(20.0).try_convert();
            assert_approx_eq!(kelvins_f.unwrap().value(), 293.15, 1e-3);
        }
    }
}

#[cfg(feature = "electric_current")]
pub mod electric_current {
    use crate::base_unit;

    named_unit!(Amperes, base_unit::Amperes, "A");
}

#[cfg(feature = "luminous_intensity")]
pub mod luminous_intensity {
    use crate::base_unit;

    named_unit!(Candelas, base_unit::Candela, "cd");
}

#[cfg(feature = "amount_of_substance")]
pub mod amount_of_substance {
    use crate::base_unit;

    named_unit!(Moles, base_unit::Moles, "mol");
}

#[cfg(feature = "area")]
pub mod area {
    use crate::base_unit;
    use crate::unit::helpers::Pow;

    scalable_unit!(MetersSquared, Pow<base_unit::Meters, 2>, 2, "m²", {KILO, DECI, CENTI, MILLI});
}

#[cfg(feature = "volume")]
pub mod volume {
    use crate::base_unit;
    use crate::unit::helpers::Pow;

    scalable_unit!(MetersCubic, Pow<base_unit::Meters, 3>, 3, "m³", {DECI, CENTI, MILLI});
}

#[cfg(feature = "acceleration")]
pub mod acceleration {
    use crate::base_unit;
    use crate::unit::helpers::{Div, Pow};

    named_unit!(MetersPerSecondPerSecond, Div<base_unit::Meters, Pow<base_unit::Seconds, 2>>, "m/s²");
}

#[cfg(feature = "velocity")]
pub mod velocity {
    use crate::base_unit;
    use crate::length::KiloMeters;
    use crate::time::Hours;
    use crate::unit::helpers::{Alias, Div, U};

    named_unit!(
        MetersPerSecond,
        Alias<Div<base_unit::Meters, base_unit::Seconds>, "㎧">,
        "㎧"
    );
    named_unit!(
        KilometersPerHour,
        Alias<Div<U<KiloMeters::<()>>, U<Hours::<()>>>, "kph">,
        "kph"
    );
}

#[cfg(feature = "wave_number")]
pub mod wave_number {
    use crate::base_unit;
    use crate::unit::helpers::Pow;

    named_unit!(ReciprocalMeter, Pow<base_unit::Meters, -1>, "m⁻¹");
}

#[cfg(feature = "mass_density")]
pub mod mass_density {
    use crate::mass::*;
    use crate::unit::helpers::{Div, U};
    use crate::volume::*;

    named_unit!(
        KilogramPerCubicMeter,
        Div::<U::<KiloGrams::<()>>, U::<MetersCubic::<()>>>,
        "kg/m³"
    );
    named_unit!(
        GramPerCubicCentieter,
        Div<U<Grams<()>>, U<CentiMetersCubic<()>>>,
        "g/cm³"
    );
    named_unit!(
        MicrogramPerCubicMeter,
        Div<U<MicroGrams<()>>, U<MetersCubic<()>>>,
        "μg/m³"
    );
}

#[cfg(feature = "specific_volume")]
pub mod specific_volume {
    use crate::mass::KiloGrams;
    use crate::unit::helpers::{Div, U};
    use crate::volume::MetersCubic;

    named_unit!(
        CubicMeterPerKilogram,
        Div<U<MetersCubic<()>>, U<KiloGrams<()>>>,
        "m³/kg"
    );
}

#[cfg(feature = "current_density")]
pub mod current_density {
    use crate::area::MetersSquared;
    use crate::electric_current::Amperes;
    use crate::unit::helpers::{Div, U};

    named_unit!(
        AmperePerSquareMeter,
        Div<U<Amperes<()>>, U<MetersSquared<()>>>,
        "A/m²"
    );
}

#[cfg(feature = "magnetic_field_strength")]
pub mod magnetic_field_strength {
    use crate::electric_current::Amperes;
    use crate::length::Meters;
    use crate::unit::helpers::{Div, U};

    named_unit!(AmperePerMeter, Div<U<Amperes<()>>, U<Meters<()>>>, "A/m");
}

#[cfg(feature = "amount_of_substance_concentration")]
pub mod amount_of_substance_concentration {
    use crate::amount_of_substance::Moles;
    use crate::unit::helpers::{Div, U};
    use crate::volume::MetersCubic;

    named_unit!(
        MolPerCubicMeter,
        Div<U<Moles<()>>, U<MetersCubic<()>>>,
        "mol/m³"
    );
}

#[cfg(feature = "luminance")]
pub mod luminance {
    use crate::area::MetersSquared;
    use crate::luminous_intensity::Candelas;
    use crate::unit::helpers::{Div, U};

    named_unit!(
        CandelaPerSquareMeter,
        Div<U<Candelas<()>>, U<MetersSquared<()>>>,
        "cd/m²"
    );
}

#[cfg(feature = "frequency")]
pub mod frequency {
    use crate::time::Seconds;
    use crate::unit::helpers::{Alias, Pow, U};

    named_unit!(Hertz, Alias<Pow<U<Seconds<()>>, -1>, "Hz">, "Hz");
}

#[cfg(feature = "force")]
pub mod force {
    use crate::acceleration::MetersPerSecondPerSecond;
    use crate::mass::KiloGrams;
    use crate::unit::helpers::{Alias, Mul, U};

    named_unit!(
        Newtons,
        Alias<Mul<U<KiloGrams<()>>, U<MetersPerSecondPerSecond<()>>>, "N">,
        "N"
    );
}

#[cfg(feature = "energy")]
pub mod energy {
    use crate::force::Newtons;
    use crate::length::Meters;
    use crate::unit::helpers::{Alias, Mul, U};

    named_unit!(Joules, Alias<Mul<U<Newtons<()>>, U<Meters<()>>>, "J">, "J");
}

#[cfg(feature = "electric_charge")]
pub mod electric_charge {
    use crate::electric_current::Amperes;
    use crate::time::Seconds;
    use crate::unit::helpers::{Alias, Mul, U};

    named_unit!(
        Coulombs,
        Alias<Mul<U<Amperes<()>>, U<Seconds<()>>>, "C">,
        "C"
    );
}

#[cfg(feature = "power")]
pub mod power {
    use crate::energy::Joules;
    use crate::time::Seconds;
    use crate::unit::helpers::{Alias, Div, U};

    named_unit!(Watts, Alias<Div<U<Joules<()>>, U<Seconds<()>>>, "W">, "W");
}

#[cfg(feature = "potential_difference")]
pub mod potential_difference {
    use crate::electric_current::Amperes;
    use crate::power::Watts;
    use crate::unit::helpers::{Alias, Div, U};

    named_unit!(Volts, Alias<Div<U<Watts<()>>, U<Amperes<()>>>, "V">, "V");
}

#[cfg(feature = "capacitance")]
pub mod capacitance {
    use crate::electric_charge::Coulombs;
    use crate::potential_difference::Volts;
    use crate::unit::helpers::{Alias, Div, U};

    named_unit!(Farad, Alias<Div<U<Coulombs<()>>, U<Volts<()>>>, "F">, "F");
}

#[cfg(feature = "electrical_resistance")]
pub mod electrical_resistance {
    use crate::electric_current::Amperes;
    use crate::potential_difference::Volts;
    use crate::unit::helpers::{Alias, Div, U};

    named_unit!(Ohms, Alias<Div<U<Volts<()>>, U<Amperes<()>>>, "Ω">, "Ω");
}

#[cfg(feature = "magnetic_flux")]
pub mod magnetic_flux {
    use crate::potential_difference::Volts;
    use crate::time::Seconds;
    use crate::unit::helpers::{Alias, Mul, U};

    named_unit!(Weber, Alias<Mul<U<Volts<()>>, U<Seconds<()>>>, "Wb">, "Wb");
}

#[cfg(feature = "magnetic_flux_density")]
pub mod magnetic_flux_density {
    use crate::area::MetersSquared;
    use crate::magnetic_flux::Weber;
    use crate::unit::helpers::{Alias, Mul, U};

    named_unit!(
        Tesla,
        Alias<Mul<U<Weber<()>>, U<MetersSquared<()>>>, "T">,
        "T"
    );
}

#[cfg(feature = "inductance")]
pub mod inductance {
    use crate::electric_current::Amperes;
    use crate::magnetic_flux::Weber;
    use crate::unit::helpers::{Alias, Div, U};

    named_unit!(Henry, Alias<Div<U<Weber<()>>, U<Amperes<()>>>, "H">, "H");
}

#[cfg(feature = "pressure")]
pub mod pressure {
    use crate::area::MetersSquared;
    use crate::force::Newtons;
    use crate::unit::helpers::{Alias, Div, U};

    scalable_unit!(Pascals, Alias::<Div::<U::<Newtons::<()>>, U::<MetersSquared<()>>>, "Pa">, 1, "Pa", {MEGA,KILO,HECTO,MILLI});
}

#[cfg(feature = "dynamic_viscosity")]
pub mod dynamic_viscosity {
    use crate::pressure::Pascals;
    use crate::time::Seconds;
    use crate::unit::helpers::{Mul, U};

    named_unit!(PascalSeconds, Mul<U<Pascals<()>>, U<Seconds<()>>>, "Pa·s");
}

#[cfg(feature = "surface_tension")]
pub mod surface_tension {
    use crate::force::Newtons;
    use crate::length::Meters;
    use crate::unit::helpers::{Div, U};

    named_unit!(NewtonsPerMeter, Div<U<Newtons<()>>, U<Meters<()>>>, "N/m");
}

#[cfg(feature = "angular_velocity")]
pub mod angular_velocity {
    use crate::angle::Radians;
    use crate::time::Seconds;
    use crate::unit::helpers::{Div, U};

    named_unit!(RadiansPerSecond, Div<U<Radians>, U<Seconds>>, "rad/s");
}

#[cfg(feature = "angular_acceleration")]
pub mod angular_acceleration {
    use crate::angle::Radians;
    use crate::time::Seconds;
    use crate::unit::helpers::{Div, Pow, U};

    named_unit!(
        RadiansPerSecond,
        Div<U<Radians>, Pow<U<Seconds<()>>, 2>>,
        "rad/s²"
    );
}

#[cfg(feature = "heat_flux_density")]
pub mod heat_flux_density {
    use crate::length::Meters;
    use crate::power::Watts;
    use crate::unit::helpers::{Div, Pow, U};

    named_unit!(
        WattsPerSquareMeter,
        Div<U<Watts>, Pow<U<Meters>, 2>>,
        "W/m²"
    );
}

#[cfg(feature = "entropy")]
pub mod entropy {
    use crate::energy::Joules;
    use crate::temperature::Kelvins;
    use crate::unit::helpers::{Div, U};

    named_unit!(JoulesPerKelvin, Div<U<Joules>, U<Kelvins>>, "J/K");
}

#[cfg(feature = "specific_heat_capacity")]
pub mod specific_heat_capacity {
    use crate::energy::Joules;
    use crate::mass::KiloGrams;
    use crate::temperature::Kelvins;
    use crate::unit::helpers::{Div, Mul, U};

    named_unit!(
        JoulesPerKilogramKelvin,
        Div<U<Joules>, Mul<U<KiloGrams>, U<Kelvins>>>,
        "J/(kg·K)"
    );
}

#[cfg(feature = "specific_energy")]
pub mod specific_energy {
    use crate::energy::Joules;
    use crate::mass::KiloGrams;
    use crate::unit::helpers::{Div, U};

    named_unit!(JoulesPerKilogram, Div<U<Joules>, U<KiloGrams>>, "J/kg");
}

#[cfg(feature = "thermal_conductivity")]
pub mod thermal_conductivity {
    use crate::length::Meters;
    use crate::power::Watts;
    use crate::temperature::Kelvins;
    use crate::unit::helpers::{Div, Mul, Pow, U};

    named_unit!(
        WattsPerSquareMeterKelvin,
        Div<U<Watts>, Mul<Pow<U<Meters>, 2>, U<Kelvins>>>,
        "W/(m²·K)"
    );
}

#[cfg(feature = "electric_field_strength")]
pub mod electric_field_strength {
    use crate::length::Meters;
    use crate::potential_difference::Volts;
    use crate::unit::helpers::{Div, U};

    named_unit!(VoltsPerMeter, Div<U<Volts>, U<Meters>>, "V/m");
}

#[cfg(feature = "electric_charge_density")]
pub mod electric_charge_density {
    use crate::electric_charge::Coulombs;
    use crate::length::Meters;
    use crate::unit::helpers::{Div, Pow, U};

    named_unit!(
        CoulombsPerCubicMeter,
        Div<U<Coulombs>, Pow<U<Meters>, 3>>,
        "C/m³"
    );
}

#[cfg(feature = "electric_flux_density")]
pub mod electric_flux_density {
    use crate::electric_charge::Coulombs;
    use crate::length::Meters;
    use crate::unit::helpers::{Div, Pow, U};

    named_unit!(
        CoulombsPerSquareMeter,
        Div<U<Coulombs>, Pow<U<Meters>, 2>>,
        "C/m²"
    );
}

#[cfg(feature = "molar_energy")]
pub mod molar_energy {
    use crate::amount_of_substance::Moles;
    use crate::energy::Joules;
    use crate::unit::helpers::{Div, U};

    named_unit!(JoulesPerMole, Div<U<Joules<()>>, U<Moles<()>>>, "J/mol");
}

#[cfg(feature = "molar_entropy")]
pub mod molar_entropy {
    use crate::amount_of_substance::Moles;
    use crate::energy::Joules;
    use crate::temperature::Kelvins;
    use crate::unit::helpers::{Div, Mul, U};

    named_unit!(
        JoulesPerMoleKelvin,
        Div<U<Joules<()>>, Mul<U<Moles<()>>, U<Kelvins<()>>>>,
        "J/(mol·K)"
    );
}

#[cfg(feature = "parts_per")]
pub mod parts_per {
    use crate::base_unit;
    use crate::scale::Rational;
    use crate::unit::helpers::{Alias, Scale};

    named_unit!(
        Percent,
        Alias::<Scale::<base_unit::Unitless, const { Rational::new(1, 100) }>, "%">,
        "%"
    );
    named_unit!(
        PerMillie,
        Alias::<Scale::<base_unit::Unitless, const { Rational::new(1, 1000) }>, "‰">,
        "‰"
    );
    named_unit!(
        PerMiriad,
        Alias::<Scale::<base_unit::Unitless, const { Rational::new(1, 10000) }>, "‱">,
        "‱"
    );
    named_unit!(
        PerCentiMillie,
        Alias::<Scale::<base_unit::Unitless, const { Rational::new(1, 100000) }>, "pcm">,
        "pcm"
    );
    named_unit!(
        PerMillion,
        Alias::<Scale::<base_unit::Unitless, const { Rational::new(1, 1000000) }>, "ppm">,
        "ppm"
    );
    named_unit!(
        PerBillion,
        Alias::<Scale::<base_unit::Unitless, const { Rational::new(1, 1000000000) }>, "ppb">,
        "ppb"
    );
    named_unit!(
        PerTrillion,
        Alias::<Scale::<base_unit::Unitless, const { Rational::new(1, 1000000000000) }>, "ppt">,
        "ppt"
    );
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
    use crate::area::{CentiMetersSquared, MetersSquared};
    use crate::force::Newtons;
    use crate::frequency::Hertz;
    use crate::length::{Meters, MicroMeters};
    use crate::mass::KiloGrams;
    use crate::velocity::MetersPerSecond;

    #[test]
    fn it_works() {
        let m = length::Meters::new(10);
        let s = time::Seconds::new(2);
        let m2: MetersSquared<_> = (m * m).alias();
        let mps: MetersPerSecond<_> = (m / s).alias();
        let kph = velocity::KilometersPerHour::new(15);
        assert_eq!(format!("{}", m), "10m");
        assert_eq!(format!("{}", MicroMeters::new(10)), "10μm");
        assert_eq!(format!("{}", m2), "100m²");
        assert_eq!(format!("{}", mps), "5㎧");
        assert_eq!(format!("{}", kph), "15kph");
        let hz: Hertz<i32> = (Unitless::new(4) / s).alias();
        assert_eq!(format!("{}", hz), "2Hz");

        let kph2: velocity::KilometersPerHour<i32> =
            velocity::MetersPerSecond::new(20).try_convert().unwrap();

        assert_eq!(format!("{}", kph2), "72kph");
        assert_eq!(format!("{}", kph2 + kph2), "144kph");

        let m1 = Meters::new(1);
        let sm1 = m1 * m1;
        let cm2: CentiMetersSquared<i32> = sm1.try_convert().unwrap();

        assert_eq!(format!("{}", cm2), "10000cm²");

        let n: Newtons<_> = (KiloGrams::new(10) * MetersPerSecondPerSecond::new(10)).alias();

        assert_eq!(format!("{}", n), "100N");
    }
}
