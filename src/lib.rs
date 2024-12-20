include!(concat!(env!("OUT_DIR"), "/constant.rs"));

mod exports;

const ROUND_256: usize = 16;
const ROUND_384: usize = 18;
const ROUND_512: usize = 20;

const GF28_M: u8 = 0b1110001;

type SBox = [u8; 256];
type Key256 = [u8; 32];
type Key384 = [u8; 48];
type Key512 = [u8; 64];
type Block256 = [u8; 32];
type Block384 = [u8; 48];
type Block512 = [u8; 64];

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

#[derive(Debug)]
struct Matrix256([[u8; 8]; 4]);

impl Matrix256 {
    fn new(bytes: Block256) -> Self {
        let mut mat = Self([[0; 8]; 4]);

        for row in 0..4 {
            for col in 0..8 {
                mat.0[row][col] = bytes[row * 8 + col];
            }
        }
        mat
    }
    fn dump(&self) -> Block256 {
        let mut bytes = [0; 32];

        for row in 0..4 {
            for col in 0..8 {
                bytes[row * 8 + col] = self.0[row][col];
            }
        }

        bytes
    }
    fn shift_down(&mut self, col: usize, step: usize) {
        reverse_col!(self.0, col, 0, 4 - step);
        reverse_col!(self.0, col, 4 - step, 4);
        reverse_col!(self.0, col, 0, 4);
    }
    fn shift_columns(&mut self) {
        for col in 1..8 {
            let step = col % 4;

            if step == 0 {
                continue;
            }

            self.shift_down(col, step);
        }
    }
    fn shift_columns_inv(&mut self) {
        for col in 1..8 {
            let step = (8 - col) % 4;

            if step == 0 {
                continue;
            }

            self.shift_down(col, step);
        }
    }
}

#[derive(Debug)]
struct Matrix384([[u8; 8]; 6]);

impl Matrix384 {
    fn new(bytes: Block384) -> Self {
        let mut mat = Self([[0; 8]; 6]);

        for row in 0..6 {
            for col in 0..8 {
                mat.0[row][col] = bytes[row * 8 + col];
            }
        }
        mat
    }
    fn dump(&self) -> Block384 {
        let mut bytes = [0; 48];

        for row in 0..6 {
            for col in 0..8 {
                bytes[row * 8 + col] = self.0[row][col];
            }
        }

        bytes
    }
    fn shift_down(&mut self, col: usize, step: usize) {
        reverse_col!(self.0, col, 0, 6 - step);
        reverse_col!(self.0, col, 6 - step, 6);
        reverse_col!(self.0, col, 0, 6);
    }
    fn shift_columns(&mut self) {
        for col in 1..8 {
            let step = col % 6;

            if step == 0 {
                continue;
            }

            self.shift_down(col, step);
        }
    }
    fn shift_columns_inv(&mut self) {
        for col in 1..8 {
            let step = (12 - col) % 6;

            if step == 0 {
                continue;
            }

            self.shift_down(col, step);
        }
    }
}

#[derive(Debug)]
struct Matrix512([[u8; 8]; 8]);

impl Matrix512 {
    fn new(bytes: Block512) -> Self {
        let mut mat = Self([[0; 8]; 8]);

        for row in 0..8 {
            for col in 0..8 {
                mat.0[row][col] = bytes[row * 8 + col];
            }
        }
        mat
    }
    fn dump(&self) -> Block512 {
        let mut bytes = [0; 64];

        for row in 0..8 {
            for col in 0..8 {
                bytes[row * 8 + col] = self.0[row][col];
            }
        }

        bytes
    }
    fn shift_down(&mut self, col: usize, step: usize) {
        reverse_col!(self.0, col, 0, 8 - step);
        reverse_col!(self.0, col, 8 - step, 8);
        reverse_col!(self.0, col, 0, 8);
    }
    fn shift_columns(&mut self) {
        for col in 1..8 {
            let step = col;

            self.shift_down(col, step);
        }
    }
    fn shift_columns_inv(&mut self) {
        for col in 1..8 {
            let step = 8 - col;

            self.shift_down(col, step);
        }
    }
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

fn round_key_gen_256(key: &Key256, round: usize) -> Key256 {
    let mut round_key = [0; 32];

    round_key[0] = gf_mul_inv(key[0].rotate_left(4), GF28_M) ^ rc(round);

    for i in 1..32 {
        round_key[i] = gf_mul_inv(key[i].rotate_left(4), GF28_M) ^ round_key[i - 1];
    }

    round_key
}

fn round_key_gen_384(key: &Key384, round: usize) -> Key384 {
    let mut round_key = [0; 48];

    round_key[0] = gf_mul_inv(key[0].rotate_left(4), GF28_M) ^ rc(round);

    for i in 1..48 {
        round_key[i] = gf_mul_inv(key[i].rotate_left(4), GF28_M) ^ round_key[i - 1];
    }

    round_key
}

fn round_key_gen_512(key: &Key512, round: usize) -> Key512 {
    let mut round_key = [0; 64];

    round_key[0] = gf_mul_inv(key[0].rotate_left(4), GF28_M) ^ rc(round);

    for i in 1..64 {
        round_key[i] = gf_mul_inv(key[i].rotate_left(4), GF28_M) ^ round_key[i - 1];
    }

    round_key
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

fn sub_bytes_256(s_boxes: &[SBox; 4], mat: &mut Matrix256) {
    for (row, s_box) in s_boxes.iter().enumerate() {
        for col in 0..8 {
            mat.0[row][col] = s_box[mat.0[row][col] as usize];
        }
    }
}

fn sub_bytes_384(s_boxes: &[SBox; 6], mat: &mut Matrix384) {
    for (row, s_box) in s_boxes.iter().enumerate() {
        for col in 0..8 {
            mat.0[row][col] = s_box[mat.0[row][col] as usize];
        }
    }
}

fn sub_bytes_512(s_boxes: &[SBox; 8], mat: &mut Matrix512) {
    for (row, s_box) in s_boxes.iter().enumerate() {
        for col in 0..8 {
            mat.0[row][col] = s_box[mat.0[row][col] as usize];
        }
    }
}

fn sub_bytes_inv_256(s_invs: &[SBox; 4], mat: &mut Matrix256) {
    for (row, s_inv) in s_invs.iter().enumerate() {
        for col in 0..8 {
            mat.0[row][col] = s_inv[mat.0[row][col] as usize]
        }
    }
}

fn sub_bytes_inv_384(s_invs: &[SBox; 6], mat: &mut Matrix384) {
    for (row, s_inv) in s_invs.iter().enumerate() {
        for col in 0..8 {
            mat.0[row][col] = s_inv[mat.0[row][col] as usize]
        }
    }
}

fn sub_bytes_inv_512(s_invs: &[SBox; 8], mat: &mut Matrix512) {
    for (row, s_inv) in s_invs.iter().enumerate() {
        for col in 0..8 {
            mat.0[row][col] = s_inv[mat.0[row][col] as usize]
        }
    }
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

fn apply_round_256(mat: &mut Matrix256, key: &Key256) {
    for row in 0..4 {
        func_f(
            &mut mat.0[row],
            key[8 * row..8 * row + 8].try_into().unwrap(),
        );
    }
}

fn apply_round_384(mat: &mut Matrix384, key: &Key384) {
    for row in 0..6 {
        func_f(
            &mut mat.0[row],
            key[8 * row..8 * row + 8].try_into().unwrap(),
        );
    }
}

fn apply_round_512(mat: &mut Matrix512, key: &Key512) {
    for row in 0..8 {
        func_f(
            &mut mat.0[row],
            key[8 * row..8 * row + 8].try_into().unwrap(),
        );
    }
}

fn apply_round_inv_256(mat: &mut Matrix256, key: &Key256) {
    for row in 0..4 {
        func_f_inv(
            &mut mat.0[row],
            key[8 * row..8 * row + 8].try_into().unwrap(),
        );
    }
}

fn apply_round_inv_384(mat: &mut Matrix384, key: &Key384) {
    for row in 0..6 {
        func_f_inv(
            &mut mat.0[row],
            key[8 * row..8 * row + 8].try_into().unwrap(),
        );
    }
}

fn apply_round_inv_512(mat: &mut Matrix512, key: &Key512) {
    for row in 0..8 {
        func_f_inv(
            &mut mat.0[row],
            key[8 * row..8 * row + 8].try_into().unwrap(),
        );
    }
}

/** NarrowWay-128 block cipher */
#[derive(Debug)]
pub struct Cipher256 {
    s_boxes: [[SBox; 4]; ROUND_256],
    s_inves: [[SBox; 4]; ROUND_256],
    round_keys: [Key256; ROUND_256],
}

impl Cipher256 {
    pub fn new(key: Key256) -> Self {
        let mut s_boxes = [[[0; 256]; 4]; ROUND_256];
        let mut s_inves = [[[0; 256]; 4]; ROUND_256];
        let mut round_keys = [[0; 32]; ROUND_256];

        for round in 0..ROUND_256 {
            if round == 0 {
                round_keys[round] = round_key_gen_256(&key, round);
            } else {
                round_keys[round] = round_key_gen_256(&round_keys[round - 1], round);
            }
            for s in 0..4 {
                s_boxes[round][s] = s_box_gen(digest_key(&round_keys[round][8 * s..8 * s + 8]));
                s_inves[round][s] = s_inv_gen(&s_boxes[round][s]);
            }
        }

        Self {
            s_boxes,
            s_inves,
            round_keys,
        }
    }
    /** Encrypt a block through NarrowWay-256 */
    pub fn encrypt(&self, block: Block256) -> Block256 {
        let mut mat = Matrix256::new(block);

        for round in 0..ROUND_256 {
            mat.shift_columns();
            sub_bytes_256(&self.s_boxes[round], &mut mat);
            apply_round_256(&mut mat, &self.round_keys[round]);
        }

        mat.dump()
    }
    /** Decrypt a block through NarrowWay-256 */
    pub fn decrypt(&self, block: Block256) -> Block256 {
        let mut mat = Matrix256::new(block);

        for round in (0..ROUND_256).rev() {
            apply_round_inv_256(&mut mat, &self.round_keys[round]);
            sub_bytes_inv_256(&self.s_inves[round], &mut mat);
            mat.shift_columns_inv();
        }

        mat.dump()
    }
}

/** NarrowWay-384 block cipher */
#[derive(Debug)]
pub struct Cipher384 {
    s_boxes: [[SBox; 6]; ROUND_384],
    s_inves: [[SBox; 6]; ROUND_384],
    round_keys: [Key384; ROUND_384],
}

impl Cipher384 {
    pub fn new(key: Key384) -> Self {
        let mut s_boxes = [[[0; 256]; 6]; ROUND_384];
        let mut s_inves = [[[0; 256]; 6]; ROUND_384];
        let mut round_keys = [[0; 48]; ROUND_384];

        for round in 0..ROUND_384 {
            if round == 0 {
                round_keys[round] = round_key_gen_384(&key, round);
            } else {
                round_keys[round] = round_key_gen_384(&round_keys[round - 1], round);
            }
            for s in 0..6 {
                s_boxes[round][s] = s_box_gen(digest_key(&round_keys[round][8 * s..8 * s + 8]));
                s_inves[round][s] = s_inv_gen(&s_boxes[round][s]);
            }
        }

        Self {
            s_boxes,
            s_inves,
            round_keys,
        }
    }
    /** Encrypt a block through NarrowWay-384 */
    pub fn encrypt(&self, block: Block384) -> Block384 {
        let mut mat = Matrix384::new(block);

        for round in 0..ROUND_384 {
            mat.shift_columns();
            sub_bytes_384(&self.s_boxes[round], &mut mat);
            apply_round_384(&mut mat, &self.round_keys[round]);
        }

        mat.dump()
    }
    /** Decrypt a block through NarrowWay-384 */
    pub fn decrypt(&self, block: Block384) -> Block384 {
        let mut mat = Matrix384::new(block);

        for round in (0..ROUND_384).rev() {
            apply_round_inv_384(&mut mat, &self.round_keys[round]);
            sub_bytes_inv_384(&self.s_inves[round], &mut mat);
            mat.shift_columns_inv();
        }

        mat.dump()
    }
}

/** NarrowWay-512 block cipher */
#[derive(Debug)]
pub struct Cipher512 {
    s_boxes: [[SBox; 8]; ROUND_512],
    s_inves: [[SBox; 8]; ROUND_512],
    round_keys: [Key512; ROUND_512],
}

impl Cipher512 {
    pub fn new(key: Key512) -> Self {
        let mut s_boxes = [[[0; 256]; 8]; ROUND_512];
        let mut s_inves = [[[0; 256]; 8]; ROUND_512];
        let mut round_keys = [[0; 64]; ROUND_512];

        for round in 0..ROUND_512 {
            if round == 0 {
                round_keys[round] = round_key_gen_512(&key, round);
            } else {
                round_keys[round] = round_key_gen_512(&round_keys[round - 1], round);
            }
            for s in 0..8 {
                s_boxes[round][s] = s_box_gen(digest_key(&round_keys[round][8 * s..8 * s + 8]));
                s_inves[round][s] = s_inv_gen(&s_boxes[round][s]);
            }
        }

        Self {
            s_boxes,
            s_inves,
            round_keys,
        }
    }
    /** Encrypt a block through NarrowWay-512 */
    pub fn encrypt(&self, block: Block512) -> Block512 {
        let mut mat = Matrix512::new(block);

        for round in 0..ROUND_512 {
            mat.shift_columns();
            sub_bytes_512(&self.s_boxes[round], &mut mat);
            apply_round_512(&mut mat, &self.round_keys[round]);
        }

        mat.dump()
    }
    /** Decrypt a block through NarrowWay-512 */
    pub fn decrypt(&self, block: Block512) -> Block512 {
        let mut mat = Matrix512::new(block);

        for round in (0..ROUND_512).rev() {
            apply_round_inv_512(&mut mat, &self.round_keys[round]);
            sub_bytes_inv_512(&self.s_inves[round], &mut mat);
            mat.shift_columns_inv();
        }

        mat.dump()
    }
}
