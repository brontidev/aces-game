#[allow(unused)]

mod types;
mod state;

#[cfg(test)]
mod tests {
    use super::types::board::Board;

    #[test]
    fn it_works() {
        let board = Board::new();
        println!("{:?}", board);
    }
}
