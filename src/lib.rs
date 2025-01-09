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

    pub fn default(mut self) -> Self {
        if self.main_player_uuid.is_none() {
            self.main_player_uuid = Some(Uuid::now_v7());
        }
        if self.main_player_user_name.is_none() {
            self.main_player_user_name = Some(String::from(self.main_player_uuid.unwrap()));
        }
        if self.party_size.is_none() {
            self.party_size = Some(1);
        }
        self
    }

    pub fn build(self) -> BevyEasyPlayerHandlerPlugin {
        Self {
            main_player_email: self.main_player_email,
            main_player_user_name: self.main_player_user_name,
            main_player_uuid: self.main_player_uuid,
            party_size: self.party_size,
        }
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
        app.add_systems(Startup, start_up_protocol);
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
    DatabaseLockPoisoned,
    DBActionFailedPlayerTableInsertRecordPlayer,
    DBDeleteFailedPlayerTableDropAllRecords,
    DBActionFailedPlayerCreation,
    DBActionFailedPlayerTableCreation,
    DBActionFailedPlayerTableInsertRecordTestRef,
    DBDeleteFailedPlayerRecordFromPlayerTable,
    DBQueryFailedExistingPlayers,
    DBQueryFailedPlayerCount,
    DBQueryFailedPlayerTablePlayerMain,
    DBQueryFailedVerifyPlayerTableExists,
    DBQueryMappingFailedExistingPlayers,
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
    fn new(player_email: Option<String>, player_user_name: Option<String>) -> Self where Self: Sized;
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
    PlayerAi,
    PlayerLocal,
    PlayerRemote,
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

pub fn start_up_protocol(
    db: Res<DatabaseConnection>,
    dbi: Res<PlayerHandlerDatabaseCommands>,
    plugin: Res<BevyEasyPlayerHandlerPlugin>,
    mut party: ResMut<Party>,
) {
    info!("BevyEasyPlayerHandler: [ Start Up Protocol ]");

    // ----- [ Vertify database table "player_table" exists ] ----- //

    let player_table_exists = match dbi.query_table_player_exists(&db) {
        Ok(exists) => {
            exists
        },
        Err(e) => {
            warn!("Error: start_up_protocol -> query_table_player_exists: {:?}", e);
            false // Assume the table does not exist on unexpected errors
        }
    };

    info!("Result [query_table_player_exists]: [{}]", player_table_exists);

    if !player_table_exists {
        match dbi.action_table_player_init(&db) {
            Ok(_) => info!("Table: 'player_table' created successfully!"),
            Err(ErrorType::DBActionFailedPlayerTableCreation) => warn!("Error: Failed to create 'player_table'..."),
            Err(e) => warn!("Error: start_up_protocol -> action_table_player_init: {:?}", e)
        };
    }

    // ----- [ Vertify database test ref and main player exists ] ----- //

    let players_test_ref_and_owner_exists = match dbi.query_test_ref_and_main_player_exists(&db) {
        Ok(does_exist) => {
            does_exist
        },
        Err(e) => {
            warn!("Error: start_up_protocol -> query_test_ref_and_main_player_exists: {:?}", e);
            false // Assume the test ref and main players do not exist on unexpected errors
        }
    };

    info!("Result [query_test_ref_and_main_player_exists]: [{}]", players_test_ref_and_owner_exists);

    if !players_test_ref_and_owner_exists {
        match dbi.pipeline_init_test_ref_and_main_player(&db, plugin, &mut party) {
            Ok(_) => info!("Database: Records [ test_ref ] & [ main_player ] created successfully!"),
            Err(ErrorType::DBActionFailedPlayerCreation) => warn!("Error: Failed to create 'player'..."),
            Err(e) => warn!("Error: start_up_protocol -> init_test_ref_and_main_player: {:?}", e)
        }
    }

    // ----- [ Sync party and database main players uuid ] ----- //

    let party_and_database_main_player_synced = match dbi.query_party_and_db_main_player_synced(&db, &mut party) {
        Ok(synced) => {
            synced
        },
        Err(e) => {
            warn!("Error: start_up_protocol -> query_party_and_db_main_player_synced: {:?}", e);
            false // Assume the table does exist on unexpected errors
        }
    };
    
    info!("Result [query_party_and_db_main_player_synced]: [{}]", party_and_database_main_player_synced);

    if !party_and_database_main_player_synced {
        match dbi.action_sync_party_and_db_main_player(&db, &mut party) {
            Ok(_) => info!("Party: Database and Party [ main_player ] synced successfully!"),
            Err(ErrorType::DBQueryFailedPlayerTablePlayerMain) => warn!("Error: Failed to query main player..."),
            Err(ErrorType::UuidParsingFailed) => warn!("Error: Failed to parse uuid..."),
            Err(e) => warn!("Error: start_up_protocol -> action_sync_party_and_db_main_player: {:?}", e)
        }
    }
}