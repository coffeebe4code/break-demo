use crate::piece::{PieceKind, PowerKind};
#[derive(Clone, Debug, Copy)]
pub struct Single {
    pub kind: PieceKind,
    pub pkind: PowerKind,
}

impl Single {
    pub fn new(kind: PieceKind, pkind: PowerKind) -> Self {
        Self { kind, pkind }
    }
    pub fn from_random(input: u64) -> Self {
        let new = mod_u64_to_6(input);
        let mut single = Single {
            kind: PieceKind::B,
            pkind: PowerKind::None,
        };
        match new {
            5 => single.kind = PieceKind::I,
            4 => single.kind = PieceKind::K,
            3 => single.kind = PieceKind::A,
            2 => single.kind = PieceKind::E,
            1 => single.kind = PieceKind::R,
            _ => single.kind = PieceKind::B,
        }
        return single;
    }
    pub fn set_power(&mut self, pkind: PowerKind) {
        self.pkind = pkind;
    }
}

fn mod_u64_to_6(input: u64) -> u64 {
    input % 6
}

macro_rules! single {
    ($i:ident, $j:ident) => {
        Single::new($i, $j)
    };
}
