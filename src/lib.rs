use bevy::prelude::*;

use bevy_easy_shared_definitions::{
    DatabaseConnection, 
    ErrorTypePlayerHandler,
};

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use uuid::Uuid;

pub mod database;
pub mod handlers;

#[derive(Clone, Resource)]
pub struct BevyEasyPlayerHandlerPlugin {
    main_player_email: Option<String>,
    main_player_username: Option<String>,
    main_player_uuid: Option<Uuid>,
    party_size: Option<usize>,
}

impl BevyEasyPlayerHandlerPlugin {
    pub fn init() -> Self {
        BevyEasyPlayerHandlerPlugin {
            main_player_email: None,
            main_player_username: None,
            main_player_uuid: None,
            party_size: None,
        }
    }

    pub fn main_player_email(mut self, main_player_email: &str) -> Self {
        self.main_player_email = Some(String::from(main_player_email));
        self
    }

    pub fn main_player_username(mut self, main_player_username: &str) -> Self {
        self.main_player_username = Some(String::from(main_player_username));
        self
    }

    pub fn main_player_uuid(mut self, main_player_uuid: &Uuid) -> Self {
        self.main_player_uuid = Some(*main_player_uuid);
        self
    }

    pub fn party_size(mut self, party_size: usize) -> Self {
        self.party_size = Some(party_size);
        self
    }

    pub fn build(mut self) -> BevyEasyPlayerHandlerPlugin {
        if self.main_player_uuid.is_none() {
            self.main_player_uuid = Some(Uuid::now_v7());
        }
        
        if self.main_player_username.is_none() {
            self.main_player_username = Some(String::from(self.main_player_uuid.unwrap()));
        }
        
        let party_size = match self.party_size {
            Some(usize) => usize,
            None => 1,
        };
        self.party_size = Some(party_size);

        Self {
            main_player_email: self.main_player_email,
            main_player_username: self.main_player_username,
            main_player_uuid: self.main_player_uuid,
            party_size: self.party_size,
        }
    }

    pub fn get_main_player_email(&self) -> Result<Option<&String>, ErrorTypePlayerHandler> {
        Ok(self.main_player_email.as_ref())
    }

    pub fn get_main_player_username(&self) -> Result<Option<&String>, ErrorTypePlayerHandler> {
        Ok(self.main_player_username.as_ref())
    }

    pub fn get_main_player_uuid(&self) -> Result<Option<&Uuid>, ErrorTypePlayerHandler> {
        Ok(self.main_player_uuid.as_ref())
    }

    pub fn get_party_size_limit(&self) -> Result<Option<&usize>, ErrorTypePlayerHandler> {
        Ok(self.party_size.as_ref())
    }

    pub fn set_party_size_limit(&mut self, party_size: usize) -> Result<(), ErrorTypePlayerHandler> {
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

        // Insert the plugin itself and other resources into the host app
        app.insert_resource(self.clone());
        app.insert_resource(PlayerHandlerInterface::get());
        app.insert_resource(Party::new());

        // Add the startup protocol system
        app.add_systems(Startup, PlayerHandlerInterface::start_up_protocol);
        app.add_systems(Update, on_player_component_spawned);
        app.add_systems(Update, PlayerHandlerInterface::start_up_protocol_finish.run_if(run_once()));
    }
}

// System to trigger when MyComponent is added to an entity
pub fn on_player_component_spawned(
    mut party: ResMut<Party>,
    listen_query: Query<Entity, Added<PlayerComponent>>,
    party_query: Query<&PlayerComponent>,
) {
    // let mut target_ent: Option<Entity> = None; 
    for entity in listen_query.iter() {
        // target_ent = Some(entity.clone());
        println!("PlayerComponent init: entity {:?}", entity);
        let target_idx = party.get_player_count_party(&party_query).unwrap();
        match party.set_active_player_index(target_idx) {
            Ok(result) => result,
            Err(e) => {
                warn!("on_player_component_spawned -> party.active_player_set [{}] Error: {:?}", target_idx, e);
                ()
            },
        };
        // self.active_player_set_uuid(existing_uuid.to_owned());
    };
    // if target_ent.is_some() {
    //     let target = target_ent.unwrap();
    //     for (entity, player) in target_query.iter() {
    //         if target == entity {
    //             let player_data: Arc<Mutex<dyn Player + Send>> = player.player.clone();
    //             let player_lock = player_data.lock().unwrap();
                
    //             let player_email = match player_lock.get_player_email() {
    //                 Ok(email) => email.to_owned(),
    //                 Err(_) => {
    //                     warn!("on_player_component_spawned -> match player_data.get_player_email Failed: Spawning Failed Email");
    //                     String::from("Email Fetch Failed")
    //                 },
    //             };
    //             let player_username = match player_lock.get_player_username() {
    //                 Ok(email) => email.to_owned(),
    //                 Err(_) => {
    //                     warn!("on_player_component_spawned -> match player_data.get_player_username Failed: Spawning Failed UserName");
    //                     String::from("Username Fetch Failed")
    //                 },
    //             };
    //             let player_uuid = match player_lock.get_player_id() {
    //                 Ok(uuid) => uuid.to_owned(),
    //                 Err(_) => {
    //                     warn!("on_player_component_spawned -> player_data.get_player_id Failed: Spawning random Uuid");
    //                     Uuid::now_v7()
    //                 },
    //             };

    //             let already_exists = match party.has_player_with_id(&player_uuid){
    //                 Ok(success) => success,
    //                 Err(e) => {
    //                     warn!("Failed: on_player_component_spawned -> match party.players_add_player Error: [{:?}]", e);
    //                     false
    //                 },
    //             };
    //             if !already_exists {
    //                 let target_player_data: Arc<Mutex<dyn Player + Send>> = Arc::new(Mutex::new(PlayerLocal::new(Some(player_email), Some(player_username), PlayerType::PlayerLocal)));
    //                 match party.players_add_player(target_player_data) {
    //                     Ok(success) => success,
    //                     Err(e) => {
    //                         warn!("Failed: on_player_component_spawned -> match party.players_add_player Error: [{:?}]", e);
    //                         ()
    //                     },
    //                 };
    //             };
    //         }
    //     }
    // }
}

#[derive(Resource)]
pub struct PlayerHandlerInterface {}

#[derive(Clone, Debug)]
pub struct DBPlayer {
    pub uuid: String,
    pub email: String,
    pub username: String,
}

#[derive(Resource)]
pub struct Party {
    pub active_player: usize,
    pub player_map: HashMap<usize, Uuid>,
}

pub trait Player { //  ->  
    // fn new(player_email: Option<String>, player_username: Option<String>) -> Self where Self: Sized;
    fn new(player_email: Option<String>, player_username: Option<String>, player_type: PlayerType) -> Self where Self: Sized;
    fn clone_with_new_id(&self) -> Result<Arc<Mutex<dyn Player + Send>>, ErrorTypePlayerHandler>;
    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler>;
    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler>;
    fn get_player_type(&self) -> Result<&PlayerType, ErrorTypePlayerHandler>;
    fn get_player_username(&self) -> Result<&String, ErrorTypePlayerHandler>;
    fn set_player_email(&mut self, new_email: &str) -> Result<(), ErrorTypePlayerHandler>;
    fn set_player_id(&mut self, new_id: Uuid) -> Result<(), ErrorTypePlayerHandler>;
    fn set_player_username(&mut self, new_username: &str) -> Result<(), ErrorTypePlayerHandler>;
}

#[derive(Component)]
pub struct PlayerComponent {
    pub player: Arc<Mutex<dyn Player + Send>>,
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

#[derive(Clone, Component, Debug, Resource)]
pub struct PlayerAi {
    player_email: Option<String>,
    player_id: Uuid,
    player_type: PlayerType,
    player_username: Option<String>,
}

#[derive(Clone, Component, Debug, Resource)]
pub struct PlayerLocal {
    player_email: Option<String>,
    player_id: Uuid,
    player_type: PlayerType,
    player_username: Option<String>,
}

#[derive(Clone, Component, Debug, Resource)]
pub struct PlayerMain {
    player_email: Option<String>,
    player_id: Uuid,
    player_type: PlayerType,
    player_username: Option<String>,
}

#[derive(Clone, Component, Debug, Resource)]
pub struct PlayerRemote {
    player_email: Option<String>,
    player_id: Uuid,
    player_type: PlayerType,
    player_username: Option<String>,
}