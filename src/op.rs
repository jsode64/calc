//! # Operator System
//!
//! All operators are defined in a macro at the bottom of the file.
//! To add more, just give the input token string, its kind, and the function.
//! Make sure the last operators in the binary and unary sections end with a `;`, not a `,`.
//!
//! The different kinds are `Op::Bi` and `Op::Un`, representing binary and
//! unary operators respectively.
//! Binary operators take two inputs. For example, adding two numbers.
//! Unary operators take one input. For example, getting the square root of a number.

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::str::FromStr;

/// An operator, unary or binary.
#[derive(Debug, Clone, Copy)]
pub enum Op {
    Un(fn(f64) -> f64),
    Bi(fn(f64, f64) -> f64),
}

macro_rules! make_ops {
    ($($bi_name:expr => $bi_f:expr),* ; $($un_name:expr => $un_f:expr),* $(;)*) => {
        impl FromStr for Op {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($bi_name => Ok(Self::Bi($bi_f)),)*
                    $($un_name => Ok(Self::Un($un_f)),)*
                    _ => Err(()),
                }
            }
        }
    };
}

make_ops!(
    "+" => f64::add,
    "-" => f64::sub,
    "*" => f64::mul,
    "/" => f64::div,
    "%" => f64::rem,

    "root" => |a, b| a.powf(1.0 / b),
    "pow" => f64::powf,
    "log" => f64::log,

    "max" => f64::max,
    "min" => f64::min;

    "neg" => f64::neg,
    "abs" => f64::abs,
    "nabs" => |n| -n.abs(),

    "inv" => |n| 1.0 / n,
    "sqrt" => f64::sqrt,
    "cbrt" => f64::cbrt,

    "exp" => f64::exp,
    "ln" => f64::ln,

    "sin" => f64::sin,
    "cos" => f64::cos,
    "tan" => f64::tan,
    "asin" => f64::asin,
    "acos" => f64::acos,
    "atan" => f64::atan,

    "sind" => |n| n.to_radians().sin(),
    "cosd" => |n| n.to_radians().cos(),
    "tand" => |n| n.to_radians().tan(),
    "asind" => |n| n.to_radians().asin(),
    "acosd" => |n| n.to_radians().acos(),
    "atand" => |n| n.to_radians().atan(),

    "sinh" => f64::sinh,
    "cosh" => f64::cosh,
    "tanh" => f64::tanh,
    "asinh" => f64::asinh,
    "acosh" => f64::acosh,
    "atanh" => f64::atanh;
);
