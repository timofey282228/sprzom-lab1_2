use super::*;

pub trait EvenOdd {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
}

impl EvenOdd for UnsignedLongInt {
    #[inline]
    fn is_even(&self) -> bool {
        !self.get_bit(0)
    }

    #[inline]
    fn is_odd(&self) -> bool {
        self.get_bit(0)
    }
}

pub trait BitOps{
    fn get_bit(&self, i: usize) -> bool;
    fn get_highest_set_bit(&self, ) -> Option<usize>;
}

impl BitOps for UnsignedLongInt{
    fn get_bit(&self, i: usize) -> bool {
        self.get_bit(i)
    }

    fn get_highest_set_bit(&self) -> Option<usize> {
        self.get_highest_set_bit()
    }
}