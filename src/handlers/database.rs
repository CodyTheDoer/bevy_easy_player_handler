use bevy::prelude::*;

use bevy_easy_shared_definitions::ErrorTypePlayerHandler;

use crate::{
    BevyEasyPlayerHandlerPlugin, 
    DatabaseConnection,
    DBPlayer, 
    Party, 
    Player,
    PlayerComponent,
    PlayerMain,
    PlayerType,
    PlayerHandlerInterface, 
};

use std::sync::Arc;
use std::sync::Mutex;
use rusqlite::Result;

impl PlayerHandlerInterface {
    pub fn get() -> Self {
        PlayerHandlerInterface {}
    }

    pub fn start_up_protocol(
        mut commands: Commands,
        db: Res<DatabaseConnection>,
        phi: ResMut<PlayerHandlerInterface>,
        plugin: ResMut<BevyEasyPlayerHandlerPlugin>,
    ) {
        info!("BevyEasyPlayerHandler: [ Start Up Protocol ]");
    
        let player_table_exists = phi.query_db_table_player_exists(&db).unwrap();
    
        info!("Result [query_table_player_exists]: [{}]", player_table_exists);
    
        if !player_table_exists {
            phi.action_table_player_init(&db).unwrap();
        }

        // ----- [ Build main player ] ----- //

        let player_email = plugin.main_player_email.clone().unwrap();
        let player_username = plugin.main_player_username.clone().unwrap();
        info!("Spawning [ main_player ]: Username [{}]: Email [{}]", player_email, player_username);

        let player_component = PlayerMain::new(
            Some(player_email), 
            Some(player_username),
            None,
            PlayerType::PlayerMain,
        );

        let player: Arc<Mutex<dyn Player + Send>> = Arc::new(Mutex::new(player_component));
        commands.spawn(PlayerComponent{player: player});
    }

    pub fn start_up_protocol_finish(
        mut commands: Commands,
        db: Res<DatabaseConnection>,
        entity_player_query: Query<(Entity, &PlayerComponent)>, 
        mut party: ResMut<Party>,
        phi: ResMut<PlayerHandlerInterface>,
        player_query: Query<&PlayerComponent>,
        mut plugin: ResMut<BevyEasyPlayerHandlerPlugin>,
    ) {
        println!("Init: start_up_protocol_finish:");
    
        // ----- [ Vertify database test ref and main player exists ] ----- //
    
        println!("Step: 1 [ start_up_protocol_finish ]");
        let players_test_ref_and_owner_exists = phi.query_db_test_ref_and_main_player_exists(&db).unwrap();
        
        info!("Result [query_test_ref_and_main_player_exists]: [{}]", players_test_ref_and_owner_exists);
    
        println!("Step: 2 [ start_up_protocol_finish ]");
        if !players_test_ref_and_owner_exists {
            println!("Step: 3 [ start_up_protocol_finish ]");
            phi.pipeline_db_and_party_startup_test_ref_and_init_main_player(&db, &mut commands, &entity_player_query, &mut party, &player_query, &mut plugin).unwrap();
        }
    
        // ----- [ Sync party and database main players uuid ] ----- //
    
        println!("Step: 4 [ start_up_protocol_finish ]");
        let party_and_database_main_player_synced = phi.query_party_and_db_main_player_synced(&db, &mut party, &player_query).unwrap();
        
        info!("Result [query_party_and_db_main_player_synced]: [{}]", party_and_database_main_player_synced);
    
        println!("Step: 5 [ start_up_protocol_finish ]");
        if !party_and_database_main_player_synced {
            println!("Step: 6 [ start_up_protocol_finish ]");
            let player_mutex = player_query.single().player.lock().unwrap();
            println!("Step: 7 [ start_up_protocol_finish ]");
            let player_id = player_mutex.get_player_id().unwrap().clone();
            drop(player_mutex);
            println!("Step: 8 [ start_up_protocol_finish ]");
            info!("Result [player_map -> pre insert]: {:?}", &party.player_map);
            party.player_map.insert(1, player_id);
            info!("Result [player_map -> post insert]: {:?}", &party.player_map);

            println!("Step: 9 [ start_up_protocol_finish ]");
            match phi.pipeline_db_and_party_sync_main_player_uuids(&db, &mut party, &player_query, plugin) {
                Ok(sync) => sync,
                Err(e) => {
                    warn!("start_up_protocol_finish -> pipeline_db_and_party_sync_main_player_uuids [ Failed ] Error: {:?}", e);
                }, 
            };
        }
    }

    // --- Internal Helper Functions --- //

    pub fn verify_if_party_size_exceeds_limit( 
        &self,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
        player_query: &Query<&PlayerComponent>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        info!("Init: verify_if_party_size_exceeds_limit:");

        info!("Step 1 [ verify_if_party_size_exceeds_limit ]");
        // Party Size Management Checks
        let party_size_limit = plugin.party_size.unwrap() as usize;
        info!("Step 2 [ verify_if_party_size_exceeds_limit ]");
        let party_size = party.get_player_count_party(&player_query)?;

        info!("Step 3 [ verify_if_party_size_exceeds_limit ]");
        if party_size == party_size_limit {
            info!("Error 1 [ verify_if_party_size_exceeds_limit ]");
            return Err(ErrorTypePlayerHandler::PartySizeAtSetLimit)
        }
        info!("Step 4 [ verify_if_party_size_exceeds_limit ]");
        if party_size > party_size_limit {
            info!("Errro 1 [ verify_if_party_size_exceeds_limit ]");
            return Err(ErrorTypePlayerHandler::PartySizeGreaterThanSetLimit)
        }
        info!("Success [ verify_if_party_size_exceeds_limit ]");
        Ok(())
    }
}

impl DBPlayer {
    pub fn get_uuid_string(&self) -> &String {
        &self.uuid
    }
    pub fn get_email_string(&self) -> &String {
        &self.email
    }
    pub fn get_username_string(&self) -> &String {
        &self.username
    }
}