use std::io::{Result, Write};
use std::path::Path;
use std::fs::File;
use std::env;

fn main() {
    generate_lut().unwrap();
}

fn generate_lut() -> Result<()>{
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lut_sin.rs");
    let mut f = File::create(&dest_path).unwrap();

    write!(f, "const SIN_LUT: [f64; 256] = [\n")?;
    for i in 0..256 {
        write!(f, "    {}f64,\n", (i as f64).sin())?;
    }
    write!(f, "];\n")?;

    Ok(())
}