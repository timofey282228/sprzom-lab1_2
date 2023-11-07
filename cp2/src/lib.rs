pub use vl_big_ints::{UnsignedLongInt};
mod auxiliary;
use auxiliary::*;

pub mod context;

struct BarretParameters<'m> {
    modulo: &'m UnsignedLongInt,
    k: usize,
    mu: UnsignedLongInt,
}

impl<'m> BarretParameters<'m> {
    pub fn new(modulo: &'m UnsignedLongInt, k: usize) -> Self {
        let mu = Self::calculate_mu(k as u64, &modulo);
        Self {
            modulo,
            k,
            mu,
        }
    }

    pub fn get_val(&self) -> &UnsignedLongInt {
        &self.modulo
    }
    pub fn get_mu(&self) -> &UnsignedLongInt {
        &self.mu
    }

    /// calculate `mu` for given modulo and 2k - digit operand
    fn calculate_mu(k: u64, m: &UnsignedLongInt) -> UnsignedLongInt {
        vl_big_ints::BASE.pow(&UnsignedLongInt::from(k * 2)) / m
    }
}

pub fn modulo(a: &UnsignedLongInt, m: &UnsignedLongInt) -> UnsignedLongInt{
    UnsignedLongInt::div(&a, &m).1
}

pub fn gcd(a: &UnsignedLongInt, b: &UnsignedLongInt) -> UnsignedLongInt {
    let mut d = UnsignedLongInt::from(1);
    let mut a = a.to_owned();
    let mut b = b.to_owned();
    let const_1 = UnsignedLongInt::from(1);
    let const_0 = UnsignedLongInt::from(0);
    while a.is_even() && b.is_even() {
        a = a.shr(&const_1);
        b = b.shr(&const_1);
        d = d.shl(1);
    }

    while a.is_even() {
        a = a.shr(&const_1);
    }


    while a != const_0 {
        while b.is_even() {
            b = b.shr(&const_1);
        }

        if &a < &b {
            b = &b - &a;
        } else {
            b = &a - &b;
            std::mem::swap(&mut a, &mut b);
        }
    }

    d = d * b;

    d
}

pub fn lcm(a: &UnsignedLongInt, b: &UnsignedLongInt) -> UnsignedLongInt {
    (a * b) / gcd(a, b)
}

fn barret_reduction<'m>(x: &UnsignedLongInt, m: &'m BarretParameters) -> UnsignedLongInt {
    let mu = &m.mu;
    let n = m.modulo;
    let k = m.k;

    let mut q = x.shr_digits(k - 1);
    q = q * mu;
    q = q.shr_digits(k + 1);
    let mut r = x - q * n;
    loop {
        if let Some(rr) = r.checked_sub(n) {
            r = rr;
        } else {
            break;
        }
    }

    r
}

pub fn add_mod(a: &UnsignedLongInt, b: &UnsignedLongInt, m: &UnsignedLongInt) -> UnsignedLongInt {
    modulo(&(a+b), m)
}

pub fn sub_mod(a: &UnsignedLongInt, b: &UnsignedLongInt, m: &UnsignedLongInt) -> UnsignedLongInt {
    let a = modulo(a, m);
    let b = modulo(b, m);

    if b < a{
        return modulo(&(a-b), m)
    }else{
        return m - modulo(&(b-a), m)
    }
}

pub fn mul_mod(a: &UnsignedLongInt, b: &UnsignedLongInt, m: &UnsignedLongInt) -> UnsignedLongInt {
    modulo(&(modulo(a, m) * modulo(b, m)), m)
}

pub fn square_mod(a: &UnsignedLongInt, m: &UnsignedLongInt) -> UnsignedLongInt {
    power_mod_barret(a, &UnsignedLongInt::from(2), m)
}

pub fn power_mod_barret(a: &UnsignedLongInt, b: &UnsignedLongInt, m: &UnsignedLongInt) -> UnsignedLongInt {
    if b == &UnsignedLongInt::from(0) {
        return UnsignedLongInt::from(1);
    }
    if a == &UnsignedLongInt::from(1){
        return UnsignedLongInt::from(1);
    }

    let mut a = modulo(&a, &m);
    let bp = BarretParameters::new(m, (a.num_digits() + 1) / 2);
    let mut c = UnsignedLongInt::from(1);

    for i in 0..(b.get_highest_set_bit().expect("must not be 0 at this point") + 1) {
        if b.get_bit(i) {
            c = barret_reduction(&(&c * &a), &bp);
        }
        a = barret_reduction(&(&a * &a), &bp)
    }

    c
}

#[cfg(test)]
mod tests;