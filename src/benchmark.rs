use super::*;
extern crate test;

#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::Bencher;

    /// ~10.9 ms    - Base Minmax
    /// ~6.7 ms     - Alpha Beta Pruning
    /// ~7.8 ms     - Square Tables
    /// ~
    #[bench]
    fn bench_minimax(b: &mut Bencher) {
        let state = ChessState::from_FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ");

        // generate all the lazy static's
        state.run_minimax(1);

        b.iter(|| {
            state.run_minimax(3);
        })
    }
}
