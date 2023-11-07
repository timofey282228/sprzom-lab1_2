use super::*;
use std::error::Error;
use std::str::FromStr;
use crate::context::{ModuloContext, ModuloUint};

#[test]
fn gcd1_test() -> Result<(), Box<dyn Error>> {
    let a = UnsignedLongInt::from_str("332620a94e016efe4880db1392fef137ac4ed9c5449395a7d55bc40328e20e77")?;
    let b = UnsignedLongInt::from_str("71df139f99653b654069b68d5c693c2c35d7740f1b2d2331766be1e5cd483a3a")?;

    let result = gcd(&a, &b);
    let expected = UnsignedLongInt::from(1);

    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn gcd2_test() -> Result<(), Box<dyn Error>> {
    let a = UnsignedLongInt::from_str("62d972e5a1c8f8a1ddb169e740bb706698eefa9efa30b65df1de1556047afc4")?;
    let b = UnsignedLongInt::from_str("88f1979cc28247ede75fda269e4b892a")? / UnsignedLongInt::from(2);

    let result = lcm(&a, &b);
    let expected = UnsignedLongInt::from_str("62d972e5a1c8f8a1ddb169e740bb706698eefa9efa30b65df1de1556047afc4")?;

    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn lcm_test() -> Result<(), Box<dyn Error>> {
    let a = UnsignedLongInt::from_str("62d972e5a1c8f8a1ddb169e740bb706698eefa9efa30b65df1de1556047afc4")?;
    let b = UnsignedLongInt::from_str("88f1979cc28247ede75fda269e4b892a")? / UnsignedLongInt::from(2);

    let result = gcd(&a, &b);
    let expected = UnsignedLongInt::from_str("4478cbce614123f6f3afed134f25c495")?;

    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn barret_reduction_test() -> Result<(), Box<dyn Error>> {
    let a = UnsignedLongInt::from_str("c0bb89c50cf9de8295a86586d798b31d365543b97dbf4242d546e086d6effbb6f53a884341b9f7999a8b160eb82d33962091dbdaab1d550f8f56ea8e42a253638ea8fb2ac8b7a564278e700b7610f96570f034ee5bfc8e6ef5de9a0b49696b841064c39f735ec412a327f91dae88bc3cb4af1971c7d82cd9082c01e4fea5abc1")?;
    let m = &UnsignedLongInt::from_str("0d18c7d0c18e96748e8d051cbe70f7588ada152df907fe942cb18b502836e999fd37e4ffe221006f556af44b0d5fcf566b71c7a335c741860c18a5ba417b4522")?;
    let bp = BarretParameters::new(&m, a.num_digits() / 2);

    let expected = UnsignedLongInt::from_str("424ad1096ff8a8c447f7b330e652dc71e0da3ea5f39228a34eb753d8eacc4678c8bd58f7c0ff27e4b0a2b25b7bccd6843e7c6218ab86359900cfec6ffd15455")?;

    assert_eq!(barret_reduction(&a, &bp), expected);

    Ok(())
}

#[test]
fn power_mod_barret_test() -> Result<(), Box<dyn Error>> {
    let a = UnsignedLongInt::from_str("aaabbbccbacbfbfbcbacdefbcdfbe")?;
    let b = UnsignedLongInt::from_str("aaabbcdb")?;
    let m = UnsignedLongInt::from_str("abbbbb")?;

    let expected = UnsignedLongInt::from_str("3cada8")?;

    assert_eq!(power_mod_barret(&a, &b, &m), expected);

    Ok(())
}

#[test]
fn equality_1() -> Result<(), Box<dyn Error>> {
    let m = UnsignedLongInt::from_str("7bf7183")?;
    let mc = ModuloContext::new(&m);
    let a = UnsignedLongInt::from_str("c0bb89c50cf9de8295a86586d798b31d365543b97dbf4242d546e086d6effbb6f53a884341b9f7999a8b160eb82d33962091dbdaab1d550f8f56ea8e42a253638ea8fb2ac8b7a564278e700b7610f96570f034ee5bfc8e6ef5de9a0b49696b841064c39f735ec412a327f91dae88bc3cb4af1971c7d82cd9082c01e4fea5abc1")?;
    let b = UnsignedLongInt::from_str("7c03fdaaad1383ff444c71aed5ba267ed7188ca3054dcba94a5280df5ca21bfd744d8b0fa85f55aedaeb23f3eafb3d88f379dafd0bc5055fc5e6d5f3fc7915de9fce5ede8597564a4821fc336863cd7034c3eccaa6b48a6cc790004c404c9db89799d24eea8e6a8f7a60c9ee77b8d125dc9f5c172a262adf6c0fa9f6309023b5")?;
    let c = UnsignedLongInt::from_str("de2db897f14521c6dd6b31f4eb6e21c3c824571c6fe04aed9818a113454d8728c47e64c3f6352b1398ae5f07be5bb4f7aaac079699896ee52e1ba849b284ec075a6b415d839ff1083f87eaf1cd7035c932c63eb46a14d5a31fa77ef672de63d77647493320490dfc80878b8b685c2529bc53c404f1dda835d164fb9f9c78ec00")?;

    let a_mod_m = mc.modulo(&a);
    let b_mod_m = mc.modulo(&b);
    let c_mod_m = mc.modulo(&c);

    let res1: UnsignedLongInt = mc.mul(&mc.add(&a_mod_m, &b_mod_m), &c_mod_m).into();
    let res2: UnsignedLongInt = mc.mul(&c_mod_m, &mc.add(&a_mod_m, &b_mod_m)).into();
    let res3: UnsignedLongInt = mc.add(&mc.mul(&a_mod_m, &c_mod_m), &mc.mul(&b_mod_m, &c_mod_m)).into();

    assert_eq!(&res1, &res2);
    assert_eq!(&res2, &res3);

    Ok(())
}

#[test]
fn equality_2() -> Result<(), Box<dyn Error>> {
    const N: u64 = 4096;
    let a = UnsignedLongInt::from_str("c61fb27bcf3d643d1d9e26e519f42e322d1aa2e13b812dd5ddea2d7cc2bd8ecc00c0c485b634a3b99cd3c6e5aec41b9f0b24bd45fdde6e6b19b2e91082ec40acafefddcbfdf67690a327824924cef12f8688d15a0b05a04b646502486432ec8bf63836c1206a491c0aa30047d1c3edf6d5d7a5ff1ef877de4d237bbb223b879c")?;
    let n = UnsignedLongInt::from(N);
    let m = UnsignedLongInt::from_str("8d1fc443466f05e22ec69316a2c609d2626cfbceabf45b8fe79f1e4ae9808aff")?;

    let mc = ModuloContext::new(&m);
    let amodm = mc.modulo(&a);
    let nmodm = mc.modulo(&n);

    let mult = mc.mul(&amodm, &nmodm);
    let mut summ = mc.modulo(&UnsignedLongInt::from(0));

    for i in 0..N {
        summ = mc.add(&summ, &amodm);
    }

    assert_eq!(<ModuloUint<'_> as Into<UnsignedLongInt>>::into(mult), <ModuloUint<'_> as Into<UnsignedLongInt>>::into(summ));
    Ok(())
}

fn pow2_1(i: u64) -> UnsignedLongInt {
    UnsignedLongInt::from(2).pow(&UnsignedLongInt::from(i)) - UnsignedLongInt::from(1)
}

#[test]
fn equality_3_1() -> Result<(), Box<dyn Error>> {
    let a = UnsignedLongInt::from_str("c61fb27bcf3d643d1d9e26e519f42e322d1aa2e13b812dd5ddea2d7cc2bd8ecc00c0c485b634a3b99cd3c6e5aec41b9f0b24bd45fdde6e6b19b2e91082ec40acafefddcbfdf67690a327824924cef12f8688d15a0b05a04b646502486432ec8bf63836c1206a491c0aa30047d1c3edf6d5d7a5ff1ef877de4d237bbb223b879c")?;
    let mers = pow2_1(31);

    let mc = ModuloContext::new(&mers);
    let amodm = mc.modulo(&a);
    assert!(gcd(&a, &mers) == UnsignedLongInt::from(1));

    let phi = &mers - UnsignedLongInt::from(1);

    assert_eq!(<ModuloUint<'_> as Into<UnsignedLongInt>>::into(mc.pow(&amodm, &phi)), UnsignedLongInt::from(1));

    Ok(())
}

#[test]
fn equality_3_2() -> Result<(), Box<dyn Error>> {
    let a = UnsignedLongInt::from_str("c61fb27bcf3d643d1d9e26e519f42e322d1aa2e13b812dd5ddea2d7cc2bd8ecc00c0c485b634a3b99cd3c6e5aec41b9f0b24bd45fdde6e6b19b2e91082ec40acafefddcbfdf67690a327824924cef12f8688d15a0b05a04b646502486432ec8bf63836c1206a491c0aa30047d1c3edf6d5d7a5ff1ef877de4d237bbb223b8791")?;
    assert!(gcd(&a, &UnsignedLongInt::from(3)) == UnsignedLongInt::from(1));

    let k = UnsignedLongInt::from_str("1c")?;
    let n = UnsignedLongInt::from(3).pow(&k);
    let phi_n = UnsignedLongInt::from(2) * UnsignedLongInt::from(3).pow(&(k - UnsignedLongInt::from(1)));

    let mc = ModuloContext::new(&n);
    let amodm = mc.modulo(&a);


    assert_eq!(<ModuloUint<'_> as Into<UnsignedLongInt>>::into(mc.pow(&amodm, &phi_n)), UnsignedLongInt::from(1));

    Ok(())
}

