use core::str::FromStr;
use std::cmp::{PartialEq, PartialOrd, Eq, Ord, Ordering};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod ops;

#[derive(Debug, Eq)]
pub struct UnsignedLongInt {
    underlying_array: Vec<u64>,
}

impl Display for UnsignedLongInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hex_string = String::new();
        for i in (0..self.underlying_array.len()).rev() {
            let next_hexdigits = format!("{:X}", self.underlying_array[i]);
            hex_string.push_str(&next_hexdigits);
        }

        write!(f, "{}", &hex_string)
    }
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
            let temp_carry2: bool; // carry from `longer` digit addition

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

    pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        let (subtrahend, minuend) = (rhs, self);

        // if minuend is shorter than subtrahend, then it's obviously smaller
        if minuend.underlying_array.len() < subtrahend.underlying_array.len() {
            return None;
        }

        // assume the best minimum length to minimize reallocations in the future
        let mut result = Self::empty_with_capcity(minuend.num_digits());

        let mut borrow: bool = false;
        let mut current_digit = 0usize;

        while current_digit < subtrahend.underlying_array.len() {
            let mut new_digit: u64 = minuend.underlying_array[current_digit];
            let mut temp_borrow1: bool = false;
            let temp_borrow2: bool;

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
            return None;
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
        Some(result)
    }

    pub fn sub(&self, rhs:&Self) -> Self{
        const OVERFLOW_PANIC: &str = "Subtraction with overflow";
        if let Some(result) = Self::checked_sub(self, rhs){
            return result
        }
        else{
            panic!("{}", OVERFLOW_PANIC);
        }
    }

    pub fn mul_single_digit(&self, rhs: u64) -> Self {
        let mut result = UnsignedLongInt::empty_with_capcity(self.underlying_array.len() + 1);
        let mut carry = 0u128;
        let b = rhs as u128;

        for i in 0..self.underlying_array.len() {
            let a = self.underlying_array[i] as u128;
            let temp: u128 = a * b + carry;
            result.underlying_array.push((temp & (u64::MAX as u128)) as u64);
            carry = temp >> 64;
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
            for _ in 0..i {
                temp.underlying_array.insert(0, 0);
            }
            result = UnsignedLongInt::add(&result, &temp);
        }

        result
    }

    fn shl(&mut self, n: usize){
        (0..n).for_each(|_|{self.underlying_array.insert(0,0)});
    }

    fn div(&self, rhs: &Self) -> (Self, Self){
        let k = self.num_digits();
        let r = self;
        let q = Self::new();
        while r >= rhs{

        }
        todo!();
    }
}

impl Default for UnsignedLongInt {
    fn default() -> Self {
        Self::new()
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
        write!(f, "Conversion from hex string resulted in an error")
    }
}

impl Error for FromHexError {}

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

impl PartialEq for UnsignedLongInt{
    fn eq(&self, other: &Self) -> bool {
        self.underlying_array == other.underlying_array
    }
}

impl PartialOrd for UnsignedLongInt{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       if let Some(differenece) = UnsignedLongInt::checked_sub(self, other){
           if differenece.underlying_array.len() == 1 && differenece.underlying_array[0] == 0{
               return Some(Ordering::Equal);
           }else{
               return Some(Ordering::Greater);
           }
       }
        else{
            return Some(Ordering::Less);
        }
    }
}

impl Ord for UnsignedLongInt{
    fn cmp(&self, other: &Self) -> Ordering {
        UnsignedLongInt::partial_cmp(self, other).expect("implementation ensures strict total ordering")
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
        println!("{}", (&a + &b) * &c);
        println!("{}", (&a * &c) + (&b * &c));
        assert_eq!((&a + &b) * &c, (&a * &c) + (&b * &c));

        Ok(())
    }

    #[test]
    fn mul_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("DEADBEEFDEADBEEFDEADBEEF")?;
        let b = UnsignedLongInt::from(u64::MAX);
        let expected = UnsignedLongInt::from_str("deadbeefdeadbeeeffffffff2152411021524111")?;
        println!("A: {}", &a);
        println!("B: {}", &b);
        println!("C: {}", &a * &b);
        println!("Should be: {}", &expected);
        assert_eq!(&a * &b, expected);

        Ok(())
    }

    #[test]
    fn add_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("DEADBEEFDEADBEEFDEADBEEF")?;
        let b = UnsignedLongInt::from_str("abcdeffedecbaddddd")?;
        let c = &a + &b;
        let expected = UnsignedLongInt::from_str("deadbf9bac9dbdceaa5b9ccc")?;
        println!("A: {}\nB: {}\nC: {}\nShould be: {}", &a, &b, &c, &expected);
        assert_eq!(c, expected);

        Ok(())
    }

    #[test]
    fn mul_single_digit_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("deadbeefdeadbeefdeadbeef")?;
        let b = 0xffff;
        let c = UnsignedLongInt::mul_single_digit(&a, b);
        let expected = UnsignedLongInt::from_str("deace0421fbde0421fbde0414111")?;
        println!("A: {}\nB: {}\nC: {}\nShould be: {}", &a, &b, &c, &expected);
        assert_eq!(c, expected);

        Ok(())
    }

    #[test]
    fn sub_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("deadbeefdeadbeefdeadbeef")?;
        let b = UnsignedLongInt::from_str("abcdeffedecbaddddd")?;
        let c =  &a - &b;
        let expected = UnsignedLongInt::from_str("deadbe4410bdc01112ffe112")?;
        println!("A: {}\nB: {}\nC: {}\nShould be: {}", &a, &b, &c, &expected);
        assert_eq!(c, expected);

        Ok(())
    }

    #[test]
    fn ordering_test()-> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("deadbeefdeadbeefdeadbeef")?;
        let b = UnsignedLongInt::from_str("abcdeffedecbaddddd")?;

        assert!(&a > &b);
        assert!(&a >= &b);
        assert_eq!(&a, &a);
        assert!(&a > &(&a-&b));
        assert_ne!(a+b, UnsignedLongInt::from(0));

        Ok(())


    }
}