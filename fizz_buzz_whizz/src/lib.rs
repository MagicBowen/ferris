trait Game {
    fn apply(&self, number: u32) -> String;
}

#[cfg(feature = "game_v1")]
mod game_v1;

#[cfg(feature = "game_v2")]
mod game_v2;

pub fn play(max: u32) {
    #[cfg(feature = "game_v1")]
    let game = game_v1::FizzBuzzWhizz::new();
    #[cfg(feature = "game_v2")]
    let game = game_v2::FizzBuzzWhizz::new();

    (1..max).for_each(|i| println!("number of {} student say {}", i, game.apply(i)));
}