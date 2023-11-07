use std::error::Error;
use std::ops::Add;
use vl_big_ints::UnsignedLongInt;
use vl_big_ints_modulo::*;
use std::time;
use vl_big_ints_modulo::context::{ModuloContext, ModuloUint};

const NUM_EXPERIMENTS: [usize; 3] = [1000, 10000, 100000];
const OPERAND_BIT_LENGTH: [usize; 3] = [1024, 2048, 4096];

const OUTPUT_CSV: &str = "./report/data.csv";

fn main() -> Result<(), Box<dyn Error>> {
    println!("Performance calculation example");

    let mut wrt = csv::Writer::from_path(OUTPUT_CSV)?;

    for op_len in OPERAND_BIT_LENGTH {
        println!("\nOperand length: {} bits", op_len);

        for experiment_l in NUM_EXPERIMENTS {
            println!("\nExperiment count: {}", experiment_l);
            let num_digits = op_len / u64::BITS as usize;

            let mut op1: Vec<ModuloUint> = Vec::with_capacity(experiment_l);
            let mut op2: Vec<ModuloUint> = Vec::with_capacity(experiment_l);

            let mrandom: Vec<u64> = (0..num_digits).map(|_| { rand::random::<u64>() }).collect();
            let modulo = UnsignedLongInt::from(mrandom.as_slice());

            let mc = ModuloContext::new(&modulo);

            for _ in 0..experiment_l {
                let random_bytes: Vec<u64> = (0..num_digits).map(|_| { rand::random::<u64>() }).collect();
                op1.push(mc.modulo( &UnsignedLongInt::from(random_bytes.as_slice())));
                let random_bytes: Vec<u64> = (0..num_digits).map(|_| { rand::random::<u64>() }).collect();
                op2.push(mc.modulo( &UnsignedLongInt::from(random_bytes.as_slice())));
            }
            println!("Using modulo: {}", mc.get_modulo());
            for op in ["+", "-", "*"] {
                measure(op, experiment_l, &op1, &op2, op_len, &mut wrt)?;
            }
        }
    }

    Ok(())
}

fn measure(op: &str, exps: usize, op1: &Vec<ModuloUint>, op2: &Vec<ModuloUint>, oplen: usize, wrt: &mut csv::Writer<std::fs::File>) -> Result<time::Duration, Box<dyn Error>> {
    println!("Running {exps} experiments on {}-bit bigints; operation: {}", oplen, op);

    let mut durations = Vec::with_capacity(exps);
    for i in 0..exps {
        let duration = match op {
            "+" => { measure_add(&op1[i], &op2[i]) }
            "*" => { measure_mul(&op1[i], &op2[i]) }
            "-" => { measure_sub(&op1[i], &op2[i]) }
            _ => panic!("must be valid op string")
        };

        durations.push(duration);
    }

    let total_duration: time::Duration = durations.iter().sum();

    let average_duration = total_duration / durations.len() as u32;

    println!(" - Average operaition duration: {:?}", average_duration);
    // op oplem n_exp dur
    wrt.write_record([
        op.to_string(),
        format!("{exps}"),
        format!("{oplen}"),
        format!("{}", average_duration.as_nanos()),
    ])?;

    Ok(average_duration)
}

fn measure_add(op1: &ModuloUint, op2: &ModuloUint) -> time::Duration {
    let then = time::Instant::now();
    let _ = op1.context().add(op1, op2);
    let now = time::Instant::now();

    now.duration_since(then)
}

fn measure_mul(op1: &ModuloUint, op2: &ModuloUint) -> time::Duration {
    let then = time::Instant::now();
    let _ = op1.context().mul(op1, op2);
    let now = time::Instant::now();

    now.duration_since(then)
}

fn measure_sub(op1: &ModuloUint, op2: &ModuloUint) -> time::Duration {
    let then = time::Instant::now();
    let _ = op1.context().sub(op1, op2);
    let now = time::Instant::now();

    now.duration_since(then)
}