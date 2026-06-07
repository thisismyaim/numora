use crate::error::Numora;

pub fn evaluate_builtin_symbol(name: &str) -> Result<f64, Numora> {
    match name {
        "PI" | "pi" => Ok(std::f64::consts::PI),

        "E" | "e" => Ok(std::f64::consts::E),

        "TAU" | "tau" => Ok(std::f64::consts::TAU),

        "PHI" | "phi" => Ok(1.618_033_988_749_895),

        "i" => Err(Numora::EvaluationError(
            "Symbol 'i' is reserved for imaginary numbers, but complex numbers are not supported yet"
                .to_string(),
        )),

        _ => Err(Numora::EvaluationError(format!(
            "Unknown symbol '{}'",
            name
        ))),
    }
}

pub fn call_builtin_function(name: &str, values: &[f64]) -> Result<f64, Numora> {
    match name {
        "sumof" => sumof(values),
        "avgof" => avgof(values),
        "minof" => minof(values),
        "maxof" => maxof(values),

        "abs" => one_arg(name, values, |x| Ok(x.abs())),
        "round" => one_arg(name, values, |x| Ok(x.round())),
        "floor" => one_arg(name, values, |x| Ok(x.floor())),
        "ceil" => one_arg(name, values, |x| Ok(x.ceil())),

        "sin" => one_arg(name, values, |x| Ok(x.sin())),
        "cos" => one_arg(name, values, |x| Ok(x.cos())),
        "tan" => one_arg(name, values, |x| Ok(x.tan())),

        "ln" => one_arg(name, values, |x| {
            if x <= 0.0 {
                return Err(Numora::EvaluationError(
                    "ln() needs a number greater than 0".to_string(),
                ));
            }

            Ok(x.ln())
        }),

        "log" => one_arg(name, values, |x| {
            if x <= 0.0 {
                return Err(Numora::EvaluationError(
                    "log() needs a number greater than 0".to_string(),
                ));
            }

            Ok(x.log10())
        }),

        _ => Err(Numora::EvaluationError(format!(
            "Unknown function '{}'",
            name
        ))),
    }
}

fn sumof(values: &[f64]) -> Result<f64, Numora> {
    require_at_least_one("sumof", values)?;
    Ok(values.iter().sum())
}

fn avgof(values: &[f64]) -> Result<f64, Numora> {
    require_at_least_one("avgof", values)?;

    let total: f64 = values.iter().sum();
    let count = values.len() as f64;

    Ok(total / count)
}

fn minof(values: &[f64]) -> Result<f64, Numora> {
    require_at_least_one("minof", values)?;

    let mut smallest = values[0];

    for value in values.iter().skip(1) {
        if *value < smallest {
            smallest = *value;
        }
    }

    Ok(smallest)
}

fn maxof(values: &[f64]) -> Result<f64, Numora> {
    require_at_least_one("maxof", values)?;

    let mut biggest = values[0];

    for value in values.iter().skip(1) {
        if *value > biggest {
            biggest = *value;
        }
    }

    Ok(biggest)
}

fn one_arg<F>(name: &str, values: &[f64], operation: F) -> Result<f64, Numora>
where
    F: Fn(f64) -> Result<f64, Numora>,
{
    if values.len() != 1 {
        return Err(Numora::EvaluationError(format!(
            "{}() needs exactly 1 argument, but got {}",
            name,
            values.len()
        )));
    }

    operation(values[0])
}

fn require_at_least_one(name: &str, values: &[f64]) -> Result<(), Numora> {
    if values.is_empty() {
        return Err(Numora::EvaluationError(format!(
            "{}() needs at least 1 argument",
            name
        )));
    }

    Ok(())
}
