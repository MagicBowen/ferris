trait Game {
    fn apply(&self, number: u32) -> String;
}

#[cfg(feature = "game_v1")]
mod game_v1;

#[cfg(feature = "game_v2")]
mod game_v2;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub fn play_result(max: u32) -> Vec<String> {
    #[cfg(feature = "game_v1")]
    let game = game_v1::FizzBuzzWhizz::new();
    #[cfg(feature = "game_v2")]
    let game = game_v2::FizzBuzzWhizz::new();

    #[cfg(not(feature = "parallel"))]
    let result = (1..max).map(|i| game.apply(i)).collect();

    // RAYON_NUM_THREADS=4 cargo bench --bench game_bench_test --features=game_v2,parallel
    #[cfg(feature = "parallel")]
    let result = (1..max).into_par_iter().map(|i| game.apply(i)).collect();

    result
}

pub fn play(max: u32) {
    play_result(max).iter().enumerate().for_each(|(i, result)| {
        println!("student {} say: {}", i + 1, result);
    });
}
