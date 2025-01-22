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

pub mod prelude {
    pub use crate::{
        BevyEasyPlayerHandlerPlugin,
        Party,
        PlayerHandlerInterface,
        PlayerComponent,
    };
}

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
        app.add_systems(Update, on_player_component_removal);
        app.add_systems(Update, sync_plugin_party_main_player_uuid);
        app.add_systems(Update, PlayerHandlerInterface::start_up_protocol_finish.run_if(run_once()));
    }
}

// System to trigger when PlayerComponent is spawned
fn on_player_component_removal(
    mut commands: Commands,
    db: Res<DatabaseConnection>,
    mut party: ResMut<Party>,
    phi: ResMut<PlayerHandlerInterface>,
    player_query: Query<&PlayerComponent>,
    mut removed: RemovedComponents<PlayerComponent>,
) {
    let mut removed_event: bool = false;
    for _ in removed.read() {
        removed_event = true;
    }
    if removed_event {
        let index_minus_one = match party.get_active_player_index() {
            Ok(value) => value - 1,
            Err(_) => {
                panic!("on_player_component_removal -> match party.get_active_player_index failed...");
            },
        };
        match party.set_active_player_index(index_minus_one) {
            Ok(()) => (),
            Err(_) => warn!("on_player_component_removal -> party.set_active_player_index(index_minus_one) failed..."),
        };
        let player_map_ids = match party.get_player_map_clone() {
            Ok(result) => result,
            Err(_) => {
                panic!("on_player_component_removal -> match party.get_player_map_clone failed...");
            },
        };
        println!("player_map_ids [{:?}]", &player_map_ids);
        let party_size = player_map_ids.len();
        println!("party_size [{:?}]", &party_size);
        let mut player_vec_ids: Vec<(usize, Uuid)> = Vec::new();
        for n in 0..party_size {
            let ref_uuid = match player_map_ids.get(&( n + 1 )) {
                Some(value) => value.to_owned(),
                None => {
                    panic!("on_player_component_removal -> match player_map_ids.get(&n) failed...");
                },
            };
            println!("loop player_vec_ids [{}]::[{}]", ( n + 1 ), &ref_uuid);
            player_vec_ids.push((( n + 1 ), ref_uuid));
        }
        for player in player_query.iter() {
            let player_mutex = match player.player.lock() {
                Ok(result) => result,
                Err(_) => {
                    panic!("on_player_component_removal -> match match player.player.lock failed...");
                },
            };
            let player_uuid = match player_mutex.get_player_id() {
                Ok(value) => value.to_owned(),
                Err(_) => {
                    panic!("on_player_component_removal -> match player_mutex.get_player_id() failed...");
                },
            };
            println!("before: player_vec_ids [{:?}]", &player_vec_ids);
            player_vec_ids = player_vec_ids
                .into_iter()
                .filter_map(|(usize, player)| 
                    if player == player_uuid {
                        return None; 
                    } else {
                        return Some((usize, player));
                    } 
                )
                .collect();
            println!("after: player_vec_ids [{:?}]", &player_vec_ids);
        }
        let main_player_uuid = match party.get_main_player_uuid() {
            Ok(result) => result,
            Err(_) => {
                warn!("on_player_component_removal -> match party.get_main_player_uuid failed...");
                let uuid: Uuid = Uuid::now_v7();
                Some(uuid)
            },
        };
        for player in player_vec_ids.iter() {
            if Some(player.1) != main_player_uuid {
                party.player_map.remove(&player.0);
            } else {
                match phi.pipeline_db_and_party_add_main_player_from_db_to_party(&mut commands, &db, &player.1) {
                    Ok(()) => (),
                    Err(_) => {
                        warn!("on_player_component_removal -> match party.get_main_player_uuid failed...");
                    },
                }
            }
        }
    }
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
        println!("PlayerComponent init: entity {:?}", entity);
        let target_idx = party.get_player_count_party(&player_query).unwrap();
        match party.set_active_player_index(target_idx) {
            Ok(result) => result,
            Err(e) => {
                warn!("on_player_component_spawned -> party.active_player_set [{}] Error: {:?}", target_idx, e);
                ()
            },
        };
        target_ent = Some(entity);
    };
    if target_ent.is_some() {
        let target = target_ent.unwrap();
        let mut username: Option<String> = None;
        let mut user_uuid: Option<Uuid> = None;
        let mut player_type: Option<PlayerType> = None;
        for (entity, player) in entity_player_query.iter() {
            if &target == &entity {
                let player_data: Arc<Mutex<dyn Player + Send>> = player.player.clone();
                let player_mutex = player_data.lock().unwrap();

                let player_username = match player_mutex.get_player_username() {
                    Ok(email) => email.to_owned(),
                    Err(_) => {
                        warn!("on_player_component_spawned -> match player_mutex.get_player_username Failed: Spawning Failed UserName");
                        String::from("Username Fetch Failed")
                    },
                };
                username = Some(player_username);

                let player_uuid = match player_mutex.get_player_id() {
                    Ok(uuid) => uuid.to_owned(),
                    Err(_) => {
                        warn!("on_player_component_spawned -> match player_mutex.get_player_id Failed: Spawning random Uuid");
                        Uuid::now_v7()
                    },
                };
                user_uuid = Some(player_uuid);

                let p_type = match player_mutex.get_player_type() {
                    Ok(player_type) => player_type.to_owned(),
                    Err(_) => {
                        warn!("on_player_component_spawned -> match player_mutex.get_player_type Failed: Spawning random Uuid");
                        PlayerType::PlayerLocal
                    },
                };
                player_type = Some(p_type);

                drop(player_mutex);

                let map_entry_exists = match party.verify_player_exists_player_map_uuid(&player_uuid) {
                    Ok(status) => status,
                    Err(e) => {
                        warn!("Failed: on_player_component_spawned -> match party.players_add_player Error: [{:?}]", e);
                        false
                    },
                };
                if !map_entry_exists {
                    let party_size = party.player_map.len();
                    let party_size_plus_one = party_size + 1;
                    party.player_map.insert(party_size_plus_one, player_uuid);
                }
            }
        }

        if username.is_none() {
            warn!("Failed: on_player_component_spawned -> username.is_none()");
        }

        if player_type.is_none() {
            warn!("Failed: on_player_component_spawned -> username.is_none()");
        }

        if user_uuid.is_none() {
            warn!("Failed: on_player_component_spawned -> username.is_none()");
        }

        let party_size = match party.get_player_count_party(&player_query) {
            Ok(usize) => usize,
            Err(e) => {
                warn!("Failed: on_player_component_spawned -> match party.get_player_count_party Error: [{:?}]", e);
                0
            },
        };
        match party.set_active_player_index(party_size) {
            Ok(status) => status,
            Err(e) => {
                warn!("Failed: on_player_component_spawned -> match party.set_active_player_index Error: [{:?}]", e);
                ()
            },
        };
        let username = username.unwrap(); 
        match phi.action_insert_player_record(&db, &user_uuid.unwrap(), Some(&username), Some(&username), player_type.unwrap()) {
            Ok(status) => status,
            Err(e) => {
                warn!("Failed: on_player_component_spawned -> match phi.action_insert_player_record Error: [{:?}]", e);
            },
        };
        ()
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
                Ok(uuid) => uuid.unwrap(),
                Err(_) => {
                    warn!("sync_plugin_party_main_player_uuid -> match plugin.get_main_player_uuid -> match party.get_main_player_uuid failed... [{:?}]", e); 
                    Uuid::now_v7()
                },
            }
        }
    };
    party.main_player_uuid = Some(new_uuid);
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
    fn new(player_email: Option<String>, player_username: Option<String>, player_uuid: Option<Uuid>, player_type: PlayerType) -> Self where Self: Sized;
    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler>;
    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler>;
    fn get_player_type(&self) -> Result<&PlayerType, ErrorTypePlayerHandler>;
    fn get_player_username(&self) -> Result<&String, ErrorTypePlayerHandler>;
    fn set_player_email(&mut self, new_email: &str) -> Result<(), ErrorTypePlayerHandler>;
    fn set_player_id(&mut self, new_id: Uuid) -> Result<(), ErrorTypePlayerHandler>;
    fn set_player_username(&mut self, new_username: &str) -> Result<(), ErrorTypePlayerHandler>;
}

#[derive(Clone, Component)]
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