use crate::error::Numora;
use crate::format::format_number;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitDim {
    pub length: i32,
    pub time: i32,
    pub mass: i32,
}

impl UnitDim {
    pub fn none() -> Self {
        Self {
            length: 0,
            time: 0,
            mass: 0,
        }
    }

    pub fn length() -> Self {
        Self {
            length: 1,
            time: 0,
            mass: 0,
        }
    }

    pub fn time() -> Self {
        Self {
            length: 0,
            time: 1,
            mass: 0,
        }
    }

    pub fn mass() -> Self {
        Self {
            length: 0,
            time: 0,
            mass: 1,
        }
    }

    pub fn is_dimensionless(&self) -> bool {
        self.length == 0 && self.time == 0 && self.mass == 0
    }

    pub fn multiply(self, other: Self) -> Self {
        Self {
            length: self.length + other.length,
            time: self.time + other.time,
            mass: self.mass + other.mass,
        }
    }

    pub fn divide(self, other: Self) -> Self {
        Self {
            length: self.length - other.length,
            time: self.time - other.time,
            mass: self.mass - other.mass,
        }
    }

    pub fn power(self, exponent: i32) -> Self {
        Self {
            length: self.length * exponent,
            time: self.time * exponent,
            mass: self.mass * exponent,
        }
    }

    pub fn sqrt(self) -> Result<Self, Numora> {
        if self.length % 2 != 0 || self.time % 2 != 0 || self.mass % 2 != 0 {
            return Err(Numora::EvaluationError(format!(
                "sqrt() cannot cleanly simplify unit '{}'",
                self.format()
            )));
        }

        Ok(Self {
            length: self.length / 2,
            time: self.time / 2,
            mass: self.mass / 2,
        })
    }

    pub fn format(&self) -> String {
        if self.is_dimensionless() {
            return "".to_string();
        }

        let mut positive = Vec::new();
        let mut negative = Vec::new();

        push_unit_part(&mut positive, &mut negative, "m", self.length);
        push_unit_part(&mut positive, &mut negative, "s", self.time);
        push_unit_part(&mut positive, &mut negative, "kg", self.mass);

        if negative.is_empty() {
            positive.join("*")
        } else {
            format!("{}/{}", positive.join("*"), negative.join("*"))
        }
    }
}

fn push_unit_part(positive: &mut Vec<String>, negative: &mut Vec<String>, name: &str, power: i32) {
    if power == 0 {
        return;
    }

    let abs_power = power.abs();

    let text = if abs_power == 1 {
        name.to_string()
    } else {
        format!("{}^{}", name, abs_power)
    };

    if power > 0 {
        positive.push(text);
    } else {
        negative.push(text);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Value {
    pub number: f64,
    pub dim: UnitDim,
}

impl Value {
    pub fn number(number: f64) -> Self {
        Self {
            number,
            dim: UnitDim::none(),
        }
    }

    pub fn quantity(number: f64, unit_name: &str) -> Result<Self, Numora> {
        let unit = parse_unit(unit_name)?;

        Ok(Self {
            number: number * unit.scale,
            dim: unit.dim,
        })
    }

    pub fn add(self, other: Self) -> Result<Self, Numora> {
        if self.dim != other.dim {
            return Err(Numora::EvaluationError(format!(
                "Cannot add values with different units: '{}' and '{}'",
                self.dim.format(),
                other.dim.format()
            )));
        }

        Ok(Self {
            number: self.number + other.number,
            dim: self.dim,
        })
    }

    pub fn subtract(self, other: Self) -> Result<Self, Numora> {
        if self.dim != other.dim {
            return Err(Numora::EvaluationError(format!(
                "Cannot subtract values with different units: '{}' and '{}'",
                self.dim.format(),
                other.dim.format()
            )));
        }

        Ok(Self {
            number: self.number - other.number,
            dim: self.dim,
        })
    }

    pub fn multiply(self, other: Self) -> Self {
        Self {
            number: self.number * other.number,
            dim: self.dim.multiply(other.dim),
        }
    }

    pub fn divide(self, other: Self) -> Result<Self, Numora> {
        if other.number == 0.0 {
            return Err(Numora::EvaluationError("Cannot divide by zero".to_string()));
        }

        Ok(Self {
            number: self.number / other.number,
            dim: self.dim.divide(other.dim),
        })
    }

    pub fn power(self, other: Self) -> Result<Self, Numora> {
        if !other.dim.is_dimensionless() {
            return Err(Numora::EvaluationError(
                "Power exponent must not have a unit".to_string(),
            ));
        }

        let rounded = other.number.round();

        if (other.number - rounded).abs() > 1e-9 {
            if !self.dim.is_dimensionless() {
                return Err(Numora::EvaluationError(
                    "Fractional powers with units are not supported yet".to_string(),
                ));
            }

            return Ok(Self {
                number: self.number.powf(other.number),
                dim: UnitDim::none(),
            });
        }

        let exponent = rounded as i32;

        Ok(Self {
            number: self.number.powf(other.number),
            dim: self.dim.power(exponent),
        })
    }

    pub fn negative(self) -> Self {
        Self {
            number: -self.number,
            dim: self.dim,
        }
    }

    pub fn is_dimensionless(&self) -> bool {
        self.dim.is_dimensionless()
    }

    pub fn format(&self) -> String {
        let number_text = format_number(self.number);
        let unit_text = self.dim.format();

        if unit_text.is_empty() {
            number_text
        } else {
            format!("{} {}", number_text, unit_text)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct UnitInfo {
    scale: f64,
    dim: UnitDim,
}

fn parse_unit(name: &str) -> Result<UnitInfo, Numora> {
    match name {
        "m" => Ok(UnitInfo {
            scale: 1.0,
            dim: UnitDim::length(),
        }),

        "cm" => Ok(UnitInfo {
            scale: 0.01,
            dim: UnitDim::length(),
        }),

        "km" => Ok(UnitInfo {
            scale: 1_000.0,
            dim: UnitDim::length(),
        }),

        "s" => Ok(UnitInfo {
            scale: 1.0,
            dim: UnitDim::time(),
        }),

        "kg" => Ok(UnitInfo {
            scale: 1.0,
            dim: UnitDim::mass(),
        }),

        "g" => Ok(UnitInfo {
            scale: 0.001,
            dim: UnitDim::mass(),
        }),

        _ => Err(Numora::EvaluationError(format!("Unknown unit '{}'", name))),
    }
}
