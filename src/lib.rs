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
        app.add_systems(Update, sync_plugin_party_main_player_uuid);
        app.add_systems(Update, PlayerHandlerInterface::start_up_protocol_finish.run_if(run_once()));
    }
}

pub fn sync_plugin_party_main_player_uuid(
    mut party: ResMut<Party>,
    plugin: Res<BevyEasyPlayerHandlerPlugin>
) {
    let new_uuid = match plugin.get_main_player_uuid() {
        Ok(uuid) => uuid.unwrap().to_owned(),
        Err(e) => {
            warn!("sync_plugin_party_main_player_uuid -> match plugin.get_main_player_uuid failed... [{:?}]", e); 
            match party.get_main_player_uuid() {
                Ok(uuid) => uuid,
                Err(_) => {
                    warn!("sync_plugin_party_main_player_uuid -> match plugin.get_main_player_uuid -> match party.get_main_player_uuid failed... [{:?}]", e); 
                    Uuid::now_v7()
                },
            }
        }
    };
    party.main_player_uuid = Some(new_uuid);
}

// System to trigger when PlayerComponent is spawned
pub fn on_player_component_spawned(
    mut party: ResMut<Party>,
    db: Res<DatabaseConnection>,
    entity_player_query: Query<(Entity, &PlayerComponent)>,
    listen_query: Query<Entity, Added<PlayerComponent>>,
    player_query: Query<&PlayerComponent>,
    phi: ResMut<PlayerHandlerInterface>,
) {
    let mut target_ent: Option<Entity> = None; 
    for entity in listen_query.iter() {
        println!("Init: on_player_component_spawned:");
        println!("Step 1 [ on_player_component_spawned ]");
        println!("PlayerComponent init: entity {:?}", entity);
        let target_idx = party.get_player_count_party(&player_query).unwrap();
        println!("Step 2 [ on_player_component_spawned ]");
        match party.set_active_player_index(target_idx, &player_query) {
            Ok(result) => result,
            Err(e) => {
                println!("Error 1 [ on_player_component_spawned ]");
                warn!("on_player_component_spawned -> party.active_player_set [{}] Error: {:?}", target_idx, e);
                ()
            },
        };
        println!("Step 3 [ on_player_component_spawned ]");
        target_ent = Some(entity);
    };
    if target_ent.is_some() {
        println!("Step 4 [ on_player_component_spawned ]");
        let target = target_ent.unwrap();
        println!("Step 5 [ on_player_component_spawned ]");
        let mut username: Option<String> = None;
        println!("Step 6 [ on_player_component_spawned ]");
        let mut user_uuid: Option<Uuid> = None;
        println!("Step 7 [ on_player_component_spawned ]");
        let mut player_type: Option<PlayerType> = None;
        println!("Step 8 [ on_player_component_spawned ]");
        for (entity, player) in entity_player_query.iter() {
            println!("Step 9 [ on_player_component_spawned ]");
            if &target == &entity {
                println!("Step 10 [ on_player_component_spawned ]");
                let player_data: Arc<Mutex<dyn Player + Send>> = player.player.clone();
                println!("Step 11 [ on_player_component_spawned ]");
                let player_mutex = player_data.lock().unwrap();

                println!("Step 12 [ on_player_component_spawned ]");
                let player_username = match player_mutex.get_player_username() {
                    Ok(email) => email.to_owned(),
                    Err(_) => {
                        println!("Error 2 [ on_player_component_spawned ]");
                        warn!("on_player_component_spawned -> match player_mutex.get_player_username Failed: Spawning Failed UserName");
                        String::from("Username Fetch Failed")
                    },
                };
                println!("Step 13 [ on_player_component_spawned ]");
                username = Some(player_username);

                println!("Step 14 [ on_player_component_spawned ]");
                let player_uuid = match player_mutex.get_player_id() {
                    Ok(uuid) => uuid.to_owned(),
                    Err(_) => {
                        println!("Error 3 [ on_player_component_spawned ]");
                        warn!("on_player_component_spawned -> match player_mutex.get_player_id Failed: Spawning random Uuid");
                        Uuid::now_v7()
                    },
                };
                println!("Step 15 [ on_player_component_spawned ]");
                user_uuid = Some(player_uuid);

                println!("Step 16 [ on_player_component_spawned ]");
                let p_type = match player_mutex.get_player_type() {
                    Ok(player_type) => player_type.to_owned(),
                    Err(_) => {
                        println!("Error 4 [ on_player_component_spawned ]");
                        warn!("on_player_component_spawned -> match player_mutex.get_player_type Failed: Spawning random Uuid");
                        PlayerType::PlayerLocal
                    },
                };
                println!("Step 17 [ on_player_component_spawned ]");
                player_type = Some(p_type);

                println!("Step 18 [ on_player_component_spawned ]");
                drop(player_mutex);

                println!("Step 19 [ on_player_component_spawned ]");
                let map_entry_exists = match party.verify_player_exists_player_map_uuid(&player_uuid) {
                    Ok(status) => status,
                    Err(e) => {
                        println!("Error 5 [ on_player_component_spawned ]");
                        warn!("Failed: on_player_component_spawned -> match party.players_add_player Error: [{:?}]", e);
                        false
                    },
                };
                println!("Step 20 [ on_player_component_spawned ]");
                if !map_entry_exists {
                    println!("Step 21 [ on_player_component_spawned ]");
                    let party_size = party.player_map.len();
                    println!("Step 22 [ on_player_component_spawned ]");
                    let party_size_plus_one = party_size + 1;
                    println!("Step 22 [ on_player_component_spawned ]");
                    party.player_map.insert(party_size_plus_one, player_uuid);
                }
            }
        }

        println!("Step 23 [ on_player_component_spawned ]");
        if username.is_none() {
            println!("Error 6 [ on_player_component_spawned ]");
            warn!("Failed: on_player_component_spawned -> username.is_none()");
        }

        println!("Step 24 [ on_player_component_spawned ]");
        if player_type.is_none() {
            println!("Error 7 [ on_player_component_spawned ]");
            warn!("Failed: on_player_component_spawned -> username.is_none()");
        }

        println!("Step 25 [ on_player_component_spawned ]");
        if user_uuid.is_none() {
            println!("Error 8 [ on_player_component_spawned ]");
            warn!("Failed: on_player_component_spawned -> username.is_none()");
        }

        // Get the new party size
        println!("Step 26 [ on_player_component_spawned ]");
        let party_size = match party.get_player_count_party(&player_query) {
            Ok(usize) => usize,
            Err(e) => {
                println!("Error 9 [ on_player_component_spawned ]");
                warn!("Failed: on_player_component_spawned -> match party.get_player_count_party Error: [{:?}]", e);
                0
            },
        };
        println!("Step 27 [ on_player_component_spawned ]");
        match party.set_active_player_index(party_size, &player_query) {
            Ok(status) => status,
            Err(e) => {
                println!("Error 10 [ on_player_component_spawned ]");
                warn!("Failed: on_player_component_spawned -> match party.set_active_player_index Error: [{:?}]", e);
                ()
            },
        };
        println!("Step 28 [ on_player_component_spawned ]");
        let username = username.unwrap(); 
        println!("Step 29 [ on_player_component_spawned ]");
        match phi.action_insert_player_record(&db, &user_uuid.unwrap(), Some(&username), Some(&username), player_type.unwrap()) {
            Ok(status) => status,
            Err(e) => {
                println!("Error 11 [ on_player_component_spawned ]");
                warn!("Failed: on_player_component_spawned -> match phi.action_insert_player_record Error: [{:?}]", e);
            },
        };
        println!("Success [ pipeline_db_and_party_add_new_synced_player_local ]");
        ()
    }
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
    pub main_player_uuid: Option<Uuid>,
    pub player_map: HashMap<usize, Uuid>,
}

pub trait Player { //  ->  
    // fn new(player_email: Option<String>, player_username: Option<String>) -> Self where Self: Sized;
    fn new(player_email: Option<String>, player_username: Option<String>, player_uuid: Option<Uuid>, player_type: PlayerType) -> Self where Self: Sized;
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
pub struct PlayerAiLocal {
    player_email: Option<String>,
    player_uuid: Uuid,
    player_type: PlayerType,
    player_username: Option<String>,
}

#[derive(Clone, Component, Debug, Resource)]
pub struct PlayerAiRemote {
    player_email: Option<String>,
    player_uuid: Uuid,
    player_type: PlayerType,
    player_username: Option<String>,
}

#[derive(Clone, Component, Debug, Resource)]
pub struct PlayerLocal {
    player_email: Option<String>,
    player_uuid: Uuid,
    player_type: PlayerType,
    player_username: Option<String>,
}

#[derive(Clone, Component, Debug, Resource)]
pub struct PlayerMain {
    player_email: Option<String>,
    player_uuid: Uuid,
    player_type: PlayerType,
    player_username: Option<String>,
}

#[derive(Clone, Component, Debug, Resource)]
pub struct PlayerRemote {
    player_email: Option<String>,
    player_uuid: Uuid,
    player_type: PlayerType,
    player_username: Option<String>,
}