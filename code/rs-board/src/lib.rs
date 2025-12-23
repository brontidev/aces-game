enum Side {
    Left,
    Right
}

enum PieceKind {
    Core,
    Monarch,
    Brute(Side),
    Tank
}
