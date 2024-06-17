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

fn bit_transform(b: u8) -> u8 {
    let mut bit_array = [0; 8];

    for (i, bit) in bit_array.iter_mut().enumerate() {
        *bit = (b >> i) & 1;
    }

    let mut result = [0; 8];
    for i in 0..8 {
        result[i] = bit_array[i]
            ^ bit_array[(i + 2) % 8]
            ^ bit_array[(i + 4) % 8]
            ^ bit_array[(i + 6) % 8]
            ^ bit_array[(i + 7) % 8];
    }

    let mut b = 0;
    for (i, bit) in result.iter().enumerate() {
        b |= *bit << i;
    }

    b
}

fn main() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("constant.rs");

    let mut s0 = [0; 256];
    for (i, byte) in s0.iter_mut().enumerate() {
        *byte = bit_transform(gf_mul_inv(i as u8, GF28_M));
    }

    let mut gf28_inv = [0; 256];
    for (i, byte) in gf28_inv.iter_mut().enumerate() {
        *byte = gf_mul_inv(i as u8, GF28_M);
    }

    let mut gf28_table = [[0; 256]; 256];
    for (f, row) in gf28_table.iter_mut().enumerate() {
        for (g, item) in row.iter_mut().enumerate() {
            *item = gf_mul(f as u8, g as u8, GF28_M);
        }
    }

    let mut f = File::create(path).unwrap();
    writeln!(f, "pub const S0: [u8; 256] = {:?};", s0)?;
    writeln!(f, "pub const GF28_INV: [u8; 256] = {:?};", gf28_inv)?;
    writeln!(
        f,
        "pub const GF28_TABLE: [[u8; 256]; 256] = {:?};",
        gf28_table
    )?;
    println!("cargo:return-if-changed=build.rs");

    Ok(())
}
