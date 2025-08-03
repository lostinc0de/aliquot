use std::cmp::{Eq, PartialOrd};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Trait with contraints for unsigned numbers used to compute aliquot sequences.
pub trait Number
where
    Self: Sized
        + Copy
        + Display
        + Debug
        + Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + SubAssign
        + Mul<Output = Self>
        + MulAssign
        + Div<Output = Self>
        + DivAssign
        + Eq
        + PartialOrd
        + Hash,
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MAX: Self;
}

macro_rules! impl_number {
    ($Type: ty) => {
        impl Number for $Type {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TWO: Self = 2;
            const MAX: Self = <$Type>::MAX;
        }
    };
}

impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(u128);
