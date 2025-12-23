mod game;
#[allow(unused)]
mod types;

#[cfg(test)]
mod tests {
    use super::types::board::Board;

    #[test]
    fn it_works() {
        let board = Board::new();
        println!("{:?}", board);
    }
}
