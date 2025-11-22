use crate::Result;
use crate::cooking::units::mass::units::Mass;
use crate::cooking::units::traits::{UnitOperations, UnitScaler};
use crate::cooking::units::unit::Unit;

impl UnitScaler for Mass {
    fn scale(&self, factor: f64) -> Result<Unit> {
        let scaled_value = self.value() * factor;

        match self {
            Mass::Milligram(_) => {
                if scaled_value >= 1e6 {
                    Ok(Unit::Mass(Mass::Kilogram(scaled_value * 1e-6)))
                } else if scaled_value >= 1e5 {
                    Ok(Unit::Mass(Mass::Hectogram(scaled_value * 1e-5)))
                } else if scaled_value >= 1e4 {
                    Ok(Unit::Mass(Mass::Dekagram(scaled_value * 1e-4)))
                } else if scaled_value >= 1e3 {
                    Ok(Unit::Mass(Mass::Gram(scaled_value * 1e-3)))
                } else {
                    Ok(Unit::Mass(Mass::Milligram(scaled_value)))
                }
            }
            Mass::Gram(_) => {
                if scaled_value >= 1e3 {
                    Ok(Unit::Mass(Mass::Kilogram(scaled_value * 1e-3)))
                } else if scaled_value >= 1e2 {
                    Ok(Unit::Mass(Mass::Hectogram(scaled_value * 1e-2)))
                } else if scaled_value >= 1e1 {
                    Ok(Unit::Mass(Mass::Dekagram(scaled_value * 1e-1)))
                } else if scaled_value >= 1.0 {
                    Ok(Unit::Mass(Mass::Gram(scaled_value)))
                } else {
                    Ok(Unit::Mass(Mass::Milligram(scaled_value * 10.0)))
                }
            }
            Mass::Dekagram(_) => {
                if scaled_value >= 1e2 {
                    Ok(Unit::Mass(Mass::Kilogram(scaled_value * 1e-2)))
                } else if scaled_value >= 1e1 {
                    Ok(Unit::Mass(Mass::Hectogram(scaled_value * 1e-1)))
                } else if scaled_value >= 1.0 {
                    Ok(Unit::Mass(Mass::Dekagram(scaled_value)))
                } else if scaled_value >= 1e-1 {
                    Ok(Unit::Mass(Mass::Gram(scaled_value * 1e1)))
                } else {
                    Ok(Unit::Mass(Mass::Milligram(scaled_value * 1e4)))
                }
            }
            Mass::Hectogram(_) => {
                if scaled_value >= 1e1 {
                    Ok(Unit::Mass(Mass::Kilogram(scaled_value * 1e-1)))
                } else if scaled_value >= 1.0 {
                    Ok(Unit::Mass(Mass::Hectogram(scaled_value)))
                } else if scaled_value >= 1e-1 {
                    Ok(Unit::Mass(Mass::Dekagram(scaled_value * 1e1)))
                } else if scaled_value >= 1e-2 {
                    Ok(Unit::Mass(Mass::Gram(scaled_value * 1e2)))
                } else {
                    Ok(Unit::Mass(Mass::Milligram(scaled_value * 1e5)))
                }
            }
            Mass::Kilogram(_) => {
                if scaled_value >= 1.0 {
                    Ok(Unit::Mass(Mass::Kilogram(scaled_value)))
                } else if scaled_value >= 1e-1 {
                    Ok(Unit::Mass(Mass::Hectogram(scaled_value * 1e1)))
                } else if scaled_value >= 1e-2 {
                    Ok(Unit::Mass(Mass::Dekagram(scaled_value * 1e2)))
                } else if scaled_value >= 1e-3 {
                    Ok(Unit::Mass(Mass::Gram(scaled_value * 1e3)))
                } else {
                    Ok(Unit::Mass(Mass::Milligram(scaled_value * 1e6)))
                }
            }
            Mass::Ounce(_) => {
                if scaled_value >= 16.0 {
                    Ok(Unit::Mass(Mass::Pound(scaled_value * 0.0625)))
                } else {
                    Ok(Unit::Mass(Mass::Ounce(scaled_value)))
                }
            }
            Mass::Pound(_) => {
                if scaled_value >= 1.0 {
                    Ok(Unit::Mass(Mass::Pound(scaled_value)))
                } else {
                    Ok(Unit::Mass(Mass::Ounce(scaled_value * 16.0)))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Milligram
    #[test]
    fn test_mg_to_mg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Milligram(10.0)).scale(2.0)?,
            Unit::Mass(Mass::Milligram(20.0))
        );
        Ok(())
    }

    #[test]
    fn test_mg_to_g() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Milligram(100.0)).scale(10.0)?,
            Unit::Mass(Mass::Gram(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_mg_to_dag() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Milligram(1000.0)).scale(10.0)?,
            Unit::Mass(Mass::Dekagram(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_mg_to_hg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Milligram(10_000.0)).scale(10.0)?,
            Unit::Mass(Mass::Hectogram(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_mg_to_kg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Milligram(100_000.0)).scale(20.0)?,
            Unit::Mass(Mass::Kilogram(2.0))
        );
        Ok(())
    }

    // Gram
    #[test]
    fn test_g_to_mg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Gram(1.0)).scale(0.5)?,
            Unit::Mass(Mass::Milligram(5.0))
        );
        Ok(())
    }

    #[test]
    fn test_g_to_g() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Gram(1.0)).scale(8.0)?,
            Unit::Mass(Mass::Gram(8.0))
        );
        Ok(())
    }

    #[test]
    fn test_g_to_dag() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Gram(15.0)).scale(2.0)?,
            Unit::Mass(Mass::Dekagram(3.0))
        );
        Ok(())
    }

    #[test]
    fn test_g_to_hg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Gram(10.0)).scale(10.0)?,
            Unit::Mass(Mass::Hectogram(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_g_to_kg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Gram(100.0)).scale(20.0)?,
            Unit::Mass(Mass::Kilogram(2.0))
        );
        Ok(())
    }

    // Dekagram
    #[test]
    fn test_dag_to_mg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Dekagram(1.0)).scale(0.005)?,
            Unit::Mass(Mass::Milligram(50.0))
        );
        Ok(())
    }

    #[test]
    fn test_dag_to_g() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Dekagram(1.0)).scale(0.5)?,
            Unit::Mass(Mass::Gram(5.0))
        );
        Ok(())
    }

    #[test]
    fn test_dag_to_dag() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Dekagram(3.0)).scale(2.0)?,
            Unit::Mass(Mass::Dekagram(6.0))
        );
        Ok(())
    }

    #[test]
    fn test_dag_to_hg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Dekagram(10.0)).scale(3.0)?,
            Unit::Mass(Mass::Hectogram(3.0))
        );
        Ok(())
    }

    #[test]
    fn test_dag_to_kg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Dekagram(100.0)).scale(20.0)?,
            Unit::Mass(Mass::Kilogram(20.0))
        );
        Ok(())
    }

    // Hectogram
    #[test]
    fn test_hg_to_mg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Hectogram(1.0)).scale(0.0002)?,
            Unit::Mass(Mass::Milligram(20.0))
        );
        Ok(())
    }

    #[test]
    fn test_hg_to_g() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Hectogram(1.0)).scale(0.02)?,
            Unit::Mass(Mass::Gram(2.0))
        );
        Ok(())
    }

    #[test]
    fn test_hg_to_dag() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Hectogram(1.0)).scale(0.1)?,
            Unit::Mass(Mass::Dekagram(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_hg_to_hg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Hectogram(1.0)).scale(2.0)?,
            Unit::Mass(Mass::Hectogram(2.0))
        );
        Ok(())
    }

    #[test]
    fn test_hg_to_kg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Hectogram(10.0)).scale(20.0)?,
            Unit::Mass(Mass::Kilogram(20.0))
        );
        Ok(())
    }

    // Kilogram
    #[test]
    fn test_kg_to_mg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Kilogram(1.0)).scale(0.000002)?,
            Unit::Mass(Mass::Milligram(2.0))
        );
        Ok(())
    }

    #[test]
    fn test_kg_to_g() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Kilogram(1.0)).scale(0.002)?,
            Unit::Mass(Mass::Gram(2.0))
        );
        Ok(())
    }

    #[test]
    fn test_kg_to_dag() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Kilogram(1.0)).scale(0.02)?,
            Unit::Mass(Mass::Dekagram(2.0))
        );
        Ok(())
    }

    #[test]
    fn test_kg_to_hg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Kilogram(1.0)).scale(0.2)?,
            Unit::Mass(Mass::Hectogram(2.0))
        );
        Ok(())
    }

    #[test]
    fn test_kg_to_kg() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Kilogram(100.0)).scale(20.0)?,
            Unit::Mass(Mass::Kilogram(2000.0))
        );
        Ok(())
    }

    // Ounce
    #[test]
    fn test_oz_to_oz() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Ounce(1.0)).scale(2.0)?,
            Unit::Mass(Mass::Ounce(2.0))
        );
        Ok(())
    }

    #[test]
    fn test_oz_to_lb() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Ounce(4.0)).scale(4.0)?,
            Unit::Mass(Mass::Pound(1.0))
        );
        Ok(())
    }

    // Pound
    #[test]
    fn test_lb_to_oz() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Pound(1.0)).scale(0.5)?,
            Unit::Mass(Mass::Ounce(8.0))
        );
        Ok(())
    }

    #[test]
    fn test_lb_to_lb() -> crate::Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Pound(4.0)).scale(4.0)?,
            Unit::Mass(Mass::Pound(16.0))
        );
        Ok(())
    }
}
