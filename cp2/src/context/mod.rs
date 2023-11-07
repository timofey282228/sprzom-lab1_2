use vl_big_ints::UnsignedLongInt;
use crate::{barret_reduction, BarretParameters};
use crate::auxiliary::BitOps;

pub mod ops;

pub struct ModuloUint<'c> {
    val: UnsignedLongInt,
    context: &'c ModuloContext<'c>,
}

impl ModuloUint<'_>{
    pub fn context(&self) -> &ModuloContext{
        self.context
    }
}

impl Into<UnsignedLongInt> for ModuloUint<'_> {
    fn into(self) -> UnsignedLongInt {
        self.val
    }
}

impl BitOps for ModuloUint<'_> {
    fn get_bit(&self, i: usize) -> bool {
        self.val.get_bit(i)
    }
    fn get_highest_set_bit(&self) -> Option<usize> {
        self.val.get_highest_set_bit()
    }
}

pub struct ModuloContext<'m> {
    modulo: &'m UnsignedLongInt,
    bp: BarretParameters<'m>,
}


impl<'m> ModuloContext<'m> {
    // TODO how do you make sure the numbers were taken by the same modulo???
    /// constucts new context for modular calculations
    pub fn new(modulo: &'m UnsignedLongInt) -> Self {
        Self {
            modulo,
            bp: BarretParameters::new(modulo, modulo.num_digits()),
        }
    }

    pub fn get_modulo(&self) -> &UnsignedLongInt{
        self.modulo
    }
    pub fn add(&'m self, a: &ModuloUint, b: &ModuloUint) -> ModuloUint<'m> {
        ModuloUint {
            val: barret_reduction(&(&a.val + &b.val), &self.bp),
            context: self,
        }
    }

    pub fn mul(&'m self, a: &ModuloUint, b: &ModuloUint) -> ModuloUint<'m> {
        ModuloUint {
            val: barret_reduction(&(&a.val * &b.val), &self.bp),
            context: self,
        }
    }

    pub fn sub(&'m self, a: &ModuloUint, b: &ModuloUint) -> ModuloUint<'m> {
        if b.val < a.val {
            return ModuloUint {
                val: &a.val - &b.val,
                context: self,
            };
        } else {
            return ModuloUint {
                val: self.modulo - (&b.val - &a.val),
                context: self,
            };
        }
    }

    /// returns `a mod self.modulo`
    pub fn modulo(&'m self, a: &UnsignedLongInt) -> ModuloUint<'m> {
        if a < &self.modulo.pow(&UnsignedLongInt::from(2)) {
            ModuloUint { val: barret_reduction(a, &self.bp), context: self }
        } else {
            ModuloUint {
                val: UnsignedLongInt::div(a, self.modulo).1,
                context: self,
            }
        }
    }

    pub fn pow<T: BitOps>(&self, a: &ModuloUint, b: &T) -> ModuloUint {
        let mut c = UnsignedLongInt::from(1);
        let mut a = a.val.to_owned();

        for i in 0..(b.get_highest_set_bit().expect("must not be 0 at this point") + 1) {
            if b.get_bit(i) {
                c = barret_reduction(&(&c * &a), &self.bp);
            }
            a = barret_reduction(&(&a * &a), &self.bp);
        }

        ModuloUint {
            val: c,
            context: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn mod_add() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("4b11e668e24d513ec96654975ce04ca09f3eeea20501df88edba0eedce6814a2562fb8fe4f6ef5588aa704c65892813d151c1766922556238687d7c54f15c0c6")?;
        let b = UnsignedLongInt::from_str("3eac59a64ad86a1e503329e28f48b12ba3677c4fe0171efd46749ec57387f1353ff5ff9c1a640df60811d70c202950c4c7f9c566807f17e0fca271456f001125")?;
        let m = UnsignedLongInt::from_str("463563730008fbffd4a9214247f6142f3c87912ab199e8a0c6e08e6c1454b96a")?;

        let mc = ModuloContext::new(&m);
        let amod = mc.modulo(&a);
        let bmod = mc.modulo(&b);
        let result: UnsignedLongInt = mc.add(&amod, &bmod).into();
        let expected = UnsignedLongInt::from_str("38147867842076323880686886bd472ef6322525fb2a6cdf76e165f12503bd81")?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn mod_sub() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("ea5b5b3ebdb1bfc379e3463138434bfcc1fffd7cb70ca67751271a7a2894784ee7a0b2df68cd23a1e5e01fe37626dc77d4cd7e8d1da5365ca90ed72529f3952f")?;
        let b = UnsignedLongInt::from_str("3eac59a64ad86a1e503329e28f48b12ba3677c4fe0171efd46749ec57387f1353ff5ff9c1a640df60811d70c202950c4c7f9c566807f17e0fca271456f001125")?;
        let m = UnsignedLongInt::from_str("463563730008fbffd4a9214247f6142f3c87912ab199e8a0c6e08e6c1454b96a")?;

        let mc = ModuloContext::new(&m);
        let amod = mc.modulo(&a);
        let bmod = mc.modulo(&b);
        let result: UnsignedLongInt = mc.sub(&amod, &bmod).into();
        let expected = UnsignedLongInt::from_str("413863433f216063621c09177d450e4d7d3f1399797db04673a58f5911909838")?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn mod_mul() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("ea5b5b3ebdb1bfc379e3463138434bfcc1fffd7cb70ca67751271a7a2894784ee7a0b2df68cd23a1e5e01fe37626dc77d4cd7e8d1da5365ca90ed72529f3952f")?;
        let b = UnsignedLongInt::from_str("3eac59a64ad86a1e503329e28f48b12ba3677c4fe0171efd46749ec57387f1353ff5ff9c1a640df60811d70c202950c4c7f9c566807f17e0fca271456f001125")?;
        let m = UnsignedLongInt::from_str("0b8c9870a515714526f4a3731f6b6dda")?;

        let mc = ModuloContext::new(&m);
        let amod = mc.modulo(&a);
        let bmod = mc.modulo(&b);
        let result: UnsignedLongInt = mc.mul(&amod, &bmod).into();
        let expected = UnsignedLongInt::from_str("b4675a959534dc32b523a508301dc8f")?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn mod_pow() -> Result<(), Box<dyn Error>> {
        let a = UnsignedLongInt::from_str("ea5b5b3ebdb1bfc379e3463138434bfcc1fffd7cb70ca67751271a7a2894784ee7a0b2df68cd23a1e5e01fe37626dc77d4cd7e8d1da5365ca90ed72529f3952f")?;
        let b = UnsignedLongInt::from_str("3eac59a64ad86a1e503329e28f48b12ba3677c4fe0171efd46749ec57387f1353ff5ff9c1a640df60811d70c202950c4c7f9c566807f17e0fca271456f001125")?;
        let m = UnsignedLongInt::from_str("0b8c9870a515714526f4a3731f6b6dda")?;

        let mc = ModuloContext::new(&m);
        let amod = mc.modulo(&a);

        let result: UnsignedLongInt = mc.pow(&amod, &b).into();
        let expected = UnsignedLongInt::from_str("24ba44c33c255e78c7ab3d60fe8db81")?;

        assert_eq!(result, expected);

        Ok(())
    }
}
