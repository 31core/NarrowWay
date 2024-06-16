use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const GF28_M: u8 = 0b1110001;

/** Addition on GF(2^8) */
fn gf_add(f: u8, g: u8) -> u8 {
    f ^ g
}

/** Subtration on GF(2^8) */
fn gf_sub(f: u8, g: u8) -> u8 {
    gf_add(f, g)
}

/** Multiple on GF(2^8)
 *
 * Note: m is without x^8 nomial
 */
fn gf_mul(f: u8, g: u8, m: u8) -> u8 {
    let mut cache = Vec::new();
    for i in 0..8 {
        if (g >> i) & 1 == 1 {
            let mut j = f;
            for _ in 0..i {
                /* the highest bit is 1 */
                if j >> 7 == 1 {
                    j <<= 1;
                    j = gf_sub(j, m);
                } else {
                    j <<= 1;
                }
            }
            cache.push(j);
        }
    }

    let mut result = 0;
    for i in cache {
        result = gf_add(result, i);
    }
    result
}

/** Multiple inverse on GF(2^8) */
fn gf_mul_inv(f: u8, m: u8) -> u8 {
    for i in 0..256 {
        if gf_mul(f, i as u8, m) == 1 {
            return i as u8;
        }
    }
    0
}

fn main() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("s0.rs");

    let mut s0 = [0; 256];
    for (i, byte) in s0.iter_mut().enumerate() {
        *byte = gf_mul_inv(i as u8, GF28_M);
    }

    let mut f = File::create(path).unwrap();
    write!(f, "pub const S0: [u8; 256] = {:?};", s0)?;
    println!("cargo:return-if-changed=build.rs");

    Ok(())
}
