use bevy::prelude::*;

use bevy_easy_shared_definitions::{
    DatabaseConnection, 
    ErrorTypePlayerHandler,
};

use std::sync::Arc;
use std::sync::Mutex;

use uuid::Uuid;

pub mod database;
pub mod handlers;

#[derive(Clone, Resource)]
pub struct BevyEasyPlayerHandlerPlugin {
    main_player_email: Option<String>,
    main_player_user_name: Option<String>,
    main_player_uuid: Option<Uuid>,
    party_size: Option<i32>,
}

impl BevyEasyPlayerHandlerPlugin {
    pub fn init() -> Self {
        BevyEasyPlayerHandlerPlugin {
            main_player_email: None,
            main_player_user_name: None,
            main_player_uuid: None,
            party_size: None,
        }
    }

    pub fn main_player_email(mut self, main_player_email: &str) -> Self {
        self.main_player_email = Some(String::from(main_player_email));
        self
    }

    pub fn main_player_user_name(mut self, main_player_user_name: &str) -> Self {
        self.main_player_user_name = Some(String::from(main_player_user_name));
        self
    }

    pub fn main_player_uuid(mut self, main_player_uuid: &Uuid) -> Self {
        self.main_player_uuid = Some(*main_player_uuid);
        self
    }

    pub fn party_size(mut self, party_size: i32) -> Self {
        self.party_size = Some(party_size);
        self
    }

    pub fn build(mut self) -> BevyEasyPlayerHandlerPlugin {
        if self.main_player_uuid.is_none() {
            self.main_player_uuid = Some(Uuid::now_v7());
        }
        
        if self.main_player_user_name.is_none() {
            self.main_player_user_name = Some(String::from(self.main_player_uuid.unwrap()));
        }
        
        let party_size = match self.party_size {
            Some(i32) => i32,
            None => 1,
        };
        self.party_size = Some(party_size);

        Self {
            main_player_email: self.main_player_email,
            main_player_user_name: self.main_player_user_name,
            main_player_uuid: self.main_player_uuid,
            party_size: self.party_size,
        }
    }

    pub fn get_main_player_email(&self) -> Result<Option<&String>, ErrorTypePlayerHandler> {
        Ok(self.main_player_email.as_ref())
    }

    pub fn get_main_player_user_name(&self) -> Result<Option<&String>, ErrorTypePlayerHandler> {
        Ok(self.main_player_user_name.as_ref())
    }

    pub fn get_main_player_uuid(&self) -> Result<Option<&Uuid>, ErrorTypePlayerHandler> {
        Ok(self.main_player_uuid.as_ref())
    }

    pub fn get_party_size_limit(&self) -> Result<Option<&i32>, ErrorTypePlayerHandler> {
        Ok(self.party_size.as_ref())
    }

    pub fn set_party_size_limit(&mut self, party_size: i32) -> Result<(), ErrorTypePlayerHandler> {
        self.party_size = Some(party_size);
        Ok(())
    }

    pub fn set_main_player_uuid(&mut self, new_uuid: &Uuid) -> Result<(), ErrorTypePlayerHandler> {
        let owned_id = new_uuid.to_owned();
        self.main_player_uuid = Some(owned_id);
        Ok(())
    }
}

impl Plugin for BevyEasyPlayerHandlerPlugin {
    fn build(&self, app: &mut App) { // Builds automatically on .add_plugins() call
        // Ensure the database connection exists as a resource
        if !app.world().contains_resource::<DatabaseConnection>() {
            panic!("ERROR: [ DatabaseConnection ] resource is missing. Ensure the host app provides it.");
        }

        // Insert the plugin itself as a resource
        app.insert_resource(self.clone());
        app.insert_resource(PlayerHandlerInterface::get());

        let party = Party::new(
            self.main_player_email.as_ref().unwrap(), 
            self.main_player_user_name.as_ref().unwrap(),
            self.party_size.unwrap(),
        );
        app.insert_resource(party);

        // Add the startup protocol system
        app.add_systems(Startup, PlayerHandlerInterface::start_up_protocol);
    }
}

#[derive(Resource)]
pub struct PlayerHandlerInterface {}

#[derive(Clone, Debug)]
pub struct DBPlayer {
    uuid: String,
    email: String,
    user_name: String,
}

#[derive(Resource)]
pub struct Party {
    pub active_player: i32,
    pub ai_vec: Option<Vec<usize>>,
    pub party_size: i32,
    pub players: Arc<Mutex<Vec<Arc<Mutex<dyn Player + Send>>>>>,
}

pub trait Player { //  ->  
    fn new(player_email: Option<String>, player_user_name: Option<String>, player_type: PlayerType) -> Self where Self: Sized;
    fn clone_with_new_id(&self) -> Result<Arc<Mutex<dyn Player + Send>>, ErrorTypePlayerHandler>;
    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler>;
    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler>;
    fn get_player_type(&self) -> Result<&PlayerType, ErrorTypePlayerHandler>;
    fn get_player_user_name(&self) -> Result<&String, ErrorTypePlayerHandler>;
    fn set_player_email(&mut self, new_email: &str) -> Result<(), ErrorTypePlayerHandler>;
    fn set_player_id(&mut self, new_id: Uuid) -> Result<(), ErrorTypePlayerHandler>;
    fn set_player_user_name(&mut self, new_user_name: &str) -> Result<(), ErrorTypePlayerHandler>;
}

#[derive(Component)]
pub struct PlayerComponent {
    player: Arc<Mutex<dyn Player + Send>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerType {
    PlayerAiLocal,
    PlayerAiRemote,
    PlayerLocal,
    PlayerMain,
    PlayerRemote,
    PlayerTestRef,
}

#[derive(Clone, Component, Resource)]
pub struct PlayerAi {
    player_email: Option<String>,
    player_id: Uuid,
    player_type: PlayerType,
    player_user_name: Option<String>,
}

#[derive(Clone, Component, Resource)]
pub struct PlayerLocal {
    player_email: Option<String>,
    player_id: Uuid,
    player_type: PlayerType,
    player_user_name: Option<String>,
}

#[derive(Clone, Component, Resource)]
pub struct PlayerRemote {
    player_email: Option<String>,
    player_id: Uuid,
    player_type: PlayerType,
    player_user_name: Option<String>,
}