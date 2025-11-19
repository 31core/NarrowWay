include!(concat!(env!("OUT_DIR"), "/constant.rs"));

mod exports;
mod nw256;
mod nw384;
mod nw512;

pub use nw256::Cipher256;
pub use nw384::Cipher384;
pub use nw512::Cipher512;

const GF28_M: u8 = 0b1110001;

type SBox = [u8; 256];

#[macro_export]
macro_rules! reverse_col {
    ($mat: expr, $col: expr, $start: expr, $end: expr) => {
        let mut start = $start;
        let mut end = $end;
        while end > start {
            ($mat[start][$col], $mat[end - 1][$col]) = ($mat[end - 1][$col], $mat[start][$col]);
            start += 1;
            end -= 1;
        }
    };
}

/** Addition on GF(2^8) */
#[cfg(feature = "poc")]
fn gf_add(f: u8, g: u8) -> u8 {
    f ^ g
}

/** Subtration on GF(2^8) */
#[cfg(feature = "poc")]
fn gf_sub(f: u8, g: u8) -> u8 {
    gf_add(f, g)
}

/** Multiple on GF(2^8)
 *
 * Note: m is without x^8 nomial
 */
#[cfg(feature = "poc")]
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
#[cfg(not(feature = "poc"))]
fn gf_mul(f: u8, g: u8, _m: u8) -> u8 {
    GF28_TABLE[f as usize][g as usize]
}

/** Multiple inverse on GF(2^8) */
#[cfg(feature = "poc")]
fn gf_mul_inv(f: u8, m: u8) -> u8 {
    for i in 0..256 {
        if gf_mul(f, i as u8, m) == 1 {
            return i as u8;
        }
    }
    0
}

/** Multiple inverse on GF(2^8) */
#[cfg(not(feature = "poc"))]
fn gf_mul_inv(f: u8, _m: u8) -> u8 {
    GF28_INV[f as usize]
}

#[cfg(feature = "poc")]
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

/** Calculate the round constant */
fn rc(i: usize) -> u8 {
    #[cfg(feature = "poc")]
    {
        let mut byte = 2;
        for _ in 0..(i + 2) {
            byte = gf_mul(byte, 2, GF28_M);
        }

        byte
    }
    #[cfg(not(feature = "poc"))]
    {
        RC[i]
    }
}

/** Generate pre-S-Box */
#[cfg(feature = "poc")]
fn s0_gen() -> SBox {
    let mut s_box = [0; 256];
    for (i, byte) in s_box.iter_mut().enumerate() {
        *byte = bit_transform(gf_mul_inv(i as u8, GF28_M));
    }
    s_box
}

/** Generate S-Box */
#[cfg(feature = "poc")]
fn s_box_gen(key: u8) -> SBox {
    let mut s0 = s0_gen();
    for byte in &mut s0 {
        *byte ^= key;
    }
    s0
}

fn digest_key(key: &[u8]) -> u8 {
    use std::cmp::max;
    let mut byte = max(key[0], 1);

    for i in key.iter().skip(1) {
        byte = gf_mul(byte, max(*i, 1), GF28_M);
    }

    byte
}

/** Generate S-Box */
#[cfg(not(feature = "poc"))]
fn s_box_gen(key: u8) -> SBox {
    let mut s0 = S0;
    for byte in &mut s0 {
        *byte ^= key;
    }
    s0
}

/** Generate Inverse S-Box */
fn s_inv_gen(s_box: &SBox) -> SBox {
    let mut s_inv = [0; 256];

    for i in 0..256 {
        s_inv[s_box[i] as usize] = i as u8;
    }

    s_inv
}

fn func_f(p: &mut [u8; 8], key: [u8; 8]) {
    p[1] ^= p[0] ^ p[2];
    p[6] ^= p[5] ^ p[7];

    p[1] = p[1].rotate_left(3);
    p[2] ^= p[4];
    p[6] = p[6].rotate_right(2);

    p[2] = p[2].rotate_left(2);
    p[5] ^= p[3] ^ p[6];

    p[4] = p[4].rotate_right(4);

    p[4] ^= p[1];

    p[3] ^= p[4] ^ p[7];

    p[5] = p[5].rotate_left(1);

    p[0] ^= p[2];
    p[7] ^= p[5];

    for (i, p) in p.iter_mut().enumerate() {
        *p ^= key[i];
    }

    (p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]) =
        (p[4], p[5], p[0], p[1], p[6], p[7], p[2], p[3])
}

fn func_f_inv(p: &mut [u8; 8], key: [u8; 8]) {
    (p[4], p[5], p[0], p[1], p[6], p[7], p[2], p[3]) =
        (p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]);

    for (i, p) in p.iter_mut().enumerate() {
        *p ^= key[i];
    }

    p[7] ^= p[5];
    p[0] ^= p[2];

    p[5] = p[5].rotate_right(1);

    p[3] ^= p[4] ^ p[7];

    p[4] ^= p[1];

    p[4] = p[4].rotate_left(4);

    p[5] ^= p[3] ^ p[6];
    p[2] = p[2].rotate_right(2);

    p[6] = p[6].rotate_left(2);
    p[2] ^= p[4];
    p[1] = p[1].rotate_right(3);

    p[6] = p[5] ^ p[6] ^ p[7];
    p[1] = p[0] ^ p[1] ^ p[2];
}
