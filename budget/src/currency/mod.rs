mod euro;
pub use euro::{
    Euro,
};
use crate::interpreter::parse::*;
use std::ops::{
    Mul,
    Add,
    Sub,
    Neg,
    AddAssign,
    SubAssign,
    MulAssign,
};

use std::fmt::{
    Display,
};
pub type Value = f32;

pub trait Quantity
    : Add + AddAssign +
      Sub + SubAssign +
      Mul + MulAssign +
      Neg<Output=Self> +
      Ord + PartialOrd +
      From<Units> +
      Clone + Sized {
    fn amount(&self) -> Units;
    fn zero() -> Units {
        0
    }
}

pub trait Currency : Quantity + Display {
    fn unit_value() -> Value;
    fn value(&self) -> Value {
        Self::unit_value() * self.amount() as Value
    }
}

