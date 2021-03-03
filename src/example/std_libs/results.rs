// Copyright 2021 Ferris Project Authors. License user Apache License.

// Result
// We've seen that the Option enum can be used as a return value from functions that may fail, where
// None can be return indicate failure. However, sometime it is important to express why an operation
// failed. To do this we have the Result enum/

// The Result<T, E> enum has two variants:
// - Ok(value), which indicates that the operation succeeded, and wraps the value returned by the
// operation. (value has type T)
// - Err(why), which indicates that the operation failed, and wraps why, which (hopefully) explains
// the cause of the failure. (why has type E)

mod checked {
    // Mathematical "errors" we want to catch
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositionLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositionLogarithm)
        } else {
            Ok(x.ln())
        }
    }

}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) -> f64 {
    // This is a three level match pyramid!
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            }
        }
    }
}

pub fn result() {
    println!("{}", op(1.0, 1.0));
}
