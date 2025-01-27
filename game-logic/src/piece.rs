#[derive(Clone, Debug, Copy)]
pub enum PieceKind {
    B, // b
    R, // r
    E, // e
    A, // a
    K, // k
    I, // !
    X, // gray x cell
    S, // gold x cell
}

#[derive(Clone, Debug, Copy)]
pub enum PowerKind {
    None,
    Def,
    Att,
}
