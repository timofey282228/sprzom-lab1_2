use core::str::FromStr;
use std::cmp::{PartialEq, PartialOrd, Eq, Ord, Ordering};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod ops;

#[derive(Debug, Eq, Clone)]
pub struct UnsignedLongInt {
    underlying_array: Vec<u64>,
}

impl Display for UnsignedLongInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hex_string = String::new();

        // special handling of the most significant digit for 0x prefix and zeroes truncation
        if let Some(last) = self.underlying_array.last() {
            let next_hexdigits = format!("{:#0X}", last);
            hex_string.push_str(&next_hexdigits);
        }

        for i in (0..self.underlying_array.len() - 1).rev() {
            let next_hexdigits = format!("{:016X}", self.underlying_array[i]);
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
            // see comments for `fn checked_sub`, same applies here
            let (mut new_digit, other_carry) = longer.underlying_array[current_digit]
                .overflowing_add(shorter.underlying_array[current_digit]);

            (new_digit, carry) = new_digit.overflowing_add(if carry {1} else {0});

            carry |= other_carry;

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
            // we will need a separate borrow for two subtractions - the digit
            // and the borrow from previous sub
            let (mut new_digit, other_borrow) = minuend.underlying_array[current_digit]
                .overflowing_sub(subtrahend.underlying_array[current_digit]);

            // we can never be sure that the new digit is not zero, consider 121 - 22
            (new_digit, borrow) = new_digit.overflowing_sub(if borrow { 1 } else { 0 });

            // we borrow from the mores significant digit either
            // when we subtract the next digit of the minuend
            // or when we subtract the previous borrow
            borrow |= other_borrow;

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

    pub fn sub(&self, rhs: &Self) -> Self {
        const OVERFLOW_PANIC: &str = "Subtraction with overflow";
        if let Some(result) = Self::checked_sub(self, rhs) {
            result
        } else {
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
    pub fn mul(&self, rhs: &Self) -> Self {
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

    pub fn shl_digits(&self, n: usize) -> Self {
        let mut result = UnsignedLongInt::from(self);
        (0..n).for_each(|_| { result.underlying_array.insert(0, 0) });
        result
    }

    pub fn shl(&self, n: usize) -> Self {
        let mut result = Self::empty_with_capcity(self.underlying_array.len());
        let digit_shift = n / u64::BITS as usize;
        let n: u32 = (n % u64::BITS as usize) as u32;

        let mut carryout: u64 = 0;
        for i in 0..self.underlying_array.len() {
            result.underlying_array.push(
                (self.underlying_array[i] << n)
                    | carryout
            );
            carryout = u64::checked_shr(self.underlying_array[i], u64::BITS - n).unwrap_or(0);
        }

        if carryout != 0 {
            result.underlying_array.push(carryout);
        }

        result = result.shl_digits(digit_shift);

        result
    }

    pub fn shr(&self, rhs: &Self) -> Self {
        UnsignedLongInt::div(self, &UnsignedLongInt::from(2).pow(rhs)).0
    }

    pub fn get_allocated_bit_length(&self) -> usize {
        self.underlying_array.len() * (u64::BITS as usize)
    }

    pub fn get_highest_set_bit(&self) -> Option<usize> {
        const HIGHEST_BIT_OF_U64: u64 = 1 << (u64::BITS - 1);

        if self == &UnsignedLongInt::from(0) {
            return None;
        }
        let allocated_bits = self.get_allocated_bit_length();

        let mut i = allocated_bits - 1; // this function is not expected to be used on "empty"/uninitialized bigints
        if let Some(last) = self.underlying_array.last() {
            let mut d = last.to_owned();
            while i > 0 && d & HIGHEST_BIT_OF_U64 == 0 {
                d <<= 1;
                i -= 1;
            }
        }

        Some(i)
    }

    pub fn set_bit(&mut self, n: usize) {
        // extend the underlying array if needed
        if n >= self.underlying_array.len() * u64::BITS as usize {
            (self.underlying_array.len()..(n / (u64::BITS as usize) + 1)).for_each(|_| { self.underlying_array.push(0) })
        }

        self.underlying_array[n / (u64::BITS as usize)] |= 1 << (n % (u64::BITS as usize));
    }

    pub fn get_bit(&self, n: usize) -> bool {
        let digit_size = u64::BITS as usize;
        self.underlying_array[n / digit_size] & (1 << (n % digit_size)) != 0
    }

    pub fn div(&self, rhs: &Self) -> (Self, Self) {
        if rhs == &UnsignedLongInt::from(0) {
            panic!("division by zero");
        }
        if self == &UnsignedLongInt::from(0) {
            return (UnsignedLongInt::from(0), UnsignedLongInt::from(0));
        }

        let b: UnsignedLongInt = rhs.clone();
        let k = b.get_highest_set_bit().expect("must be non-0 at this point") + 1;
        let mut r = self.clone();
        let mut q = UnsignedLongInt::from(0);
        while r >= b {
            let mut t = r.get_highest_set_bit().expect("must be non-0 at this point") + 1;
            let mut c = b.shl(t - k);
            if r < c {
                t -= 1;
                c = b.shl(t - k);
            }
            r = r - c;
            q.set_bit(t - k)
        }

        (q, r)
    }

    pub fn pow(&self, rhs: &Self) -> Self {
        if rhs == &UnsignedLongInt::from(0) {
            return UnsignedLongInt::from(1);
        }

        let mut result = UnsignedLongInt::from(1);
        for i in (0..rhs.get_highest_set_bit().expect("must not be 0 at this point") + 1).rev() {
            if rhs.get_bit(i) {
                result = &result * self;
            }
            if i != 0 {
                result = &result * &result;
            }
        }

        result
    }
}

impl Default for UnsignedLongInt {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&Self> for UnsignedLongInt {
    fn from(value: &Self) -> Self {
        UnsignedLongInt {
            underlying_array: value.underlying_array.clone()
        }
    }
}

impl From<u64> for UnsignedLongInt {
    fn from(value: u64) -> Self {
        UnsignedLongInt {
            underlying_array: vec![value]
        }
    }
}

impl From<&[u64]> for UnsignedLongInt {
    /// Constructs UnsignedLongInt from a little-endian slice of u64's. Higher-order zeroes are truncated.
    fn from(value: &[u64]) -> Self {
        let mut result = UnsignedLongInt { underlying_array: Vec::from(value) };

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

impl PartialEq for UnsignedLongInt {
    fn eq(&self, other: &Self) -> bool {
        self.underlying_array == other.underlying_array
    }
}

impl PartialOrd for UnsignedLongInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(differenece) = UnsignedLongInt::checked_sub(self, other) {
            if differenece.underlying_array.len() == 1 && differenece.underlying_array[0] == 0 {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Ord for UnsignedLongInt {
    fn cmp(&self, other: &Self) -> Ordering {
        UnsignedLongInt::partial_cmp(self, other).expect("implementation ensures strict total ordering")
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::FromBytesUntilNulError;
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
    fn equality_2() -> Result<(), Box<dyn Error>> {
        const COUNT: u64 = 4096;
        let a = UnsignedLongInt::from_str("DEADBEEFDEADBEEFDEADBEEF")?;
        let n = UnsignedLongInt::from(COUNT);

        let mut c = UnsignedLongInt::from(0);
        for _ in 0..COUNT {
            c = &c + &a;
        }

        assert_eq!(&n * &a, c);

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
        let c = &a - &b;
        let expected = UnsignedLongInt::from_str("deadbe4410bdc01112ffe112")?;
        println!("A: {}\nB: {}\nC: {}\nShould be: {}", &a, &b, &c, &expected);
        assert_eq!(c, expected);

        Ok(())
    }

    #[test]
    fn ordering_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("deadbeefdeadbeefdeadbeef")?;
        let b = UnsignedLongInt::from_str("abcdeffedecbaddddd")?;

        assert!(&a > &b);
        assert!(&a >= &b);
        assert_eq!(&a, &a);
        assert!(&a > &(&a - &b));
        assert_ne!(a + b, UnsignedLongInt::from(0));

        Ok(())
    }

    #[test]
    fn get_highest_set_bit_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("deadbeefdeadbeefdeadbeef")?;
        let b = UnsignedLongInt::from_str("abcdeffedecbaddddd")?;

        assert_eq!(a.get_highest_set_bit(), Some(95));
        assert_eq!(b.get_highest_set_bit(), Some(71));
        assert_eq!(UnsignedLongInt::from(1).get_highest_set_bit(), Some(0));
        assert_eq!(UnsignedLongInt::from(0).get_highest_set_bit(), None);

        Ok(())
    }

    #[test]
    fn shl_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("deadbeefdeadbeefdeadbeef")?;
        let b = UnsignedLongInt::from_str("abcdeffedecbaddddd")?;

        assert_eq!(a.shl_digits(3), UnsignedLongInt::from_str("deadbeefdeadbeefdeadbeef000000000000000000000000000000000000000000000000")?);
        assert_eq!(a.shl(13), UnsignedLongInt::from_str("1bd5b7ddfbd5b7ddfbd5b7dde000")?);
        assert_eq!(b.shl(17), UnsignedLongInt::from_str("1579bdffdbd975bbbba0000")?);

        Ok(())
    }

    #[test]
    fn shr_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("123123123123123")?;
        let b = UnsignedLongInt::from(0x34);

        assert_eq!(a.shr(&b), UnsignedLongInt::from(18));
        Ok(())
    }

    #[test]
    fn div_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("deadbeefdeadbeefdeadbeef")?;
        let b = UnsignedLongInt::from_str("abcdeffedecbaddddd")?;

        let expected_quotient = UnsignedLongInt::from(0x14bce56);
        let expected_remainder = UnsignedLongInt::from_str("3306e57e63acfe60b1")?;

        assert_eq!(UnsignedLongInt::div(&a, &b), (expected_quotient, expected_remainder));

        Ok(())
    }

    #[test]
    fn get_bit_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from(0x10);
        let b = UnsignedLongInt::from(0xffeeff);
        let c = UnsignedLongInt::from_str("80000000000000000")?;

        dbg!(&a);
        assert!(a.get_bit(4));
        assert!(!b.get_bit(8));
        assert!(c.get_bit(67));
        assert!(!c.get_bit(66));
        assert!(!c.get_bit(0));

        Ok(())
    }

    #[test]
    fn pow_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("10000000000000000")?;
        let b = UnsignedLongInt::from_str("a")?;

        let c = UnsignedLongInt::pow(&a, &b);
        assert_eq!(c, UnsignedLongInt::from_str("10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000")?);

        Ok(())
    }

    #[test]
    fn sub2_test() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("100000000000000000000000000000001")?;
        let b = UnsignedLongInt::from_str("10000000000000002")?;

        let expected = UnsignedLongInt::from_str("FFFFFFFFFFFFFFFEFFFFFFFFFFFFFFFF")?;

        assert_eq!(&a - &b, expected);
        Ok(())
    }
}