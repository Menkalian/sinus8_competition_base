use std::io::{Result, Write};
use std::path::Path;
use std::fs::File;
use std::env;
use std::i32::MAX;

fn main() {
    generate_lut().unwrap();
}

fn sinus8_int(inp: f64) -> i32 {
    // Sig. -> 1 byte
    // Exp. -> 11 byte
    // Mantisse -> 52 byte
    let be_bytes = inp.to_be_bytes();
    let mut converted: i32 = 0;

    let mut exponent: i32 = 0;
    exponent += (be_bytes[0] & 0x7f) as i32;
    exponent << 4 ;
    exponent += (be_bytes[1] >> 4) as i32;
    eprintln!("{}", exponent);

    // set top 31 bit of mantisse
    converted = converted << 4; // 5
    converted += (be_bytes[1] & 0x0f) as i32;
    converted = converted << 8; //13
    converted += (be_bytes[2]) as i32;
    converted = converted << 8; //21
    converted += (be_bytes[3]) as i32;
    converted = converted << 8; //29
    converted += (be_bytes[4]) as i32;
    converted = converted << 3; // 32
    converted += (be_bytes[5] >> 5) as i32;

    converted = converted >> (77 - exponent);

    // set signum
    converted += ((be_bytes[0] >> 7) as i32) << 31;
    eprintln!("{} -> {:?}", inp, converted);

    converted
}

fn generate_lut() -> Result<()>{
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lut_sin.rs");
    let mut f = File::create(&dest_path).unwrap();

    write!(f, "const SIN_LUT: [i32; 256] = [\n")?;
    for i in 0..256 {
        let x = (i as f64).sin();
        write!(f, "    {}i32,\n", (x * i32::MAX as f64) as i32)?;
    }
    write!(f, "];\n")?;

    Ok(())
}