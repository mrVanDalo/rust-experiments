
/// supported Engines
pub enum Engine{
    Jack,
    Alsa
}

/// full Path to a file
pub type Filename<'a> = &'a String;
/// Result of a playback
pub type PlayResult = String;
/// Function signature to play a sound
pub type Player = Fn(Filename) -> String;

/// returns a function (given the engine config) which should be played
pub fn init_player(engine : Engine) -> Box<Player> {
    match engine {
        Engine::Jack => Box::new(play_jack),
        Engine::Alsa => Box::new(play_alsa)
    }
}

/// play a track using the jack engine
fn play_jack(track : Filename) -> PlayResult {
    format!("playing with jack {}", track)
}

/// play a track using the alsa engine
fn play_alsa(track : Filename) -> PlayResult {
    format!("playing with alsa {}", track)
}

#[cfg(test)]
mod function_pointer_tests {

    use super::*;

    #[test]
    fn test_jack() {
        let player : Box<Player> = init_player(Engine::Jack);
        let filename = String::from("woop");
        assert_eq! (String::from("playing with jack woop"), player(&filename));
    }

    #[test]
    fn test_alsa() {
        let player : Box<Player> = init_player(Engine::Alsa);
        let filename = String::from("woop");
        assert_eq! (String::from("playing with alsa woop"), player(&filename));
    }
}
