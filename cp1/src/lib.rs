use std::ops::{Add, Mul, Sub};
use core::str::FromStr;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct UnsignedLongInt {
    underlying_array: Vec<u64>,
}

impl UnsignedLongInt {
    pub fn new() -> Self {
        UnsignedLongInt {
            underlying_array: vec![0u64]
        }
    }

    pub fn with_capacity(min_length: usize) -> Self {
        let mut new_int = UnsignedLongInt {
            underlying_array: Vec::<u64>::with_capacity(min_length)
        };
        new_int.underlying_array.push(0u64);

        new_int
    }

    fn empty_with_capcity(capacity: usize) -> Self {
        Self {
            underlying_array: Vec::with_capacity(capacity)
        }
    }

    pub fn num_digits(&self) -> usize {
        self.underlying_array.len()
    }

    pub fn add(&self, rhs: &Self) -> Self {
        let (shorter, longer) = if self.underlying_array.len() > rhs.underlying_array.len()
        { (rhs, self) } else { (self, rhs) };

        // assume the best minimum length to minimize reallocations in the future
        let mut result = Self::empty_with_capcity(shorter.num_digits());

        let mut carry: bool = false;
        let mut current_digit = 0usize;

        while current_digit < shorter.underlying_array.len() {
            let mut new_digit: u64 = shorter.underlying_array[current_digit];
            let mut temp_carry1: bool = false; // carry from carry addition
            let mut temp_carry2: bool = false; // carry from `longer` digit addition

            // add carry from the previous operation
            if carry {
                (new_digit, temp_carry1) = new_digit.overflowing_add(1);
            }

            // add digit of the other number
            (new_digit, temp_carry2) = new_digit.overflowing_add(longer.underlying_array[current_digit]);

            // carry on next operation?
            carry = temp_carry1 | temp_carry2;

            result.underlying_array.push(new_digit);

            current_digit += 1;
        }

        for current_digit in current_digit..longer.underlying_array.len() {
            let mut new_digit: u64 = if carry { 1 } else { 0 };


            // add digit of the other number
            (new_digit, carry) = new_digit.overflowing_add(longer.underlying_array[current_digit]);

            result.underlying_array.push(new_digit);
        }

        // new digit? (relative to `shorter` length)
        if carry {
            result.underlying_array.push(1);
        }

        result
    }

    fn sub(&self, rhs: &Self) -> Self {
        const OVERFLOW_PANIC: &str = "Subtraction with overflow";
        let (subtrahend, minuend) = (rhs, self);

        // if minuend is shorter than subtrahend, then it's obviously smaller
        if minuend.underlying_array.len() < subtrahend.underlying_array.len() {
            panic!("{}", OVERFLOW_PANIC);
        }

        // assume the best minimum length to minimize reallocations in the future
        let mut result = Self::empty_with_capcity(minuend.num_digits());

        let mut borrow: bool = false;
        let mut current_digit = 0usize;

        while current_digit < subtrahend.underlying_array.len() {
            let mut new_digit: u64 = minuend.underlying_array[current_digit];
            let mut temp_borrow1: bool = false;
            let mut temp_borrow2: bool = false;

            if borrow {
                (new_digit, temp_borrow1) = new_digit.overflowing_sub(1);
            }

            (new_digit, temp_borrow2) = new_digit.overflowing_sub(subtrahend.underlying_array[current_digit]);
            borrow = temp_borrow1 | temp_borrow2;
            result.underlying_array.push(new_digit);
            current_digit += 1;
        }

        for current_digit in current_digit..minuend.underlying_array.len() {
            let mut new_digit: u64 = minuend.underlying_array[current_digit];

            (new_digit, borrow) = new_digit.overflowing_sub(if borrow { 1 } else { 0 });
            result.underlying_array.push(new_digit);
        }

        if borrow {
            panic!("{}", OVERFLOW_PANIC);
        }

        // Truncate zeroes
        let mut new_len = result.underlying_array.len();
        while new_len > 1 {
            if result.underlying_array[new_len - 1] > 0 {
                break;
            }
            new_len -= 1;
        }

        result.underlying_array.truncate(new_len);
        result.underlying_array.shrink_to_fit();
        result
    }

    pub fn mul_single_digit(&self, rhs: u64) -> Self {
        let mut result = UnsignedLongInt::empty_with_capcity(self.underlying_array.len() + 1);
        let mut carry = 0u128;
        let b = rhs as u128;

        for i in 0..self.underlying_array.len() {
            let a = self.underlying_array[i] as u128;
            let temp: u128 = a * b + carry;
            result.underlying_array.push((temp & 63u128) as u64);
            carry = temp >> 6;
        }

        if carry != 0 {
            result.underlying_array.push(carry as u64);
        }

        result
    }
    fn mul(&self, rhs: &Self) -> Self {
        let mut result = UnsignedLongInt::with_capacity(self.underlying_array.len());
        let (shorter, longer) = if self.underlying_array.len() > rhs.underlying_array.len()
        { (rhs, self) } else { (self, rhs) };

        for i in 0..shorter.underlying_array.len() {
            let mut temp = longer.mul_single_digit(shorter.underlying_array[i]);
            // shift
            for j in 0..i {
                temp.underlying_array.insert(0, 0);
            }
            result = UnsignedLongInt::add(&result, &temp);
        }

        result
    }
}


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

impl From<u64> for UnsignedLongInt {
    fn from(value: u64) -> Self {
        UnsignedLongInt {
            underlying_array: vec![value]
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FromHexError;


impl Display for FromHexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Conversion from hex string resulted in an error")
    }
}

impl std::error::Error for FromHexError {}

impl FromStr for UnsignedLongInt {
    type Err = FromHexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = UnsignedLongInt::empty_with_capcity(s.len() / 16);
        let mut digit = 0usize;
        let mut digit_fill = 0u32;
        for c in s.chars().rev() {
            if let Some(d) = c.to_digit(16) {
                let d = d as u64;
                if result.underlying_array.len() == digit {
                    result.underlying_array.push(d * 16u64.pow(digit_fill));
                } else {
                    result.underlying_array[digit] += d * 16u64.pow(digit_fill);
                }
                digit_fill += 1;
                if digit_fill % 16 == 0 {
                    digit_fill = 0;
                    digit += 1;
                }
            } else {
                return Err(FromHexError);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_1() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("DEADBEEFDEADBEEFDEADBEEF")?;
        let b = UnsignedLongInt::from(u64::MAX);
        let c = UnsignedLongInt::from_str("AABBCCDD")?;
        assert_eq!((&a + &b) * &c, &a * &c + &b * &c);

        Ok(())
    }
}