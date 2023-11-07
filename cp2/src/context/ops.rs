use std::ops::*;
use crate::context::ModuloUint;

#[auto_impl_ops::auto_ops]
impl<'s, 'd> Add<ModuloUint<'d>> for ModuloUint<'s> where 'd: 's
{
    type Output = ModuloUint<'s>;
    fn add(self, rhs: ModuloUint<'d>) -> Self::Output {
        self.context.add(&self, &rhs)
    }
}

impl<'s, 'd> Sub<ModuloUint<'d>> for ModuloUint<'s> where 'd: 's
{
    type Output = ModuloUint<'s>;
    fn sub(self, rhs: ModuloUint<'d>) -> Self::Output {
        self.context.sub(&self, &rhs)
    }
}

impl<'s, 'd> Mul<ModuloUint<'d>> for ModuloUint<'s> where 'd: 's
{
    type Output = ModuloUint<'s>;
    fn mul(self, rhs: ModuloUint<'d>) -> Self::Output {
        self.context.mul(&self, &rhs)
    }
}