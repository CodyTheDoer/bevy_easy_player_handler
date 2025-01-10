use crate::{
    Player,
    PlayerAi,
    PlayerLocal,
    PlayerRemote,
    PlayerType,
};

use uuid::Uuid;

impl Player for PlayerAi {
    fn new(player_email: Option<String>, player_user_name: Option<String>, player_type: PlayerType) -> Self {
        PlayerAi {
            player_email: player_email.map(|email| email),
            player_id: Uuid::now_v7(),
            player_type: player_type,
            player_user_name: player_user_name.map(|user_name| user_name), 
        }
    }

    fn get_player_email(&self) -> &Option<String> {
        &self.player_email
    }

    fn get_player_id(&self) -> &Uuid {
        &self.player_id
    }

    fn get_player_type(&self) -> &PlayerType {
        &self.player_type
    }

    fn get_player_user_name(&self) -> &String {
        let player_user_name = self.player_user_name.as_ref().unwrap();
        player_user_name
    }

    fn set_player_email(&mut self, new_email: &str) {
        self.player_email = Some(new_email.to_string());
    }

    fn set_player_id(&mut self, new_id: Uuid) {
        self.player_id = new_id;
    }

    fn set_player_user_name(&mut self, new_user_name: &str) {
        self.player_user_name = Some(new_user_name.to_string());
    }
}

// --------------------------------------- //

impl Player for PlayerLocal {
    fn new(player_email: Option<String>, player_user_name: Option<String>, player_type: PlayerType) -> Self {
        PlayerLocal {
            player_email: player_email.map(|email| email),
            player_id: Uuid::now_v7(),
            player_type: player_type,
            player_user_name: player_user_name.map(|user_name| user_name), 
        }
    }

    fn get_player_email(&self) -> &Option<String> {
        &self.player_email
    }

    fn get_player_id(&self) -> &Uuid {
        &self.player_id
    }

    fn get_player_type(&self) -> &PlayerType {
        &self.player_type
    }

    fn get_player_user_name(&self) -> &String {
        let player_user_name = self.player_user_name.as_ref().unwrap();
        player_user_name
    }

    fn set_player_email(&mut self, new_email: &str) {
        self.player_email = Some(new_email.to_string());
    }

    fn set_player_id(&mut self, new_id: Uuid) {
        self.player_id = new_id;
    }

    fn set_player_user_name(&mut self, new_user_name: &str) {
        self.player_user_name = Some(new_user_name.to_string());
    }
}

// --------------------------------------- //

impl Player for PlayerRemote {
    fn new(player_email: Option<String>, player_user_name: Option<String>, player_type: PlayerType) -> Self {
        PlayerRemote {
            player_email: player_email.map(|email| email),
            player_id: Uuid::now_v7(),
            player_type: player_type,
            player_user_name: player_user_name.map(|user_name| user_name), 
        }
    }

    fn get_player_email(&self) -> &Option<String> {
        &self.player_email
    }

    fn get_player_id(&self) -> &Uuid {
        &self.player_id
    }

    fn get_player_type(&self) -> &PlayerType {
        &self.player_type
    }

    fn get_player_user_name(&self) -> &String {
        let player_user_name = self.player_user_name.as_ref().unwrap();
        player_user_name
    }

    fn set_player_email(&mut self, new_email: &str) {
        self.player_email = Some(new_email.to_string());
    }

    fn set_player_id(&mut self, new_id: Uuid) {
        self.player_id = new_id;
    }

    fn set_player_user_name(&mut self, new_user_name: &str) {
        self.player_user_name = Some(new_user_name.to_string());
    }
}