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
        let player_table_exists = phi.query_db_table_player_exists(&db).unwrap();
        if !player_table_exists {
            phi.action_table_player_init(&db).unwrap();
        }

        // ----- [ Build main player ] ----- //

        let player_email = plugin.main_player_email.clone().unwrap();
        let player_username = plugin.main_player_username.clone().unwrap();

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
        // ----- [ Vertify database test ref and main player exists ] ----- //
    
        let players_test_ref_and_owner_exists = phi.query_db_player_count_less_than_2(&db).unwrap();
        
        if !players_test_ref_and_owner_exists {
            phi.pipeline_db_and_party_startup_test_ref_and_init_main_player(&db, &mut commands, &entity_player_query, &mut party, &player_query, &mut plugin).unwrap();
        }
    
        // ----- [ Sync party and database main players uuid ] ----- //
    
        let party_and_database_main_player_synced = phi.query_party_and_db_main_player_synced(&db, &mut party, &player_query).unwrap();
        if !party_and_database_main_player_synced {
            let player_mutex = player_query.single().player.lock().unwrap();
            let player_id = player_mutex.get_player_id().unwrap().clone();
            drop(player_mutex);
            party.player_map.insert(1, player_id);
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
        let party_size_limit = plugin.party_size.unwrap() as usize;
        let party_size = party.get_player_count_party(&player_query)?;
        if party_size == party_size_limit {
            return Err(ErrorTypePlayerHandler::PartySizeAtSetLimit)
        }
        if party_size > party_size_limit {
            return Err(ErrorTypePlayerHandler::PartySizeGreaterThanSetLimit)
        }
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