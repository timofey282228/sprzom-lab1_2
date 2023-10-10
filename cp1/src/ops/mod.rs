use std::ops::{Add, Sub, Mul, Div, Rem};
use super::*;

impl Add<&UnsignedLongInt> for &UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn add(self, rhs: &UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::add(self, rhs)
    }
}

impl Add<UnsignedLongInt> for UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn add(self, rhs: UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::add(&self, &rhs)
    }
}

impl Add<&UnsignedLongInt> for UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn add(self, rhs: &UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::add(&self, rhs)
    }
}

impl Add<UnsignedLongInt> for &UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn add(self, rhs: UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::add(self, &rhs)
    }
}

impl Sub<UnsignedLongInt> for UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn sub(self, rhs: UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::sub(&self, &rhs)
    }
}

impl Sub<&UnsignedLongInt> for &UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn sub(self, rhs: &UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::sub(self, rhs)
    }
}

impl Sub<&UnsignedLongInt> for UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn sub(self, rhs: &UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::sub(&self, rhs)
    }
}

impl Sub<UnsignedLongInt> for &UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn sub(self, rhs: UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::sub(self, &rhs)
    }
}

impl Mul<UnsignedLongInt> for UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn mul(self, rhs: Self) -> Self::Output { UnsignedLongInt::mul(&self, &rhs) }
}

impl Mul<&UnsignedLongInt> for &UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn mul(self, rhs: &UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::mul(self, rhs)
    }
}

impl Mul<&UnsignedLongInt> for UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn mul(self, rhs: &UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::mul(&self, rhs)
    }
}

impl Mul<UnsignedLongInt> for &UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn mul(self, rhs: UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::mul(self, &rhs)
    }
}


impl Div<UnsignedLongInt> for UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn div(self, rhs: Self) -> Self::Output {
        UnsignedLongInt::div(&self, &rhs).0
    }
}

impl Div<&UnsignedLongInt> for &UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn div(self, rhs: &UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::div(self, rhs).0
    }
}

impl Div<&UnsignedLongInt> for UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn div(self, rhs: &UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::div(&self, rhs).0
    }
}

impl Div<UnsignedLongInt> for &UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn div(self, rhs: UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::div(self, &rhs).0
    }
}

impl Rem<UnsignedLongInt> for UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn rem(self, rhs: Self) -> Self::Output {
        UnsignedLongInt::div(&self, &rhs).1
    }
}

impl Rem<&UnsignedLongInt> for &UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn rem(self, rhs: &UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::div(self, rhs).1
    }
}

impl Rem<&UnsignedLongInt> for UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn rem(self, rhs: &UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::div(&self, rhs).1
    }
}

impl Rem<UnsignedLongInt> for &UnsignedLongInt {
    type Output = UnsignedLongInt;

    fn rem(self, rhs: UnsignedLongInt) -> Self::Output {
        UnsignedLongInt::div(self, &rhs).1
    }
}