use crate::cooking::units::UnitType;
use crate::cooking::units::traits::UnitOperations;
use crate::cooking::units::volume::units::{Volume, VolumeUnit};

impl UnitOperations for Volume {
    fn abbrev(&self) -> &str {
        use Volume::*;

        match self {
            Millilitre(_) => "ml",
            Centilitre(_) => "cl",
            Decilitre(_) => "dl",
            Litre(_) => "l",
            MetricTeaspoon(_) | AustralianTeaspoon(_) | ImperialTeaspoon(_) | USTeaspoon(_) => {
                "tsp"
            }
            MetricTablespoon(_)
            | AustralianTablespoon(_)
            | ImperialTablespoon(_)
            | USTablespoon(_) => "tbsp",
            MetricDessertspoon(_) | AustralianDessertspoon(_) | ImperialDessertspoon(_) => "dsp",
            MetricCup(_) | AustralianCup(_) | ImperialCup(_) | USCup(_) => "cup",
            ImperialFluidOunce(_) | USFluidOunce(_) => "fl oz",
            ImperialGill(_) => "gill",
            ImperialPint(_) | USPint(_) => "pt",
            ImperialQuart(_) | USQuart(_) => "qt",
            ImperialGallon(_) | USGallon(_) => "gal",
            Jigger(_) => "jig",
        }
    }

    fn unit_type(&self) -> UnitType {
        use VolumeUnit::*;

        match self {
            Volume::Millilitre(_) => UnitType::Volume(Millilitre),
            Volume::Centilitre(_) => UnitType::Volume(Centilitre),
            Volume::Decilitre(_) => UnitType::Volume(Decilitre),
            Volume::Litre(_) => UnitType::Volume(Litre),
            Volume::MetricTeaspoon(_) => UnitType::Volume(MetricTeaspoon),
            Volume::MetricTablespoon(_) => UnitType::Volume(MetricTablespoon),
            Volume::MetricDessertspoon(_) => UnitType::Volume(MetricDessertSpoon),
            Volume::MetricCup(_) => UnitType::Volume(MetricCup),
            Volume::AustralianTeaspoon(_) => UnitType::Volume(AustralianTeaspoon),
            Volume::AustralianDessertspoon(_) => UnitType::Volume(AustralianDessertspoon),
            Volume::AustralianTablespoon(_) => UnitType::Volume(AustralianTablespoon),
            Volume::AustralianCup(_) => UnitType::Volume(AustralianCup),
            Volume::ImperialTeaspoon(_) => UnitType::Volume(ImperialTeaspoon),
            Volume::ImperialDessertspoon(_) => UnitType::Volume(ImperialDessertspoon),
            Volume::ImperialTablespoon(_) => UnitType::Volume(ImperialTablespoon),
            Volume::ImperialFluidOunce(_) => UnitType::Volume(ImperialFluidOunce),
            Volume::ImperialGill(_) => UnitType::Volume(ImperialGill),
            Volume::ImperialCup(_) => UnitType::Volume(ImperialCup),
            Volume::ImperialPint(_) => UnitType::Volume(ImperialPint),
            Volume::ImperialQuart(_) => UnitType::Volume(ImperialQuart),
            Volume::ImperialGallon(_) => UnitType::Volume(ImperialGallon),
            Volume::USTeaspoon(_) => UnitType::Volume(USTeaspoon),
            Volume::USTablespoon(_) => UnitType::Volume(USTablespoon),
            Volume::USFluidOunce(_) => UnitType::Volume(USFluidOunce),
            Volume::USCup(_) => UnitType::Volume(USCup),
            Volume::USPint(_) => UnitType::Volume(USPint),
            Volume::USQuart(_) => UnitType::Volume(USQuart),
            Volume::USGallon(_) => UnitType::Volume(USGallon),
            Volume::Jigger(_) => UnitType::Volume(Jigger),
        }
    }

    fn value(&self) -> f64 {
        match self {
            Volume::Millilitre(v) => *v,
            Volume::Centilitre(v) => *v,
            Volume::Decilitre(v) => *v,
            Volume::Litre(v) => *v,
            Volume::MetricTeaspoon(v) => *v,
            Volume::MetricTablespoon(v) => *v,
            Volume::MetricDessertspoon(v) => *v,
            Volume::MetricCup(v) => *v,
            Volume::AustralianTeaspoon(v) => *v,
            Volume::AustralianDessertspoon(v) => *v,
            Volume::AustralianTablespoon(v) => *v,
            Volume::AustralianCup(v) => *v,
            Volume::ImperialTeaspoon(v) => *v,
            Volume::ImperialDessertspoon(v) => *v,
            Volume::ImperialTablespoon(v) => *v,
            Volume::ImperialFluidOunce(v) => *v,
            Volume::ImperialGill(v) => *v,
            Volume::ImperialCup(v) => *v,
            Volume::ImperialPint(v) => *v,
            Volume::ImperialQuart(v) => *v,
            Volume::ImperialGallon(v) => *v,
            Volume::USTeaspoon(v) => *v,
            Volume::USTablespoon(v) => *v,
            Volume::USFluidOunce(v) => *v,
            Volume::USCup(v) => *v,
            Volume::USPint(v) => *v,
            Volume::USQuart(v) => *v,
            Volume::USGallon(v) => *v,
            Volume::Jigger(v) => *v,
        }
    }

    fn with_value(&self, value: f64) -> Self {
        match self {
            Volume::Millilitre(_) => Volume::Millilitre(value),
            Volume::Centilitre(_) => Volume::Centilitre(value),
            Volume::Decilitre(_) => Volume::Decilitre(value),
            Volume::Litre(_) => Volume::Litre(value),
            Volume::MetricTeaspoon(_) => Volume::MetricTeaspoon(value),
            Volume::MetricTablespoon(_) => Volume::MetricTablespoon(value),
            Volume::MetricDessertspoon(_) => Volume::MetricDessertspoon(value),
            Volume::MetricCup(_) => Volume::MetricCup(value),
            Volume::AustralianTeaspoon(_) => Volume::AustralianTeaspoon(value),
            Volume::AustralianDessertspoon(_) => Volume::AustralianDessertspoon(value),
            Volume::AustralianTablespoon(_) => Volume::AustralianTablespoon(value),
            Volume::AustralianCup(_) => Volume::AustralianCup(value),
            Volume::ImperialTeaspoon(_) => Volume::ImperialTeaspoon(value),
            Volume::ImperialDessertspoon(_) => Volume::ImperialDessertspoon(value),
            Volume::ImperialTablespoon(_) => Volume::ImperialTablespoon(value),
            Volume::ImperialFluidOunce(_) => Volume::ImperialFluidOunce(value),
            Volume::ImperialGill(_) => Volume::ImperialGill(value),
            Volume::ImperialCup(_) => Volume::ImperialCup(value),
            Volume::ImperialPint(_) => Volume::ImperialPint(value),
            Volume::ImperialQuart(_) => Volume::ImperialQuart(value),
            Volume::ImperialGallon(_) => Volume::ImperialGallon(value),
            Volume::USTeaspoon(_) => Volume::USTeaspoon(value),
            Volume::USTablespoon(_) => Volume::USTablespoon(value),
            Volume::USFluidOunce(_) => Volume::USFluidOunce(value),
            Volume::USCup(_) => Volume::USCup(value),
            Volume::USPint(_) => Volume::USPint(value),
            Volume::USQuart(_) => Volume::USQuart(value),
            Volume::USGallon(_) => Volume::USGallon(value),
            Volume::Jigger(_) => Volume::Jigger(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_volumes() -> Vec<Volume> {
        vec![
            // Metric
            Volume::Millilitre(100.0),
            Volume::Centilitre(10.0),
            Volume::Decilitre(1.0),
            Volume::Litre(0.1),
            Volume::MetricTeaspoon(20.0),
            Volume::MetricTablespoon(6.67),
            Volume::MetricDessertspoon(10.0),
            Volume::MetricCup(0.4),
            // Australian
            Volume::AustralianTeaspoon(5.0),
            Volume::AustralianDessertspoon(10.0),
            Volume::AustralianTablespoon(20.0),
            Volume::AustralianCup(250.0),
            // Imperial
            Volume::ImperialTeaspoon(16.91),
            Volume::ImperialDessertspoon(8.45),
            Volume::ImperialTablespoon(5.63),
            Volume::ImperialFluidOunce(3.52),
            Volume::ImperialGill(0.88),
            Volume::ImperialCup(0.44),
            Volume::ImperialPint(0.18),
            Volume::ImperialQuart(0.09),
            Volume::ImperialGallon(0.022),
            // US
            Volume::USTeaspoon(20.29),
            Volume::USTablespoon(6.76),
            Volume::USFluidOunce(3.38),
            Volume::USCup(0.42),
            Volume::USPint(0.21),
            Volume::USQuart(0.11),
            Volume::USGallon(0.026),
            // Other
            Volume::Jigger(2.25),
        ]
    }

    fn all_volumes_with_value(value: f64) -> Vec<Volume> {
        vec![
            Volume::Millilitre(value),
            Volume::Centilitre(value),
            Volume::Decilitre(value),
            Volume::Litre(value),
            Volume::MetricTeaspoon(value),
            Volume::MetricTablespoon(value),
            Volume::MetricDessertspoon(value),
            Volume::MetricCup(value),
            Volume::AustralianTeaspoon(value),
            Volume::AustralianDessertspoon(value),
            Volume::AustralianTablespoon(value),
            Volume::AustralianCup(value),
            Volume::ImperialTeaspoon(value),
            Volume::ImperialDessertspoon(value),
            Volume::ImperialTablespoon(value),
            Volume::ImperialFluidOunce(value),
            Volume::ImperialGill(value),
            Volume::ImperialCup(value),
            Volume::ImperialPint(value),
            Volume::ImperialQuart(value),
            Volume::ImperialGallon(value),
            Volume::USTeaspoon(value),
            Volume::USTablespoon(value),
            Volume::USFluidOunce(value),
            Volume::USCup(value),
            Volume::USPint(value),
            Volume::USQuart(value),
            Volume::USGallon(value),
            Volume::Jigger(value),
        ]
    }

    #[test]
    fn test_abbrev() {
        use Volume::*;

        let test_cases = vec![
            (Millilitre(1.0), "ml"),
            (Centilitre(1.0), "cl"),
            (Decilitre(1.0), "dl"),
            (Litre(1.0), "l"),
            (MetricTeaspoon(1.0), "tsp"),
            (AustralianTeaspoon(1.0), "tsp"),
            (ImperialTeaspoon(1.0), "tsp"),
            (USTeaspoon(1.0), "tsp"),
            (MetricTablespoon(1.0), "tbsp"),
            (AustralianTablespoon(1.0), "tbsp"),
            (ImperialTablespoon(1.0), "tbsp"),
            (USTablespoon(1.0), "tbsp"),
            (MetricDessertspoon(1.0), "dsp"),
            (AustralianDessertspoon(1.0), "dsp"),
            (ImperialDessertspoon(1.0), "dsp"),
            (MetricCup(1.0), "cup"),
            (AustralianCup(1.0), "cup"),
            (ImperialCup(1.0), "cup"),
            (USCup(1.0), "cup"),
            (ImperialFluidOunce(1.0), "fl oz"),
            (USFluidOunce(1.0), "fl oz"),
            (ImperialGill(1.0), "gill"),
            (ImperialPint(1.0), "pt"),
            (USPint(1.0), "pt"),
            (ImperialQuart(1.0), "qt"),
            (USQuart(1.0), "qt"),
            (ImperialGallon(1.0), "gal"),
            (USGallon(1.0), "gal"),
            (Jigger(1.0), "jig"),
        ];

        for (volume, expected) in test_cases {
            assert_eq!(
                volume.abbrev(),
                expected,
                "Failed for volume variant: {:?}",
                volume
            );
        }
    }

    #[test]
    fn test_volume_construction_and_value() {
        let vol = Volume::Millilitre(250.0);
        assert_eq!(vol.value(), 250.0);

        let vol2 = Volume::USCup(1.5);
        assert_eq!(vol2.value(), 1.5);

        let vol3 = Volume::ImperialPint(0.5);
        assert_eq!(vol3.value(), 0.5);
    }

    #[test]
    fn test_value_method_all_types() {
        let test_value = 42.0;
        let volumes = all_volumes_with_value(test_value);

        for volume in volumes {
            assert_eq!(volume.value(), test_value);
        }
    }

    #[test]
    fn test_with_value_method_all_types() {
        let original_value = 10.0;
        let new_value = 25.0;

        let volumes = all_volumes_with_value(original_value);

        for volume in volumes {
            let new_volume = volume.with_value(new_value);
            assert_eq!(new_volume.value(), new_value);
            assert_eq!(volume.value(), original_value);
        }
    }

    #[test]
    fn test_unit_type_method() {
        assert_eq!(
            Volume::Millilitre(100.0).unit_type(),
            UnitType::Volume(VolumeUnit::Millilitre)
        );

        assert_eq!(
            Volume::USCup(2.0).unit_type(),
            UnitType::Volume(VolumeUnit::USCup)
        );

        assert_eq!(
            Volume::ImperialGallon(1.0).unit_type(),
            UnitType::Volume(VolumeUnit::ImperialGallon)
        );

        assert_eq!(
            Volume::Jigger(3.0).unit_type(),
            UnitType::Volume(VolumeUnit::Jigger)
        );
    }

    #[test]
    fn test_unit_type_all_variants() {
        let test_cases = vec![
            (Volume::Millilitre(1.0), VolumeUnit::Millilitre),
            (Volume::Centilitre(1.0), VolumeUnit::Centilitre),
            (Volume::Decilitre(1.0), VolumeUnit::Decilitre),
            (Volume::Litre(1.0), VolumeUnit::Litre),
            (Volume::MetricTeaspoon(1.0), VolumeUnit::MetricTeaspoon),
            (Volume::MetricTablespoon(1.0), VolumeUnit::MetricTablespoon),
            (
                Volume::MetricDessertspoon(1.0),
                VolumeUnit::MetricDessertSpoon,
            ),
            (Volume::MetricCup(1.0), VolumeUnit::MetricCup),
            (
                Volume::AustralianTeaspoon(1.0),
                VolumeUnit::AustralianTeaspoon,
            ),
            (
                Volume::AustralianDessertspoon(1.0),
                VolumeUnit::AustralianDessertspoon,
            ),
            (
                Volume::AustralianTablespoon(1.0),
                VolumeUnit::AustralianTablespoon,
            ),
            (Volume::AustralianCup(1.0), VolumeUnit::AustralianCup),
            (Volume::ImperialTeaspoon(1.0), VolumeUnit::ImperialTeaspoon),
            (
                Volume::ImperialDessertspoon(1.0),
                VolumeUnit::ImperialDessertspoon,
            ),
            (
                Volume::ImperialTablespoon(1.0),
                VolumeUnit::ImperialTablespoon,
            ),
            (
                Volume::ImperialFluidOunce(1.0),
                VolumeUnit::ImperialFluidOunce,
            ),
            (Volume::ImperialGill(1.0), VolumeUnit::ImperialGill),
            (Volume::ImperialCup(1.0), VolumeUnit::ImperialCup),
            (Volume::ImperialPint(1.0), VolumeUnit::ImperialPint),
            (Volume::ImperialQuart(1.0), VolumeUnit::ImperialQuart),
            (Volume::ImperialGallon(1.0), VolumeUnit::ImperialGallon),
            (Volume::USTeaspoon(1.0), VolumeUnit::USTeaspoon),
            (Volume::USTablespoon(1.0), VolumeUnit::USTablespoon),
            (Volume::USFluidOunce(1.0), VolumeUnit::USFluidOunce),
            (Volume::USCup(1.0), VolumeUnit::USCup),
            (Volume::USPint(1.0), VolumeUnit::USPint),
            (Volume::USQuart(1.0), VolumeUnit::USQuart),
            (Volume::USGallon(1.0), VolumeUnit::USGallon),
            (Volume::Jigger(1.0), VolumeUnit::Jigger),
        ];

        for (volume, expected_unit) in test_cases {
            assert_eq!(volume.unit_type(), UnitType::Volume(expected_unit));
        }
    }

    #[test]
    fn test_clone() {
        let original = Volume::Litre(2.5);
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.value(), cloned.value());

        let modified = cloned.with_value(5.0);
        assert_eq!(original.value(), 2.5);
        assert_eq!(modified.value(), 5.0);
    }

    #[test]
    fn test_partial_eq() {
        let vol1 = Volume::Millilitre(500.0);
        let vol2 = Volume::Millilitre(500.0);
        let vol3 = Volume::Millilitre(250.0);
        let vol4 = Volume::Litre(0.5);

        assert_eq!(vol1, vol2);
        assert_ne!(vol1, vol3);
        assert_ne!(vol1, vol4);
    }

    #[test]
    fn test_debug_output() {
        let vol = Volume::USCup(1.5);
        let debug_str = format!("{:?}", vol);
        assert!(debug_str.contains("USCup"));
        assert!(debug_str.contains("1.5"));
    }

    #[test]
    fn test_special_float_values() {
        let zero_vol = Volume::Litre(0.0);
        assert_eq!(zero_vol.value(), 0.0);

        let negative_vol = Volume::Millilitre(-100.0);
        assert_eq!(negative_vol.value(), -100.0);

        let large_vol = Volume::Millilitre(f64::MAX / 2.0);
        assert_eq!(large_vol.value(), f64::MAX / 2.0);

        let small_vol = Volume::Millilitre(f64::MIN_POSITIVE);
        assert_eq!(small_vol.value(), f64::MIN_POSITIVE);
    }

    #[test]
    fn test_with_value_preserves_unit_type() {
        let volumes = create_test_volumes();

        for volume in volumes {
            let original_unit_type = volume.unit_type();
            let new_volume = volume.with_value(999.0);

            assert_eq!(new_volume.unit_type(), original_unit_type);
            assert_eq!(new_volume.value(), 999.0);
        }
    }

    #[test]
    fn test_volume_unit_consistency() {
        let volumes = create_test_volumes();

        for volume in volumes {
            match volume.unit_type() {
                UnitType::Volume(_) => {
                    assert!(true);
                }
                _ => {
                    panic!("Volume should always return UnitType::Volume");
                }
            }
        }
    }

    // Test VolumeUnit Debug and PartialEq
    #[test]
    fn test_volume_unit_traits() {
        let unit1 = VolumeUnit::Millilitre;
        let unit2 = VolumeUnit::Millilitre;
        let unit3 = VolumeUnit::Litre;

        assert_eq!(unit1, unit2);
        assert_ne!(unit1, unit3);

        let debug_str = format!("{:?}", unit1);
        assert!(debug_str.contains("Millilitre"));
    }

    #[test]
    fn test_unit_operations_integration() {
        let vol = Volume::USFluidOunce(8.0);

        let unit_type = vol.unit_type();
        let value = vol.value();
        let new_vol = vol.with_value(value * 2.0);

        assert_eq!(unit_type, UnitType::Volume(VolumeUnit::USFluidOunce));
        assert_eq!(value, 8.0);
        assert_eq!(new_vol.value(), 16.0);
        assert_eq!(new_vol.unit_type(), unit_type);
    }

    #[test]
    fn test_mathematical_operations() {
        let vol1 = Volume::Millilitre(100.0);
        let vol2 = vol1.with_value(vol1.value() * 2.0);
        let vol3 = vol1.with_value(vol1.value() + 50.0);
        let vol4 = vol1.with_value(vol1.value() / 4.0);

        assert_eq!(vol2.value(), 200.0);
        assert_eq!(vol3.value(), 150.0);
        assert_eq!(vol4.value(), 25.0);
        assert_eq!(vol1.unit_type(), vol2.unit_type());
        assert_eq!(vol1.unit_type(), vol3.unit_type());
        assert_eq!(vol1.unit_type(), vol4.unit_type());
    }

    #[test]
    fn test_floating_point_precision() {
        let vol = Volume::Litre(1.0);
        let vol_third = vol.with_value(vol.value() / 3.0);
        let vol_reconstructed = vol_third.with_value(vol_third.value() * 3.0);

        assert!((vol_reconstructed.value() - 1.0).abs() < f64::EPSILON * 10.0);
    }
}
