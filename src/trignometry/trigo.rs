// A seperate trignometry module to handle all the trigonometric functions and calculations
// This will help in keeping the code organized and modular 

// CONSTANTS

pub const PI: f64 = std::f64::consts::PI;
pub const TAU: f64 = std::f64::consts::TAU; // 2 * PI TO_BE_USED_LATER

// Enums and Structs



#[derive(Debug, Clone, PartialEq)]
pub enum AngleType {
    Degrees,
    Radians,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrigonometricFunction {
    Sine, // Sin
    Cosine, // Cos
    Tangent, // Tan
    Cosecant, // Cosec
    Secant, // Sec
    Cotangent, // Cot
    InverseSine, // Sin^-1 or arcsin
    InverseCosine, // Cos^-1 or arccos
    InverseTangent, // Tan^-1 or arctan
}

fn to_degrees(radians: f64) -> f64 {
    radians * (180.0 / PI)
}

fn to_radians(degrees: f64) -> f64 {
    degrees * (PI / 180.0)
}

// Trigonometric function implementations
pub fn compute_trigo_func(func: &TrigonometricFunction, angle: f64, unit: &AngleType) -> Result<f64, String>{
   let rad = match unit {
        AngleType::Degrees => to_radians(angle),
        AngleType::Radians => angle,  // already in radians, no conversion
    };
    match func {
        TrigonometricFunction::Sine => Ok(rad.sin()),
        TrigonometricFunction::Cosine => Ok(rad.cos()),
        TrigonometricFunction::Tangent  => {
            // tan is undefined at 90, 270, etc.
            let cos = rad.cos();
            if cos.abs() < 1e-10 {
                Err(format!("tan({}) is undefined", angle))
            } else {
                Ok(rad.tan())
            }
        }
        TrigonometricFunction::Cosecant  => {
            let sin = rad.sin();
            if sin.abs() < 1e-10 {
                Err(format!("csc({}) is undefined", angle))
            } else {
                Ok(1.0 / sin)
            }
        }
        TrigonometricFunction::Secant  => {
            let cos = rad.cos();
            if cos.abs() < 1e-10 {
                Err(format!("sec({}) is undefined", angle))
            } else {
                Ok(1.0 / cos)
            }
        }
        TrigonometricFunction::Cotangent  => {
            let sin = rad.sin();
            if sin.abs() < 1e-10 {
                Err(format!("cot({}) is undefined", angle))
            } else {
                Ok(rad.cos() / sin)
            }
        }
        TrigonometricFunction::InverseSine => {
            if angle < -1.0 || angle > 1.0 {
                Err(format!("asin({}) is undefined (input must be in [-1, 1])", angle))
            } else {
                let result = Ok(angle.asin());
                match unit {
                    AngleType::Degrees => Ok(to_degrees(result?)),
                    AngleType::Radians => result,
                }
            }
        }
        TrigonometricFunction::InverseCosine => {
            if angle < -1.0 || angle > 1.0 {
                Err(format!("acos({}) is undefined (input must be in [-1, 1])", angle))
            } else {
                let result = angle.acos();
                match unit {
                    AngleType::Degrees => Ok(to_degrees(result)),
                    AngleType::Radians => Ok(result),
                }
            }
        }
        TrigonometricFunction::InverseTangent =>{ 
            let result = angle.atan();
            match unit {
                AngleType::Degrees => Ok(to_degrees(result)),
                AngleType::Radians => Ok(result),
            }
        },
    }

}

// TO_BE_USED_LATER
pub fn exact_values(func: &TrigonometricFunction, angle_val: f64) -> Option<&'static str> {
    let angle = angle_val as i64;
    match(func, angle) {
        (TrigonometricFunction::Sine, 0) => Some("0"),
        (TrigonometricFunction::Sine, 30) => Some("1/2"),
        (TrigonometricFunction::Sine, 45) => Some("√2/2"),
        (TrigonometricFunction::Sine, 60) => Some("√3/2"),
        (TrigonometricFunction::Sine, 90) => Some("1"),

        (TrigonometricFunction::Cosine, 0) => Some("1"),
        (TrigonometricFunction::Cosine, 30) => Some("√3/2"),
        (TrigonometricFunction::Cosine, 45) => Some("√2/2"),
        (TrigonometricFunction::Cosine, 60) => Some("1/2"),
        (TrigonometricFunction::Cosine, 90) => Some("0"),

        (TrigonometricFunction::Tangent, 0) => Some("0"),
        (TrigonometricFunction::Tangent, 30) => Some("1/√3"),
        (TrigonometricFunction::Tangent, 45) => Some("1"),
        (TrigonometricFunction::Tangent, 60) => Some("√3"),
        _ => None,
    }
}