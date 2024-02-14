mod exports;

const ROUND_128: usize = 16;
const ROUND_192: usize = 18;
const ROUND_256: usize = 20;

const GF28_M: u8 = 0b1110001;

type SBox = [u8; 256];
type Key128 = [u8; 16];
type Key192 = [u8; 24];
type Key256 = [u8; 32];
type Block128 = [u8; 16];
type Block192 = [u8; 24];
type Block256 = [u8; 32];

#[derive(Debug)]
struct Matrix128([[u8; 4]; 4]);

impl Matrix128 {
    fn new(bytes: Block128) -> Self {
        let mut mat = Self([[0; 4]; 4]);

        for row in 0..4 {
            for col in 0..4 {
                mat.0[row][col] = bytes[row * 4 + col];
            }
        }
        mat
    }
    fn dump(&self) -> Block128 {
        let mut bytes = [0; 16];

        for row in 0..4 {
            for col in 0..4 {
                bytes[row * 4 + col] = self.0[row][col];
            }
        }

        bytes
    }
}

#[derive(Debug)]
struct Matrix192([[u8; 4]; 6]);

impl Matrix192 {
    fn new(bytes: Block192) -> Self {
        let mut mat = Self([[0; 4]; 6]);

        for row in 0..6 {
            for col in 0..4 {
                mat.0[row][col] = bytes[row * 4 + col];
            }
        }
        mat
    }
    fn dump(&self) -> Block192 {
        let mut bytes = [0; 24];

        for row in 0..6 {
            for col in 0..4 {
                bytes[row * 4 + col] = self.0[row][col];
            }
        }

        bytes
    }
}

#[derive(Debug)]
struct Matrix256([[u8; 4]; 8]);

impl Matrix256 {
    fn new(bytes: Block256) -> Self {
        let mut mat = Self([[0; 4]; 8]);

        for row in 0..8 {
            for col in 0..4 {
                mat.0[row][col] = bytes[row * 4 + col];
            }
        }
        mat
    }
    fn dump(&self) -> Block256 {
        let mut bytes = [0; 32];

        for row in 0..8 {
            for col in 0..4 {
                bytes[row * 4 + col] = self.0[row][col];
            }
        }

        bytes
    }
}

/** Multiple on GF(2^8) */
fn gf_mul(f: u8, g: u8, m: u8) -> u8 {
    let mut cache = Vec::new();
    for i in 0..8 {
        if (g >> i) & 1 == 1 {
            let mut j = f;
            for _ in 0..i {
                /* the highest bit is 1 */
                if j >> 7 == 1 {
                    j <<= 1;
                    j ^= m;
                } else {
                    j <<= 1;
                }
            }
            cache.push(j);
        }
    }

    let mut result = 0;
    for i in cache {
        result ^= i;
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

fn bit_transform(b: u8, c: u8) -> u8 {
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

    b ^ c
}

/** Calculate the round constant */
fn rc(i: usize) -> u8 {
    let mut byte = 2;
    for _ in 0..(i + 2) {
        byte = gf_mul(byte, 2, GF28_M);
    }

    byte
}

fn round_key_gen_128(key: &Key128, round: usize) -> Key128 {
    let mut round_key = [0; 16];

    round_key[0] = gf_mul_inv(shl(key[0], 4), GF28_M) ^ rc(round);

    for i in 1..16 {
        round_key[i] = gf_mul_inv(shl(key[i], 4), GF28_M) ^ round_key[i - 1];
    }

    round_key
}

fn round_key_gen_192(key: &Key192, round: usize) -> Key192 {
    let mut round_key = [0; 24];

    round_key[0] = gf_mul_inv(shl(key[0], 4), GF28_M) ^ rc(round);

    for i in 1..24 {
        round_key[i] = gf_mul_inv(shl(key[i], 4), GF28_M) ^ round_key[i - 1];
    }

    round_key
}

fn round_key_gen_256(key: &Key256, round: usize) -> Key256 {
    let mut round_key = [0; 32];

    round_key[0] = gf_mul_inv(shl(key[0], 4), GF28_M) ^ rc(round);

    for i in 1..32 {
        round_key[i] = gf_mul_inv(shl(key[i], 4), GF28_M) ^ round_key[i - 1];
    }

    round_key
}

/** Generate S-Box */
fn s_box_gen(key: u8) -> SBox {
    let mut s_box = [0; 256];
    for (i, byte) in s_box.iter_mut().enumerate() {
        *byte = bit_transform(gf_mul_inv(i as u8, GF28_M), key);
    }
    s_box
}

fn s_inv_gen(s_box: &SBox) -> SBox {
    let mut s_inv = [0; 256];

    for i in 0..256 {
        s_inv[s_box[i] as usize] = i as u8;
    }

    s_inv
}

fn mix_columns_128(mat: &mut Matrix128) {
    for i in 1..4 {
        (
            mat.0[i % 4][i],
            mat.0[(i + 1) % 4][i],
            mat.0[(i + 2) % 4][i],
            mat.0[(i + 3) % 4][i],
        ) = (mat.0[0][i], mat.0[1][i], mat.0[2][i], mat.0[3][i]);
    }
}

fn mix_columns_192(mat: &mut Matrix192) {
    for i in 1..4 {
        (
            mat.0[i % 4][i],
            mat.0[(i + 1) % 4][i],
            mat.0[(i + 2) % 4][i],
            mat.0[(i + 3) % 4][i],
            mat.0[(i + 4) % 4][i],
            mat.0[(i + 5) % 4][i],
        ) = (
            mat.0[0][i],
            mat.0[1][i],
            mat.0[2][i],
            mat.0[3][i],
            mat.0[4][i],
            mat.0[5][i],
        );
    }
}

fn mix_columns_256(mat: &mut Matrix256) {
    for i in 1..4 {
        (
            mat.0[i % 8][i],
            mat.0[(i + 1) % 8][i],
            mat.0[(i + 2) % 8][i],
            mat.0[(i + 3) % 8][i],
            mat.0[(i + 4) % 8][i],
            mat.0[(i + 5) % 8][i],
            mat.0[(i + 6) % 8][i],
            mat.0[(i + 7) % 8][i],
        ) = (
            mat.0[0][i],
            mat.0[1][i],
            mat.0[2][i],
            mat.0[3][i],
            mat.0[4][i],
            mat.0[5][i],
            mat.0[6][i],
            mat.0[7][i],
        );
    }
}

fn mix_columns_inv_128(mat: &mut Matrix128) {
    for i in 1..4 {
        (
            mat.0[(4 - i) % 4][i],
            mat.0[(4 - i - 3) % 4][i],
            mat.0[(4 - i - 2) % 4][i],
            mat.0[(4 - i - 1) % 4][i],
        ) = (mat.0[0][i], mat.0[1][i], mat.0[2][i], mat.0[3][i]);
    }
}

fn mix_columns_inv_192(mat: &mut Matrix192) {
    for i in 1..4 {
        (
            mat.0[(4 - i) % 4][i],
            mat.0[(4 - i - 5) % 4][i],
            mat.0[(4 - i - 4) % 4][i],
            mat.0[(4 - i - 3) % 4][i],
            mat.0[(4 - i - 2) % 4][i],
            mat.0[(4 - i - 1) % 4][i],
        ) = (
            mat.0[0][i],
            mat.0[1][i],
            mat.0[2][i],
            mat.0[3][i],
            mat.0[4][i],
            mat.0[5][i],
        );
    }
}

fn mix_columns_inv_256(mat: &mut Matrix256) {
    for i in 1..4 {
        (
            mat.0[(8 - i) % 8][i],
            mat.0[(8 - i - 7) % 8][i],
            mat.0[(8 - i - 6) % 8][i],
            mat.0[(8 - i - 5) % 8][i],
            mat.0[(8 - i - 4) % 8][i],
            mat.0[(8 - i - 3) % 8][i],
            mat.0[(8 - i - 2) % 8][i],
            mat.0[(8 - i - 1) % 8][i],
        ) = (
            mat.0[0][i],
            mat.0[1][i],
            mat.0[2][i],
            mat.0[3][i],
            mat.0[4][i],
            mat.0[5][i],
            mat.0[6][i],
            mat.0[7][i],
        );
    }
}

fn sub_bytes_128(s_box: &SBox, mat: &mut Matrix128) {
    for row in 0..4 {
        for col in 0..4 {
            mat.0[row][col] = s_box[mat.0[row][col] as usize];
        }
    }
}

fn sub_bytes_192(s_box: &SBox, mat: &mut Matrix192) {
    for row in 0..6 {
        for col in 0..4 {
            mat.0[row][col] = s_box[mat.0[row][col] as usize];
        }
    }
}

fn sub_bytes_256(s_box: &SBox, mat: &mut Matrix256) {
    for row in 0..8 {
        for col in 0..4 {
            mat.0[row][col] = s_box[mat.0[row][col] as usize];
        }
    }
}

fn sub_bytes_inv_128(s_inv: &SBox, mat: &mut Matrix128) {
    for row in 0..4 {
        for col in 0..4 {
            mat.0[row][col] = s_inv[mat.0[row][col] as usize]
        }
    }
}

fn sub_bytes_inv_192(s_inv: &SBox, mat: &mut Matrix192) {
    for row in 0..6 {
        for col in 0..4 {
            mat.0[row][col] = s_inv[mat.0[row][col] as usize]
        }
    }
}

fn sub_bytes_inv_256(s_inv: &SBox, mat: &mut Matrix256) {
    for row in 0..8 {
        for col in 0..4 {
            mat.0[row][col] = s_inv[mat.0[row][col] as usize]
        }
    }
}

fn shl(num: u8, offset: u32) -> u8 {
    num.wrapping_shr(8 - offset) | (num << offset)
}

fn shr(num: u8, offset: u32) -> u8 {
    num.wrapping_shl(8 - offset) | (num >> offset)
}

fn func_f(p_0: u8, p_1: u8, p_2: u8, p_3: u8, key: [u8; 4]) -> (u8, u8, u8, u8) {
    /* XOR operation */
    let p_1 = p_0 ^ p_1 ^ p_2;
    let p_0 = shl(p_0, 2);
    let p_2 = shr(p_1, 4) ^ p_2 ^ p_3;
    let p_1 = shr(p_1, 2);
    let p_3 = shr(p_3, 2);
    let p_3 = p_3 ^ shl(p_2, 1);
    let p_0 = p_0 ^ p_1;
    let p_2 = shr(p_2, 3);
    let p_3 = shl(p_3, 1);

    /* Apply key */
    let (c_0, c_1, c_2, c_3) = (p_0 ^ key[0], p_1 ^ key[1], p_2 ^ key[2], p_3 ^ key[3]);

    (c_2, c_3, c_0, c_1)
}

fn func_f_inv(c_2: u8, c_3: u8, c_0: u8, c_1: u8, key: [u8; 4]) -> (u8, u8, u8, u8) {
    /* Apply key inverse */
    let (p_0, p_1, p_2, p_3) = (c_0 ^ key[0], c_1 ^ key[1], c_2 ^ key[2], c_3 ^ key[3]);

    /* Confuse inverse */
    let p_3 = shr(p_3, 1);
    let p_2 = shl(p_2, 3);
    let p_0 = p_0 ^ p_1;
    let p_3 = p_3 ^ shl(p_2, 1);
    let p_3 = shl(p_3, 2);
    let p_1 = shl(p_1, 2);
    let p_2 = shr(p_1, 4) ^ p_2 ^ p_3;
    let p_0 = shr(p_0, 2);
    let p_1 = p_0 ^ p_1 ^ p_2;

    (p_0, p_1, p_2, p_3)
}

fn apply_round_128(mat: &mut Matrix128, key: &Key128) {
    for i in 0..4 {
        (mat.0[i][0], mat.0[i][1], mat.0[i][2], mat.0[i][3]) = func_f(
            mat.0[i][0],
            mat.0[i][1],
            mat.0[i][2],
            mat.0[i][3],
            key[4 * i..4 * (i + 1)].try_into().unwrap(),
        );
    }
}

fn apply_round_192(mat: &mut Matrix192, key: &Key192) {
    for i in 0..6 {
        (mat.0[i][0], mat.0[i][1], mat.0[i][2], mat.0[i][3]) = func_f(
            mat.0[i][0],
            mat.0[i][1],
            mat.0[i][2],
            mat.0[i][3],
            key[4 * i..4 * (i + 1)].try_into().unwrap(),
        );
    }
}

fn apply_round_256(mat: &mut Matrix256, key: &Key256) {
    for i in 0..8 {
        (mat.0[i][0], mat.0[i][1], mat.0[i][2], mat.0[i][3]) = func_f(
            mat.0[i][0],
            mat.0[i][1],
            mat.0[i][2],
            mat.0[i][3],
            key[4 * i..4 * (i + 1)].try_into().unwrap(),
        );
    }
}

fn apply_round_inv_128(mat: &mut Matrix128, key: &Key128) {
    for i in 0..4 {
        (mat.0[i][0], mat.0[i][1], mat.0[i][2], mat.0[i][3]) = func_f_inv(
            mat.0[i][0],
            mat.0[i][1],
            mat.0[i][2],
            mat.0[i][3],
            key[4 * i..4 * (i + 1)].try_into().unwrap(),
        );
    }
}

fn apply_round_inv_192(mat: &mut Matrix192, key: &Key192) {
    for i in 0..6 {
        (mat.0[i][0], mat.0[i][1], mat.0[i][2], mat.0[i][3]) = func_f_inv(
            mat.0[i][0],
            mat.0[i][1],
            mat.0[i][2],
            mat.0[i][3],
            key[4 * i..4 * (i + 1)].try_into().unwrap(),
        );
    }
}

fn apply_round_inv_256(mat: &mut Matrix256, key: &Key256) {
    for i in 0..8 {
        (mat.0[i][0], mat.0[i][1], mat.0[i][2], mat.0[i][3]) = func_f_inv(
            mat.0[i][0],
            mat.0[i][1],
            mat.0[i][2],
            mat.0[i][3],
            key[4 * i..4 * (i + 1)].try_into().unwrap(),
        );
    }
}

#[derive(Debug)]
pub struct Cipher128 {
    s_boxes: [SBox; ROUND_128],
    s_inves: [SBox; ROUND_128],
    round_keys: [Key128; ROUND_128],
}

impl Cipher128 {
    pub fn new(key: Key128) -> Self {
        fn digest_key(key: &Key128) -> u8 {
            let mut byte = 0;

            for i in key.iter() {
                byte ^= *i;
            }

            std::cmp::max(byte, 1)
        }
        let mut s_boxes = [[0; 256]; ROUND_128];
        let mut s_inves = [[0; 256]; ROUND_128];
        let mut round_keys = [[0; 16]; ROUND_128];

        for round in 0..ROUND_128 {
            if round == 0 {
                round_keys[round] = round_key_gen_128(&key, round);
            } else {
                round_keys[round] = round_key_gen_128(&round_keys[round - 1], round);
            }
            s_boxes[round] = s_box_gen(digest_key(&round_keys[round]));
            s_inves[round] = s_inv_gen(&s_boxes[round]);
        }

        Self {
            s_boxes,
            s_inves,
            round_keys,
        }
    }
    pub fn encrypt(&self, block: Block128) -> Block128 {
        let mut mat = Matrix128::new(block);

        for round in 0..ROUND_128 {
            mix_columns_128(&mut mat);
            sub_bytes_128(&self.s_boxes[round], &mut mat);
            apply_round_128(&mut mat, &self.round_keys[round]);
        }

        mat.dump()
    }
    pub fn decrypt(&self, block: Block128) -> Block128 {
        let mut mat = Matrix128::new(block);

        for round in (0..ROUND_128).rev() {
            apply_round_inv_128(&mut mat, &self.round_keys[round]);
            sub_bytes_inv_128(&self.s_inves[round], &mut mat);
            mix_columns_inv_128(&mut mat);
        }

        mat.dump()
    }
}

#[derive(Debug)]
pub struct Cipher192 {
    s_boxes: [SBox; ROUND_192],
    s_inves: [SBox; ROUND_192],
    round_keys: [Key192; ROUND_192],
}

impl Cipher192 {
    pub fn new(key: Key192) -> Self {
        fn digest_key(key: &Key192) -> u8 {
            let mut byte = 0;

            for i in key.iter() {
                byte ^= *i;
            }

            std::cmp::max(byte, 1)
        }
        let mut s_boxes = [[0; 256]; ROUND_192];
        let mut s_inves = [[0; 256]; ROUND_192];
        let mut round_keys = [[0; 24]; ROUND_192];

        for round in 0..ROUND_192 {
            if round == 0 {
                round_keys[round] = round_key_gen_192(&key, round);
            } else {
                round_keys[round] = round_key_gen_192(&round_keys[round - 1], round);
            }
            s_boxes[round] = s_box_gen(digest_key(&round_keys[round]));
            s_inves[round] = s_inv_gen(&s_boxes[round]);
        }

        Self {
            s_boxes,
            s_inves,
            round_keys,
        }
    }
    pub fn encrypt(&self, block: Block192) -> Block192 {
        let mut mat = Matrix192::new(block);

        for round in 0..ROUND_192 {
            mix_columns_192(&mut mat);
            sub_bytes_192(&self.s_boxes[round], &mut mat);
            apply_round_192(&mut mat, &self.round_keys[round]);
        }

        mat.dump()
    }
    pub fn decrypt(&self, block: Block192) -> Block192 {
        let mut mat = Matrix192::new(block);

        for round in (0..ROUND_192).rev() {
            apply_round_inv_192(&mut mat, &self.round_keys[round]);
            sub_bytes_inv_192(&self.s_inves[round], &mut mat);
            mix_columns_inv_192(&mut mat);
        }

        mat.dump()
    }
}

#[derive(Debug)]
pub struct Cipher256 {
    s_boxes: [SBox; ROUND_256],
    s_inves: [SBox; ROUND_256],
    round_keys: [Key256; ROUND_256],
}

impl Cipher256 {
    pub fn new(key: Key256) -> Self {
        fn digest_key(key: &Key256) -> u8 {
            let mut byte = 0;

            for i in key.iter() {
                byte ^= *i;
            }

            std::cmp::max(byte, 1)
        }
        let mut s_boxes = [[0; 256]; ROUND_256];
        let mut s_inves = [[0; 256]; ROUND_256];
        let mut round_keys = [[0; 32]; ROUND_256];

        for round in 0..ROUND_256 {
            if round == 0 {
                round_keys[round] = round_key_gen_256(&key, round);
            } else {
                round_keys[round] = round_key_gen_256(&round_keys[round - 1], round);
            }
            s_boxes[round] = s_box_gen(digest_key(&round_keys[round]));
            s_inves[round] = s_inv_gen(&s_boxes[round]);
        }

        Self {
            s_boxes,
            s_inves,
            round_keys,
        }
    }
    pub fn encrypt(&self, block: Block256) -> Block256 {
        let mut mat = Matrix256::new(block);

        for round in 0..ROUND_256 {
            mix_columns_256(&mut mat);
            sub_bytes_256(&self.s_boxes[round], &mut mat);
            apply_round_256(&mut mat, &self.round_keys[round]);
        }

        mat.dump()
    }
    pub fn decrypt(&self, block: Block256) -> Block256 {
        let mut mat = Matrix256::new(block);

        for round in (0..ROUND_256).rev() {
            apply_round_inv_256(&mut mat, &self.round_keys[round]);
            sub_bytes_inv_256(&self.s_inves[round], &mut mat);
            mix_columns_inv_256(&mut mat);
        }

        mat.dump()
    }
}
