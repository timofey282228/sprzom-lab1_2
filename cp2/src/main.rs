use std::error::Error;
use std::str::FromStr;
use vl_big_ints::UnsignedLongInt;
use vl_big_ints_modulo::*;

fn main() -> Result<(), Box<dyn Error>>{
    let a = UnsignedLongInt::from_str("06c934d551eaa397b356f5d70ebd823dc481b6509a0b480715bf1544d5fd1596545474758b0d40ea5f3f49aff8ba221eb730d51ba2ea506ce30ff70006dfd24c36be216d310bef555856d8eef37936f422e26f58e0ff46511fb98567e8c1c5fc75b5acb5e8455d94500694f7399dab54826ec5ada9f97d7b9deace6d9dec7203")?;
    let m = UnsignedLongInt::from_str("faaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaafffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")?;

    println!("{}", unparametrized_barret(&a, &m));

    Ok(())
}
