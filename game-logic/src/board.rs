use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::coord::Coord;
use crate::piece::*;
use crate::single::*;
use crate::triple::*;

pub struct Board {
    pub seed: StdRng,
    pub next: [Triple; 2],
    pub board: [[Option<Single>; 6]; 19],
    pub sister_board: [[Option<Single>; 6]; 19],
    pub b_count: u32,
    pub r_count: u32,
    pub e_count: u32,
    pub a_count: u32,
    pub k_count: u32,
    pub i_count: u32,
}

impl Board {
    pub fn new(seed: u64) -> Self {
        let mut seed = StdRng::seed_from_u64(seed);

        let next = [from_random(&mut seed), from_random(&mut seed)];
        let board = {
            let row = { [None, None, None, None, None, None] };
            [row.clone(); 19]
        };
        let sister_board = {
            let row = { [None, None, None, None, None, None] };
            [row.clone(); 19]
        };

        Self {
            seed,
            next,
            board,
            sister_board,
            b_count: 0,
            r_count: 0,
            e_count: 0,
            a_count: 0,
            k_count: 0,
            i_count: 0,
        }
    }
    // sets down the current triple and returns the one in next
    pub fn set_down(&mut self, coord: Coord, triple: Triple) -> Triple {
        self.set_from_coord(&coord, triple.bot);
        self.set_from_coord(
            &Coord {
                x: coord.x,
                y: coord.y + 1,
            },
            triple.bot,
        );
        self.set_from_coord(
            &Coord {
                x: coord.x,
                y: coord.y + 2,
            },
            triple.bot,
        );
        let new = self.next[1].clone();
        self.next[1] = self.next[0].clone();
        self.next[0] = self.generate();
        while self.check_breaks() {}
        return new;
    }
    pub fn check_if_above(&self) -> bool {
        // todo check if above
        return false;
    }
    // returns true if there were breaks. trigger more.
    pub fn check_breaks(&mut self) -> bool {
        return false;
    }
    pub fn generate(&mut self) -> Triple {
        from_random(&mut self.seed)
    }
    pub fn set_from_coord(&mut self, coord: &Coord, single: Single) {
        if coord.y > 15 {
            self.board[coord.x as usize][16] = Some(single);
        }
        self.board[coord.x as usize][coord.y as usize] = Some(single);
    }
    pub fn clear_from_coord(&mut self, coord: Coord) {
        self.board[coord.x as usize][coord.y as usize] = None;
    }
}

fn from_random(rng: &mut StdRng) -> Triple {
    let top = get_next(rng);
    let mid = get_next(rng);
    let bot = get_next(rng);
    Triple::from_random(top, mid, bot)
}

fn get_next(rng: &mut StdRng) -> u64 {
    rng.gen()
}
