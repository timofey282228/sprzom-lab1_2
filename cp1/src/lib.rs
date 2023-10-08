struct UnsignedLongInt{
    underlying_array: Vec<u64>
}

impl UnsignedLongInt{
    pub fn new() -> Self{
        UnsignedLongInt{
            underlying_array: vec![0u64]
        }
    }

    pub fn with_capacity(min_length:usize) -> Self{
        let mut new_int = UnsignedLongInt{
            underlying_array: Vec::<u64>::with_capacity(min_length)
        };
        new_int.underlying_array.push(0u64);

        new_int
    }
}