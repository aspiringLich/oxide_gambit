use super::*;
extern crate test;

#[cfg(test)]
mod benchmarks {
    use crate::ai::square_table::initialize_piece_tables;

    use super::*;
    use test::Bencher;

    /// ~10.9 ms    - Base Minmax
    /// ~6.7 ms     - Alpha Beta Pruning
    /// ~7.8 ms     - Square Tables
    /// ~7.1 ms     - Incremental Evaluation
    /// ~3.4 ms     - Fixed Alpha Beta Pruning lmao
    #[bench]
    fn bench_minimax(b: &mut Bencher) {
        let state = ChessState::from_FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ");

        // initialize the piece square tables
        initialize_piece_tables();

        b.iter(|| {
            state.run_minimax(3);
        })
    }
}
