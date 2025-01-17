use bevy::prelude::*;

use bevy_easy_shared_definitions::{
    DatabaseConnection, 
    ErrorTypePlayerHandler,
};

use rusqlite::Result;
use uuid::Uuid;

use crate::{
    PlayerHandlerInterface, 
    PlayerType,
};

impl PlayerHandlerInterface {
    pub fn action_count_players_in_db(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<i32, ErrorTypePlayerHandler> {
        info!("Init: action_count_players_in_db:");
        
        // Get and Lock the mutex to access the database connection
        let conn = db.get_connection();
        let conn = conn.lock();
        let conn = match conn {
            Ok(conn) => conn,
            Err(_) => {
                error!("Database connection lock poisoned.");
                return Err(ErrorTypePlayerHandler::DatabaseLockPoisoned);
            }
        };
        
        // Execute the SQL statement to verify if the player table exists
        let party_player_count_query: i32 = conn.query_row(
            "SELECT COUNT(*) AS PartyPlayerCount FROM player_table;",
            (),
            |row| row.get(0),
        )
        .map_err(|e| ErrorTypePlayerHandler::DBQueryFailed(format!("Player Count failed, Error: [{}]", e)))?;

        info!("DB Players Count: [{:?}]", party_player_count_query);

        Ok(party_player_count_query)
    }

    pub fn action_table_player_init(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        info!("Init: action_table_player_init:");
        
        // Get and Lock the mutex to access the database connection
        let conn = db.get_connection();
        let conn = conn.lock();
        let conn = match conn {
            Ok(conn) => conn,
            Err(_) => {
                error!("Database connection lock poisoned.");
                return Err(ErrorTypePlayerHandler::DatabaseLockPoisoned);
            }
        };
        
        // Execute the SQL statement to create the player table
        conn.execute(
            "CREATE TABLE player_table (
                uuid TEXT PRIMARY KEY,
                email BLOB,
                username BLOB
            )",
            (),
        )
        .map_err(|e| ErrorTypePlayerHandler::DBActionFailed(format!("Player Table Creation Failed [{}]", e)))?; // Map any error to ErrorTypePlayerHandler and propagate it

        Ok(()) // Return success if the table is created without errors
    }

    pub fn action_insert_player_record(
        &self,
        db: &Res<DatabaseConnection>,
        main_player_uuid: &Uuid, 
        main_player_email: Option<&String>, 
        main_player_username: Option<&String>,
        player_type: PlayerType,
    ) -> Result<(), ErrorTypePlayerHandler> {
        info!("Init: action_insert_player_record");
        info!("Player [{}], UserName: [{:?}], Email: [{:?}], Player Type: [{:?}]", &main_player_uuid, &main_player_username, &main_player_email, &player_type);
        
        // Get and Lock the mutex to access the database connection
        println!("Step 1 [ action_insert_player_record ]");
        let conn = db.get_connection();
        println!("Step 2 [ action_insert_player_record ]");
        let conn = conn.lock();
        println!("Step 3 [ action_insert_player_record ]");
        let conn = match conn {
            Ok(conn) => conn,
            Err(_) => {
                println!("Error 1 [ action_insert_player_record ]");
                error!("Database connection lock poisoned.");
                return Err(ErrorTypePlayerHandler::DatabaseLockPoisoned);
            }
        };

        println!("Step 4 [ action_insert_player_record ]");
        let insert_target = String::from(*main_player_uuid);

        println!("Step 5 [ action_insert_player_record ]");
        println!("INSERT INTO player_table [ {} {} {} ]", &insert_target, &main_player_email.unwrap(), &main_player_username.unwrap());
        conn.execute(
            "INSERT INTO player_table (uuid, email, username) VALUES (?1, ?2, ?3)",
            (insert_target, main_player_email, main_player_username),
        )
            .map_err(|e| match player_type {
                PlayerType::PlayerAiLocal => ErrorTypePlayerHandler::DBActionFailed(format!("Action Insert Record Player Ai into 'player_table' failed Error: [{}]", e)),
                PlayerType::PlayerAiRemote => ErrorTypePlayerHandler::DBActionFailed(format!("Action Insert Record Remote Player Ai into 'player_table' failed Error: [{}]", e)),
                PlayerType::PlayerLocal => ErrorTypePlayerHandler::DBActionFailed(format!("Action Insert Record Player Local into 'player_table' failed Error: [{}]", e)),
                PlayerType::PlayerMain => ErrorTypePlayerHandler::DBActionFailed(format!("Action Insert Record Player Main into 'player_table' failed Error: [{}]", e)),
                PlayerType::PlayerRemote => ErrorTypePlayerHandler::DBActionFailed(format!("Action Insert Record Player Remote Local into 'player_table' failed Error: [{}]", e)),
                PlayerType::PlayerTestRef => ErrorTypePlayerHandler::DBActionFailed(format!("Action Insert Record Player Test Reference Local into 'player_table' failed Error: [{}]", e)),
            })?;
        println!("Success [ action_insert_player_record ]");
        Ok(())
    }

    pub fn action_remove_all_player_records(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        info!("Init: action_remove_all_player_records:");
        
        // Get and Lock the mutex to access the database connection
        let conn = db.get_connection();
        let conn = conn.lock();
        let conn = match conn {
            Ok(conn) => conn,
            Err(_) => {
                error!("Database connection lock poisoned.");
                return Err(ErrorTypePlayerHandler::DatabaseLockPoisoned);
            }
        };

        conn.execute(
            "DELETE FROM player_table",
            (),
        )
        .map_err(|e| ErrorTypePlayerHandler::DBActionFailed(format!("action_remove_all_player_records failed Error: [{}]", e)))?;

        Ok(())
    }
}