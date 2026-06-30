# Unit library for no_std environments

This library provides an embedded `no_std` library for type-safe unit conversions and calculations in no_std environments. This is nightly-only crate.

[![Crates.io](https://img.shields.io/crates/v/units-nostd.svg)](https://crates.io/crates/units-nostd)
[![Docs.rs](https://docs.rs/units-nostd/badge.svg)](https://docs.rs/units-nostd)
[![codecov](https://codecov.io/gh/finomen/units-nostd-rs/graph/badge.svg?token=4TU9A0sqb4)](https://codecov.io/gh/finomen/units-nostd-rs)

## Features

Every quantity lives behind its own feature flag. The base-unit modules are
enabled by default; derived units are opt-in and transitively pull in the
modules they are built from. Enable everything at once with the `full` feature.

### Base units (enabled by default)

| Feature             | Dependencies  Units       |
|---------------------|---------------------------|
| length              | m, km, dm, cm, mm, μm, nm |
| angle               | rad                       |
| time                | s, min, h                 |
| mass                | g, kg, mg, μg             |
| temperature         | K, mK, ℃                  |
| electric_current    | A                         |
| luminous_intensity  | cd                        |
| amount_of_substance | mol                       |
| parts_per           | %, ‰, ‱, ppm, ppb, ppt    |

### Derived units (opt-in)

| Feature                           | Dependencies                             | Units                  |
|-----------------------------------|------------------------------------------|------------------------|
| area                              | length                                   | m², km², dm², cm², mm² |
| volume                            | length                                   | m³, dm³, cm³, mm³      |
| acceleration                      | length, time                             | m/s²                   |
| velocity                          | length, time                             | ㎧, kph                 |
| wave_number                       | length                                   | m⁻¹                    |
| mass_density                      | length, mass, volume                     | kg/m³, g/cm³, μg/m³    |
| specific_volume                   | length, mass, volume                     | m³/kg                  |
| current_density                   | area, electric_current                   | A/m²                   |
| magnetic_field_strength           | electric_current, length                 | A/m                    |
| amount_of_substance_concentration | amount_of_substance, volume              | mol/m³                 |
| luminance                         | area, luminous_intensity                 | cd/m²                  |
| frequency                         | time                                     | Hz                     |
| force                             | acceleration, length, mass               | N                      |
| energy                            | force, length                            | J                      |
| electric_charge                   | electric_current, time                   | C                      |
| power                             | energy, time                             | W                      |
| potential_difference              | electric_current, power                  | V                      |
| capacitance                       | electric_charge, potential_difference    | F                      |
| electrical_resistance             | electric_current, potential_difference   | Ω                      |
| magnetic_flux                     | potential_difference, time               | Wb                     |
| magnetic_flux_density             | area, magnetic_flux                      | T                      |
| inductance                        | electric_current, magnetic_flux          | H                      |
| pressure                          | area, force                              | Pa, MPa, kPa, hPa, mPa |
| dynamic_viscosity                 | pressure, time                           | Pa·s                   |
| surface_tension                   | force, length                            | N/m                    |
| angular_velocity                  | angle, time                              | rad/s                  |
| angular_acceleration              | angle, time                              | rad/s²                 |
| heat_flux_density                 | length, power                            | W/m²                   |
| entropy                           | energy, temperature                      | J/K                    |
| specific_heat_capacity            | energy, mass, temperature                | J/(kg·K)               |
| specific_energy                   | energy, mass                             | J/kg                   |
| thermal_conductivity              | energy, length, power, temperature       | W/(m²·K)               |
| electric_field_strength           | length, potential_difference             | V/m                    |
| electric_charge_density           | electric_charge, length                  | C/m³                   |
| electric_flux_density             | electric_charge, length                  | C/m²                   |
| molar_energy                      | amount_of_substance, energy              | J/mol                  |
| molar_entropy                     | amount_of_substance, energy, temperature | J/(mol·K)              |

### Meta features

| Feature | Description                                          |
|---------|------------------------------------------------------|
| full    | Enables every base and derived quantity module.      |
| serde   | Derives `serde` `Serialize`/`Deserialize` support.   |

## Used unstable features

- [generic_const_exprs](https://doc.rust-lang.org/beta/unstable-book/language-features/generic-const-exprs.html) Tracking issue [#76560](https://github.com/rust-lang/rust/issues/76560)
- [const_ops](https://doc.rust-lang.org/beta/unstable-book/library-features/const-ops.html) Tracking issue [#143802](https://github.com/rust-lang/rust/issues/143802)
- [const_cmp](https://doc.rust-lang.org/beta/unstable-book/library-features/const-cmp.html) Tracking issue  [#143800](https://github.com/rust-lang/rust/issues/143800)
- [adt_const_params](https://doc.rust-lang.org/beta/unstable-book/language-features/adt-const-params.html) Tracking issue [#95174](https://github.com/rust-lang/rust/issues/95174)
- [const_param_ty_trait](https://doc.rust-lang.org/beta/unstable-book/library-features/const-param-ty-trait.html) Tracking issue [#95174](https://github.com/rust-lang/rust/issues/95174)
- [min_adt_const_params](https://doc.rust-lang.org/beta/unstable-book/language-features/min-adt-const-params.html) Tracking issue [#154042](https://github.com/rust-lang/rust/issues/154042)
- [const_trait_impl](https://doc.rust-lang.org/beta/unstable-book/language-features/const-trait-impl.html) Tracking issue [#143874](https://github.com/rust-lang/rust/issues/143874)
- [derive_const](https://doc.rust-lang.org/beta/unstable-book/library-features/derive-const.html) Tracking issue [#118304](https://github.com/rust-lang/rust/issues/118304)
- [inherent_associated_types](https://doc.rust-lang.org/beta/unstable-book/language-features/inherent-associated-types.html) Tracking issue [#8995](https://github.com/rust-lang/rust/issues/8995)
- [const_default](https://doc.rust-lang.org/beta/unstable-book/library-features/const-default.html) Tracking issue [#143894](https://github.com/rust-lang/rust/issues/143894)
- [min_generic_const_args](https://doc.rust-lang.org/beta/unstable-book/language-features/min-generic-const-args.html) Tracking issue [#132980](https://github.com/rust-lang/rust/issues/132980)
- [generic_const_parameter_types](https://doc.rust-lang.org/beta/unstable-book/language-features/generic-const-parameter-types.html) Tracking issue [#137626](https://github.com/rust-lang/rust/issues/137626)
- [unsized_const_params](https://doc.rust-lang.org/beta/unstable-book/language-features/unsized-const-params.html) Tracking issue [#95174](https://github.com/rust-lang/rust/issues/95174)
- [const_type_name](https://doc.rust-lang.org/beta/unstable-book/library-features/const-type-name.html) Tracking issue [#63084](https://github.com/rust-lang/rust/issues/63084)
- [specialization](https://doc.rust-lang.org/beta/unstable-book/language-features/specialization.html) Tracking issue [#31844](https://github.com/rust-lang/rust/issues/31844)

## Add to project

```toml
[dependencies]
units-nostd = { version = "0.0.1", features = ["full"] }
```



## GenAI Usage

This project was created with the assistance of Claude and Gemini. Primarily, GenAI is used to:
- Generate documentation
- Fix warnings and clean up code

## License

Licensed under [MIT license](https://github.com/finomen/sen6x-rs/blob/master/LICENSE)

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed under MIT License, without any additional terms or conditions.