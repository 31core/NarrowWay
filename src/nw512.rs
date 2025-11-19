use crate::*;

type Key512 = [u8; 64];
type Block512 = [u8; 64];

const ROUND_512: usize = 20;

fn sub_bytes_512(s_boxes: &[SBox; 8], mat: &mut Matrix512) {
    for (row, s_box) in s_boxes.iter().enumerate() {
        for col in 0..8 {
            mat.0[row][col] = s_box[mat.0[row][col] as usize];
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

fn apply_round_512(mat: &mut Matrix512, key: &Key512) {
    for row in 0..8 {
        func_f(
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

fn round_key_gen_512(key: &Key512, round: usize) -> Key512 {
    let mut round_key = [0; 64];

    round_key[0] = gf_mul_inv(key[0].rotate_left(4), GF28_M) ^ rc(round);

    for i in 1..64 {
        round_key[i] = gf_mul_inv(key[i].rotate_left(4), GF28_M) ^ round_key[i - 1];
    }

    round_key
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
