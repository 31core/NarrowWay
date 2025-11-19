use crate::*;

type Key384 = [u8; 48];
type Block384 = [u8; 48];

const ROUND_384: usize = 18;

fn sub_bytes_384(s_boxes: &[SBox; 6], mat: &mut Matrix384) {
    for (row, s_box) in s_boxes.iter().enumerate() {
        for col in 0..8 {
            mat.0[row][col] = s_box[mat.0[row][col] as usize];
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

fn apply_round_384(mat: &mut Matrix384, key: &Key384) {
    for row in 0..6 {
        func_f(
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

fn round_key_gen_384(key: &Key384, round: usize) -> Key384 {
    let mut round_key = [0; 48];

    round_key[0] = gf_mul_inv(key[0].rotate_left(4), GF28_M) ^ rc(round);

    for i in 1..48 {
        round_key[i] = gf_mul_inv(key[i].rotate_left(4), GF28_M) ^ round_key[i - 1];
    }

    round_key
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
