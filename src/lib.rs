use bevy::prelude::*;

use bevy_easy_shared_definitions::DatabaseConnection;

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
    target_idx: Option<i32>,
}

impl BevyEasyPlayerHandlerPlugin {
    pub fn init() -> Self {
        BevyEasyPlayerHandlerPlugin {
            main_player_email: None,
            main_player_user_name: None,
            main_player_uuid: None,
            party_size: None,
            target_idx: None,
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

    pub fn target_idx(mut self, target_idx: i32) -> Self {
        self.target_idx = Some(target_idx);
        self
    }

    pub fn default(mut self) -> Self {
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

        let target_idx = match self.target_idx {
            Some(i32) => i32,
            None => 0,
        };
        self.target_idx = Some(target_idx); 
        
        self
    }

    pub fn build(self) -> BevyEasyPlayerHandlerPlugin {
        Self {
            main_player_email: self.main_player_email,
            main_player_user_name: self.main_player_user_name,
            main_player_uuid: self.main_player_uuid,
            party_size: self.party_size,
            target_idx: self.target_idx,
        }
    }

    pub fn get_party_size_limit(&self) -> i32 {
        self.party_size.unwrap()
    }

    pub fn get_target_idx(&self) -> Option<i32> {
        self.target_idx
    }

    pub fn set_party_size_limit(&mut self, party_size: i32) {
        self.party_size = Some(party_size);
    }

    pub fn set_target_idx(&mut self, target_idx: i32) {
        self.target_idx = Some(target_idx);
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
        app.insert_resource(PlayerHandlerDatabaseCommands::get());

        let party = Party::new(
            self.main_player_email.as_ref().unwrap(), 
            self.main_player_user_name.as_ref().unwrap(),
            self.party_size.unwrap(),
        );
        app.insert_resource(party);

        // Add the startup protocol system
        app.add_systems(Startup, PlayerHandlerDatabaseCommands::start_up_protocol);
    }
}

#[derive(Resource)]
pub struct PlayerHandlerDatabaseCommands {}

#[derive(Clone, Debug)]
pub struct DBPlayer {
    uuid: String,
    email: String,
    user_name: String,
}

#[derive(Debug)]
pub enum ErrorType {
    AddPlayerFromDbToPartyFailedPlayerAlreadyInParty,
    AddPlayerFromDbToPartyFailedPlayerTestReference,
    DatabaseLockPoisoned,
    DBActionFailedPlayerCreation,
    DBActionFailedPlayerTableCreation,
    DBDeleteFailedPlayerTableDropAllRecords,
    DBActionFailedPlayerTableInsertRecordPlayerAiLocal,
    DBActionFailedPlayerTableInsertRecordPlayerAiRemote,
    DBActionFailedPlayerTableInsertRecordPlayerLocal,
    DBActionFailedPlayerTableInsertRecordPlayerMain,
    DBActionFailedPlayerTableInsertRecordPlayerRemote,
    DBActionFailedPlayerTableInsertRecordPlayerTestRef,
    DBDeleteFailedPlayerRecordFromPlayerTable,
    DBQueryFailedExistingPlayers,
    DBQueryFailedPlayerCount,
    DBQueryFailedPlayerTablePlayerMain,
    DBQueryFailedVerifyPlayerTableExists,
    DBQueryMappingFailedExistingPlayers,
    MatchTargetUuidToExistingPlayerInDatabaseNoPlayerAddedToPartyFailed,
    PartySizeAtSetLimit,
    PartySizeGreaterThanSetLimit,
    UuidParsingFailed,
}

#[derive(Resource)]
pub struct Party {
    pub active_player: i32,
    pub ai_vec: Option<Vec<usize>>,
    pub party_size: i32,
    pub players: Arc<Mutex<Vec<Arc<Mutex<dyn Player + Send>>>>>,
}

pub trait Player {
    fn new(player_email: Option<String>, player_user_name: Option<String>, player_type: PlayerType) -> Self where Self: Sized;
    fn get_player_email(&self) -> &Option<String>;
    fn get_player_id(&self) -> &Uuid;
    fn get_player_type(&self) -> &PlayerType;
    fn get_player_user_name(&self) -> &String;
    fn set_player_email(&mut self, new_email: &str);
    fn set_player_id(&mut self, new_id: Uuid);
    fn set_player_user_name(&mut self, new_user_name: &str);
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

#[derive(Clone, Resource)]
pub struct PlayerAi {
    player_email: Option<String>,
    player_id: Uuid,
    player_type: PlayerType,
    player_user_name: Option<String>,
}

#[derive(Clone, Resource)]
pub struct PlayerLocal {
    player_email: Option<String>,
    player_id: Uuid,
    player_type: PlayerType,
    player_user_name: Option<String>,
}

#[derive(Clone, Resource)]
pub struct PlayerRemote {
    player_email: Option<String>,
    player_id: Uuid,
    player_type: PlayerType,
    player_user_name: Option<String>,
}