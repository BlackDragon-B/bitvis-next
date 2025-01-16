use std::time::Duration;

use mpris::{Metadata, Player, PlayerFinder};

pub struct Mpris {
    pub player_finder: PlayerFinder,
    pub player: Option<Player>,
    pub metadata: Option<Metadata>
}

impl Mpris {
    pub fn get_player(&mut self) {
        match self.player_finder.find_active() {
            Ok(T) => self.player = Some(T),
            Err(E) => (),
        }
    }

    pub fn get_progress(&mut self) -> Result<Duration, &str> {
        let res: Result<Duration, &str> = match &self.player {
            Some(player) => {
                match player.get_position() {
                    Ok(O) => Ok(O),
                    Err(E) => Err("Failed to get player position"),
                }
            },
            None => Err("Failed to get player position"),
        };
        res
    }
    pub fn get_metadata(&mut self) -> Result<Metadata, &'static str> {
        // match &self.player { 
        //     Some(player) => {
        //         match player.get_metadata() {
        //             Ok(T) => self.metadata = Some(T),
        //             Err(E) => self.player = None,
        //         }
        //     },
        //     None => todo!(),
        // }
        match &self.player {
            Some(player) => {
                match player.get_metadata() {
                    Ok(T) => Ok(T),
                    Err(E) => Err("Failed to get player metadata")
                }
            },
            None => Err("No player found"),
        }
    }
}
pub fn create() -> Result<Mpris, &'static str> {
    let mut player_finder: Option<PlayerFinder> = None;
    match PlayerFinder::new() {
        Ok(T) => {player_finder = Some(T);},
        Err(E) => {return Err("Could not connect to D-Bus")}
    };

    // player.pause().expect("Could not pause");
    Ok(Mpris {player_finder: player_finder.unwrap(), player: None, metadata: None})
}