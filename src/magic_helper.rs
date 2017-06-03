use bit_twiddles;
use board::*;
use templates::*;


struct MagicHelper {
    square_BB: [u64; 64], // Maps index to square
    magic_bishop_table: [[u64; 4096]; 64],
    magic_rook_table: [[u64; 1024]; 64],
    knight_table: [u64; 64],
    king_table: [u64; 64]
}

//impl MagicHelper {
//    pub fn new() -> MagicHelper {
//        MagicHelper {
//            magic_bishop_moves: MagicHelper::gen_magic_bishop(),
//            magic_rook_moves: MagicHelper::gen_magic_rook()
//        }
//    }
//
//    pub fn default() -> MagicHelper { MagicHelper::new() }
//
//    fn gen_magic_bishop() -> [[u64; 4096]; 64] {
//        let mut arr: [[u64; 4096]; 64] = [[0; 4096]; 64];
//        let mut mask: u64 = 0;
//        for bitRef in 0..64 {
//            mask = BISHOP_MASK[bitRef];
//
//        }
//
//    }
//
//    fn gen_magic_rook() -> [[u64; 1024]; 64] {
//        let mut arr: [[u64; 1024]; 64] = [[0; 1024]; 64];
//
//    }
//}


fn gen_king_moves() -> [u64; 64] {
    let mut moves: [u64;64] = [0; 64];

    for index in 0..64 {
        let mut mask: u64 = 0;
        let file = index % 8;
        // LEFT
        if file != 0 {
            mask |= 1 << (index - 1);
        }
        // RIGHT
        if file != 7 {
            mask |= 1 << (index + 1);
        }
        // UP
        if index < 56  {
            mask |= 1 << (index + 8);
        }
        // DOWN
        if index > 7  {
            mask |= 1 << (index - 8);
        }
        // LEFT UP
        if file != 0 && index < 56 {
            mask |= 1 << (index + 7);
        }
        // LEFT DOWN
        if file != 0 && index > 7 {
            mask |= 1 << (index - 9);
        }
        // RIGHT DOWN
        if file!= 7 && index > 7 {
            mask |= 1 << (index - 7);
        }
        // RIGHT UP
        if file != 7 && index < 56 {
            mask |= 1 << (index + 0);
        }
        moves[index] = mask;
    }
    moves
}

fn gen_knight_moves() -> [u64; 64] {
    let mut moves: [u64;64] = [0; 64];
    for index in 0..64 {
        let mut mask: u64 = 0;
        let file = index % 8;

        // 1 UP   + 2 LEFT
        if file > 1 && index < 56 {
            mask |= 1 << (index + 6);
        }
        // 2 UP   + 1 LEFT
        if file != 0 && index < 48 {
            mask |= 1 << (index + 15);
        }
        // 2 UP   + 1 RIGHT
        if file != 7 && index < 48 {
            mask |= 1 << (index + 17);
        }
        // 1 UP   + 2 RIGHT
        if file < 6 && index < 56 {
            mask |= 1 << (index + 10);
        }
        // 1 DOWN   + 2 RIGHT
        if file < 6 && index > 7 {
            mask |= 1 << (index - 6);
        }
        // 2 DOWN   + 1 RIGHT
        if file != 7 && index > 15 {
            mask |= 1 << (index - 15 );
        }
        // 2 DOWN   + 1 LEFT
        if file != 0 && index > 15 {
            mask |= 1 << (index - 17 );
        }
        // 1 DOWN   + 2 LEFT
        if file > 1 && index > 7 {
            mask |= 1 << (index - 10 );
        }
        moves[index] = mask;
    }
    moves
}

//Bitboard  RookMasks  [SQUARE_NB];
//Bitboard  RookMagics [SQUARE_NB];
//Bitboard* RookAttacks[SQUARE_NB];
//unsigned  RookShifts [SQUARE_NB];
//
//Bitboard  BishopMasks  [SQUARE_NB];
//Bitboard  BishopMagics [SQUARE_NB];
//Bitboard* BishopAttacks[SQUARE_NB];
//unsigned  BishopShifts [SQUARE_NB];
//Bitboard RookTable[0x19000];  // To store rook attacks
//Bitboard BishopTable[0x1480]; // To store bishop attacks

// RookTable
fn get_magics() {
    let rook_table: [u64; 0x19000] = [0; 0x19000];
    let bishop_table: [u64; 0x1480] = [0; 0x1480];

    let rook_masks: [u64; 64] = [0; 64];
    let rook_magics: [u64; 64] = [0; 64];
    let rook_attacks: [u64; 64] = [0; 64];
    let rook_shifts: [u64; 64] = [0; 64];

    let bishop_masks: [u64; 64] = [0; 64];
    let bishop_magics: [u64; 64] = [0; 64];
    let bishop_attacks: [u64; 64] = [0; 64];
    let bishop_shifts: [u64; 64] = [0; 64];

}


fn init_rook_magics(mut table: [u64; 0x19000], mut attacks: [u64; 64], magics: [u64; 64],
                    mut masks: [u64; 64], shifts: [u64; 64], deltas: [u64; 64], index: [u64; 64]) {

    let seeds: [[i32;8]; 2] = [ [ 8977, 44560, 54343, 38998,  5731, 95205, 104912, 17020 ],
                                [  728, 10316, 55013, 32803, 12281, 15100,  16645,   255 ] ];

    let bishop_deltas: [i8; 4] = [7,9,9,7];
    let rook_deltas: [i8; 4] = [8,1,-8,1];

    let mut occupancy: [u64; 4096] = [0; 4096];
    let mut reference: [u64; 4096] = [0; 4096];
    let mut edges: u64 = 0;
    let mut age: [i32; 4096] =  [0; 4096];
    let mut current: i32 = 0;
    let mut size: usize = 0;

    for s in 0..64 {
        // ((Rank1BB | Rank8BB) & ~rank_bb(s)) | ((FileABB | FileHBB) & ~file_bb(s));
        edges = ((RANK_1 | RANK_8) & !rank_bb(s)) | ((FILE_A | FILE_B) & !file_bb(s));

        masks[s] = sliding_attack(deltas, s, 0) & !edges;
        shifts[s] = 64 - popcount(masks[s as usize]);
        d = size = 0;

        loop {
            occupancy[size] = b;
            reference[size] = sliding_attack(deltas, s, b);
            size += 1;
            b = (b - masks[s as usize]) * masks[s as usize];
            if b == 0 {
                break;
            }
        }

        if s < 63 {
            attacks[s + 1] = attacks[s as usize] + size;
        }

        let mut rng = PRNG::init(seeds[1][rank_of(s)]);

        'outer: loop {
            'first_in: loop {
                magics[s] = rng.sparse_rand();
                if popcount((magics[s as usize] * masks[s as usize]) >> 56) < 6 {
                    break 'first_in;
                }
            }
            // magic_index return unsigned(((occupied & Masks[s]) * Magics[s]) >> Shifts[s]);
            current += 1;
            let mut i: usize = 0;
            'secon_in: while i < size {
                let index: usize = (((occupied[i as usize] & masks[s as usize]) * magics[s as usize]) >> Shifts[s as usize]) as usize;
                if age[index] < current {
                    age[index] = current;
                    attacks[s as usize][index] = reference[i];
                } else if attacks[s as usize][index as usize] != reference[i] {
                    break 'secon_in;
                }
            }
            if i < size {
                break 'outer;
            }
        }
    }

}

struct PRNG {
    seed: u64
}

impl PRNG {
    pub fn init(s: seed) -> PRNG {
        assert!(s);
        PRNG {seed: s}
    }

    pub fn rand(&mut self) -> u64 {
        self.rand_change()
    }

    pub fn sparse_rand(&mut self) -> u64 {
        let mut s = self.rand_change();
        s ^= self.rand_change();
        s ^= self.rand_change();
        s
    }

    fn rand_change(&mut self) -> u64 {
        self.seed ^= self.seed >> 12;
        self.seed ^= self.seed << 25;
        self.seed ^= self.seed >> 27;

        self.seed * 2685821657736338717
    }
}

fn sliding_attack(deltas: [u64; 4], square: u64, u64: occupied) -> u64 {
    let mut attack: u64 = 0;

    for i in 0..4 {
        let mut s: u64 = square + deltas[i];
        while is_ok(s) &&  distance(s, s - deltas[i]) == 1 {

            if occupied & s { break;}
            s += deltas[i];
        }
    }
    attack
}


pub fn gen_rook_masks() {
    let mut arr_masks: [u64; 64] = [0; 64];
    let mut shifts: [u8; 64] = [0; 64];

    let mut bit_ref: usize = 0;
    while bit_ref < 64 {
        let mut mask: u64 = 0;
        let mut i = bit_ref + 8;
        while i < 56 {
            mask |= (1 as u64) << (i as u8);
            i += 8;
        }
        if bit_ref > 7 {
            let mut i = bit_ref - 8;
            while i > 7 {
                mask |= (1 as u64) << (i as u8);
                i -= 8;
            }
        }
        let mut i = bit_ref + 1;
        while i % 8 != 0 && i <= 63 {
            mask |= (1 as u64) << (i as u8);
            i += 1;
        }
        if bit_ref > 0  {
            let mut i = bit_ref - 1;
            while i % 8 != 0 && i >= 0 {
                mask |= (1 as u64) << (i as u8);
                i -= 1;
            }
        }
        arr_masks[bit_ref] = mask;
        format_bits(format!("{:b}",mask));
        bit_ref += 1;
    }
}

pub fn gen_bishop_masks() {
    let mut arr_masks: [u64; 64] = [0; 64];
    let mut shifts: [u8; 64] = [0; 64];

    let mut bitRef: i32 = 0;
    while bitRef < 64 {
        let mut mask: u64 = 0;

        let mut i = bitRef + 9;
        while i < 56 && i % 8 != 0 && i % 8 != 7 {
            mask |= (1 as u64) << (i as u8);
            i += 9;
        }
        let mut i = bitRef - 9;
        while i > 7 && i % 8 != 0 && i % 8 != 7 {
            mask |= (1 as u64) << (i as u8);
            i -= 9;
        }
        let mut i = bitRef + 7;
        while i < 56 && i % 8 != 0 && i % 8 != 7 {
            mask |= (1 as u64) << (i as u8);
            i += 7;
        }

        let mut i = bitRef - 7;
        while i > 7 && i % 8 != 0 && i % 8 != 7 {
            mask |= (1 as u64) << (i as u8);
            i -= 7;
        }
        format_bits(format!("{:b}",mask));
        arr_masks[bitRef as usize] = mask;
        bitRef += 1;
    }
}

pub fn format_bits(bits: String) {
    let x = 64 - bits.len();
    let mut i = 0;
    while i < x {
        print!("0");
        i += 1;
    }
    println!("{}",bits);
}

#[test]
fn test_king_mask_gen() {
    let arr = gen_king_moves().to_vec();
    let sum = arr.iter().fold(0 as  u64,|a, &b| a + (bit_twiddles::popcount64(b) as u64));
    assert_eq!(sum, (3*4) + (5 * 6 * 4) + (8 * 6 * 6));
}

#[test]
fn test_knight_mask_gen() {
    let arr = gen_knight_moves().to_vec();
    let sum = arr.iter().fold(0 as  u64,|a, &b| a + (bit_twiddles::popcount64(b) as u64));
    assert_eq!(sum, (2 * 4) + (4 * 4) + (3 * 2 * 4) + (4 * 4 * 4) + (6 * 4 * 4) + (8 * 4 * 4));
}
