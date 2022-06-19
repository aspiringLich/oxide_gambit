use super::*;
extern crate test;

#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::Bencher;

    /// ~10.9 ms
    #[bench]
    fn bench_minimax(b: &mut Bencher) {
        let state = ChessState::from_FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ");

        b.iter(|| {
            state.run_minimax(3);
        })
    }
}
