use bevy::prelude::*;

use bevy_easy_player_handler::{
    BevyEasyPlayerHandlerPlugin,
    Party,
    PlayerHandlerInterface,
    PlayerComponent,
};

use bevy_easy_shared_definitions::{
    ErrorTypePlayerHandler,
    DatabaseConnection,
};

use bevy_easy_vec_ui::{
    BevyEasyVecUiPlugin, 
    EasyVecUi,
};

use dotenv::dotenv;
use std::env;
use uuid::Uuid;

fn main() {
    // Establish the Database Connection and bring dotenv into the equation
    let db = DatabaseConnection::new("game_data.db");
    dotenv().ok();

    // Build the main app
    let mut app: App = App::new();    
    app.add_plugins(DefaultPlugins)
        .insert_resource(db)
        .add_plugins(BevyEasyVecUiPlugin::init("fonts/MatrixtypeDisplay-KVELZ.ttf")
            .camera_layer(-1)
            .title("Easy Player Handler Integration Example")
            .title_font_size(42.0)
            .data_font_size(12.0)
            .build()
        )
        .add_plugins(BevyEasyPlayerHandlerPlugin::init()
            .main_player_email(env::var("PLAYER_EMAIL").unwrap().as_str())
            .main_player_username(env::var("PLAYER_USERNAME").unwrap().as_str())
            .party_size(6)
            .build()
        )
        .insert_resource(EasyVecUiTimer(Timer::from_seconds(0.250, TimerMode::Repeating)))
        .insert_resource(DisplayInts::init())
        .add_systems(Update, update_timer)
        .add_systems(Update, listen_easy_vec_ui_timer.run_if(timer_finished))
        .add_systems(Update, temp_interface)
        .run();
}

#[derive(Resource)]
pub struct EasyVecUiTimer(pub Timer);

impl EasyVecUiTimer {
    pub fn new(duration: f32) -> Self {
      Self(Timer::from_seconds(duration, TimerMode::Repeating))
    }
}

impl Default for EasyVecUiTimer {
    fn default() -> Self {
        dotenv().ok();
        let duration = match env::var("EASY_VEC_UI_TIMER_DURATION").unwrap().parse() {
            Ok(i32) => i32,
            Err(e) => {
                warn!("Error: [ dotenv ] EASY_VEC_UI_TIMER_DURATION: [{}] Defaulting to [ 10.0 ] Seconds", e);
                10.0
            },
        };
        Self::new(duration)
    }
}

// --- lib.rs --- //
#[derive(Resource)]
pub struct DisplayInts {
    db_target_int: Option<i32>,
}

impl DisplayInts {
    pub fn init() -> Self {
        DisplayInts {
            db_target_int: Some(0),
        }
    }

    pub fn get_db_target_int(&self) -> Result<i32, ErrorType> {
        let result = self.db_target_int.unwrap();
        Ok(result)
    }

    pub fn set_db_target_int(&mut self, target: i32) -> Result<(), ErrorType> {
        self.db_target_int = Some(target);
        let result = self.db_target_int.unwrap();
        if result != target {
            return Err(ErrorType::DataUpdateFailed(format!("set_db_target_int({}) Failed: target did not update properly... Current Value: [{}]", target, result)))
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum ErrorType {
    DataUpdateFailed(String),
}

pub fn update_timer(mut ui_timer: ResMut<EasyVecUiTimer>, time: Res<Time>) {
    ui_timer.0.tick(time.delta());
}

pub fn timer_finished(ui_timer: Res<EasyVecUiTimer>) -> bool {
    ui_timer.0.finished()
}

pub fn listen_easy_vec_ui_timer(
    mut ui_timer: ResMut<EasyVecUiTimer>,
    easy_vec_ui_resource: ResMut<EasyVecUi>,
    db: Res<DatabaseConnection>,
    dbi: Res<PlayerHandlerInterface>,
    di: Res<DisplayInts>,
    party: Res<Party>,
    player_query: Query<&PlayerComponent>,
    plugin: Res<BevyEasyPlayerHandlerPlugin>,
) {
    if ui_timer.0.finished() {
        // info!("[ listen_easy_vec_ui_timer ]: UiTimer = finished...");
        match easy_vec_ui(easy_vec_ui_resource, db, dbi, di, party, player_query, plugin) {
            Ok(()) => {},
            Err(e) => {
                warn!("Error: listen_easy_vec_ui_timer -> easy_vec_ui: [{:?}]", e);
            }
        };
    }
    ui_timer.0.reset(); // Reset the timer after handling logic
}

// --- end lib.rs --- //
//-----------------------------------------------------------------------------------//

pub fn easy_vec_ui(
    mut easy_vec_ui_resource: ResMut<EasyVecUi>,
    db: Res<DatabaseConnection>,
    dbi: Res<PlayerHandlerInterface>,
    di: Res<DisplayInts>,
    party: Res<Party>,
    player_query: Query<&PlayerComponent>,
    plugin: Res<BevyEasyPlayerHandlerPlugin>,
) -> Result<(), ErrorTypePlayerHandler> {

    // --- Uuid aggrigation --- //
    let player_vec = dbi.query_db_existing_players(&db)?;

    let mut player_uuid_string_vec: Vec<Uuid> = Vec::new();
    for player in player_vec.clone().iter() {
        let uuid_string = player.get_uuid_string().as_str();
        let uuid = match Uuid::try_parse(uuid_string) {
            Ok(uuid) => uuid,
            Err(e) => {
                return Err(ErrorTypePlayerHandler::UuidParsingFailed(e.to_string()));
            },
        };
        player_uuid_string_vec.push(uuid);
    }

    // --- Left Data Vec --- //    
    let mut left_data_vec = vec![];
    let new_line_left_line = "\n_______________________________________________________________________________________________________________________________";

    // --- Local Game Owner information --- //
    left_data_vec.push(format!(
        "Main Player: [ {:?} ], Email: [ {:?} ], UserName: [ {:?} ]{}{}",
        plugin.get_main_player_uuid().unwrap().unwrap(), 
        plugin.get_main_player_email().unwrap().unwrap(), 
        plugin.get_main_player_username().unwrap().unwrap(),
        &new_line_left_line, 
        &new_line_left_line, 
    ));


    // --- Players in Database: information --- //
    for db_player in player_vec.iter() {
        left_data_vec.push(String::from(format!(
            "DB Record: [ {} ], Email: [ {:?} ], UserName: [ {:?} ]{}", 
            db_player.get_uuid_string(), 
            db_player.get_email_string(), 
            db_player.get_username_string(),
            &new_line_left_line,
        )));
    }

    easy_vec_ui_resource.inject_vec_left(left_data_vec);

    // --- Right Data Vec --- //    
    let mut right_data_vec = vec![];
    let new_line_right_line = "\n________________________________________________________________________________________________________________";

    // --- Party information --- //
    for player in player_query.iter() {
        let player_data = player.player.lock().unwrap();
        let player_uuid = player_data.get_player_id()?;
        let player_username = player_data.get_player_username()?;
        let player_type = player_data.get_player_type()?;
        right_data_vec.push(format!(
            "Querried '&PlayerComponent': [ {} ]\n
            Name: [ {} ], Type: [ {:?} ]{}", 
            player_uuid,
            player_username,
            player_type,
            &new_line_right_line, 
        ));

    }

    right_data_vec.push(String::from(new_line_right_line));

    right_data_vec.push(format!(
        "Party Limit: [{:?}], Party Size: [{:?}], Active Player: [{:?}]",
        plugin.get_party_size_limit().unwrap(),
        party.get_player_count_party(&player_query).unwrap(),
        party.get_active_player_index().unwrap(),
    ));

    right_data_vec.push(String::from(new_line_right_line));

    // Display the active party players information
    let player_map = &party.player_map;
    right_data_vec.push(format!(" ---> Player Map - Count: [{}]\n ", &player_map.len()));
    for player in player_map {
        right_data_vec.push(format!(
            "Party Player - Index: [ {:?} ], Uuid: [ {:?} ]{}", 
            player.0, 
            player.1,
            &new_line_right_line, 
        ));
    }

    right_data_vec.push(String::from(new_line_right_line));
    
    let target_idx = di.get_db_target_int().unwrap() as usize;  
    let target_uuid = player_uuid_string_vec[target_idx];

    let mut right_display = vec![
        format!("[ Numpad 8 ] pipeline_db_and_party_remove_all_build_test_ref_and_init_new_main_player"),
        format!("________________________________________________________________________________________________________________"),
        format!("[ Numpad 7 ] Previous <--- [ DBTarget: {:?} ] ---> Next [ Numpad 9 ]", target_uuid),  
        format!("________________________________________________________________________________________________________________"),
        format!("[ Numpad 6 ] player_map_and_component_remove_all_players [ PartyTarget ] "),
        format!("[ Numpad 5 ] pipeline_db_and_party_action_remove_player [ DBTarget ]"),
        format!("[ Numpad 4 ] pipeline_db_and_party_add_player_from_db_to_party [ DBTarget ]"),
        format!("[ Numpad 3 ]  "),
        format!("[ Numpad 2 ] pipeline_db_and_party_add_new_synced_player_local / [ Numpad 2 + Shift ] ...player_ai_local"),
        format!("[ Numpad 1 ] party.players_remove_player [ PartyTarget ]"),
        format!("________________________________________________________________________________________________________________"),
    ];
    right_data_vec.append(&mut right_display);

    let party_ids = party.get_all_players_ids(&player_query)?;
    let active_player = party.get_active_player_index()?;
    let active_player_idx = active_player - 1;

    if party.get_player_count_party(&player_query)? > 0 {
        let party_target = party_ids[active_player_idx];
        let mut right_display_optional = vec![
            format!("[ Numpad 0 ] Previous <--- [    PartyTarget: {:?} ] ---> Next [ NumpadDecimal ]", party_target),
            format!("________________________________________________________________________________________________________________"),
        ];
        right_data_vec.append(&mut right_display_optional);
    }

    easy_vec_ui_resource.inject_vec_right(right_data_vec);
    Ok(())
}

pub fn temp_interface(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    db: Res<DatabaseConnection>,
    dbi: Res<PlayerHandlerInterface>,
    mut di: ResMut<DisplayInts>,
    player_query: Query<&PlayerComponent>,
    entity_player_query: Query<(Entity, &PlayerComponent)>,
    mut plugin: ResMut<BevyEasyPlayerHandlerPlugin>,
    mut party: ResMut<Party>,
) { 
    let db_player_vec = match dbi.query_db_existing_players(&db){
        Ok(vec) => vec,
        Err(e) => {
            warn!("Error: easy_vec_ui -> db_pipeline_action_query_existing_players: [{:?}]", e);
            Vec::new() // Return an empty vector in case of error
        },
    };
    
    let db_count: i32 = db_player_vec.len() as i32;
    let db_count_idx = db_count - 1;
    let db_target_idx: i32 = di.get_db_target_int().unwrap();    

    let party_count = match party.get_player_count_party(&player_query) {
        Ok(usize) => usize,
        Err(e) => {
            warn!("Error: temp_interface -> plugin.get_target_idx() [Error: {:?}]", e);
            return ()
        },
    };

    let active_player = match party.get_active_player_index() {
        Ok(usize) => usize,
        Err(e) => {
            warn!("Error: temp_interface -> party.get_active_player_index() [Error: {:?}]", e);
            return ()
        },
    };
    let active_player_idx = active_player - 1;

    // --- Key Function Declaration --- //

    if keys.just_released(KeyCode::Numpad9) {
        info!("just_released: Numpad9");   
        {
            if db_count_idx <= db_target_idx {
                match di.set_db_target_int(0) {
                    Ok(_) => {},
                    Err(e) => warn!("Error: temp_interface -> set_db_target_int Error:[{:?}]", e),
                };
            } else {
                match di.set_db_target_int(db_target_idx + 1) {
                    Ok(_) => {},
                    Err(e) => warn!("Error: temp_interface -> set_db_target_int Error:[{:?}]", e),
                };
            }
            info!("Call [ set_db_target_int += 1 ]: Finished...");
        }
    };

    if keys.just_released(KeyCode::Numpad8) {
        info!("just_released: Numpad8");  
        {
            match di.set_db_target_int(0) {
                Ok(()) => {},
                Err(e) => warn!("Error: temp_interface -> di.set_db_target_int(0) Error:[{:?}]", e),
            };
            // match dbi.action_remove_all_player_records(&db) {
            //     Ok(()) => {},
            //     Err(e) => warn!("Error: temp_interface -> action_remove_all_player_records Error:[{:?}]", e),
            // };
            match dbi.pipeline_db_and_party_remove_all_build_test_ref_and_init_new_main_player(&db, &mut commands, &entity_player_query, &mut party, &player_query, &mut plugin) {
                Ok(_) => {},
                Err(e) => warn!("Error: temp_interface -> dbi.pipeline_db_and_party_reset_all_and_init_test_ref_and_main_player Error:[{:?}]", e),
            };
            info!("Call [ action_remove_all_player_records ]: Finished...");
        }
    };

    if keys.just_released(KeyCode::Numpad7) {
        info!("just_released: Numpad7");  
        {    
            if db_target_idx == 0 {
                match di.set_db_target_int(db_count_idx) {
                    Ok(_) => {},
                    Err(e) => warn!("Error: temp_interface -> di.set_db_target_int Error:[{:?}]", e),
                };
            } else {
                match di.set_db_target_int(db_target_idx - 1) {
                    Ok(_) => {},
                    Err(e) => warn!("Error: temp_interface -> di.set_db_target_int Error:[{:?}]", e),
                };
            }
            info!("Call [ set_db_target_int -= 1 ]: Finished...");
        }
    };

    if keys.just_released(KeyCode::Numpad6) {
        info!("just_released: Numpad6");  
        {
            match party.player_map_and_component_remove_all_players_besides_main(&mut commands, &entity_player_query, &player_query, &mut plugin) {
                Ok(success) => success,
                Err(e) => warn!("Error: temp_interface -> party.player_map_and_component_remove_all_players Error:[{:?}]", e),
            };
        }    
    };

    if keys.just_released(KeyCode::Numpad5) {
        info!("just_released: Numpad5"); 
        {
            if db_player_vec.len() > 2 {
                let db_target = &db_player_vec[db_target_idx as usize];
                let db_target_uuid_str = db_target.get_uuid_string().as_str();
                let db_target_uuid = Uuid::try_parse(db_target_uuid_str).unwrap();
                let stored_id = &db_target_uuid;
                
                if db_target_idx == ( db_count - 1 ) { // Indexed to test against the future result that would be post action
                    match di.set_db_target_int(db_target_idx - 1) {
                        Ok(_) => {},
                        Err(e) => warn!("Error: temp_interface -> set_db_target_int Error:[{:?}]", e),
                    };
                }
                let party_idx = match party.get_active_player_index() {
                    Ok(value) => value,
                    Err(e) => {
                        warn!("Error: temp_interface -> party.get_active_player_index() Error:[{:?}]", e);
                        1
                    },
                };
                let party_size = match party.get_player_count_party(&player_query) {
                    Ok(value) => value,
                    Err(e) => {
                        warn!("Error: temp_interface -> party.get_active_player_index() Error:[{:?}]", e);
                        1
                    },
                };
                if party_idx == party_size && party_size > 1 {
                    match party.set_active_player_index(party_idx - 1, &player_query) {
                        Ok(value) => value,
                        Err(e) => {
                            warn!("Error: temp_interface -> party.get_active_player_index() Error:[{:?}]", e);
                        },
                    };
                }
                match dbi.pipeline_db_and_party_action_remove_player(&mut commands, &db, &entity_player_query, &mut party, &player_query, stored_id, &mut plugin) {
                    // match dbi.pipeline_db_and_party_action_remove_player(&db, &mut party, &mut plugin, stored_id) {
                    Ok(_) => {},
                    Err(e) => warn!("Error: temp_interface -> {} -> pipeline_db_and_party_action_remove_player [{:?}]", &db_target_uuid, e),
                }
                info!("Call [ pipeline_db_and_party_action_remove_player [{}] ]: Finished...", stored_id);
            } else {
                info!("Call [ pipeline_db_and_party_action_remove_player ]: Denied: Only TestRef and Main Exist...");
            }
        }
    };

    if keys.just_released(KeyCode::Numpad4) {
        info!("just_released: Numpad4");  
        {
            let db_target = &db_player_vec[db_target_idx as usize];
            let db_target_uuid_str = db_target.get_uuid_string().as_str();
            let db_target_uuid = Uuid::try_parse(db_target_uuid_str).unwrap();
            match dbi.pipeline_db_and_party_add_player_from_db_to_party(&mut commands, &db, &db_target_uuid, &mut party, &player_query, &mut plugin) {
                Ok(_) => {},
                Err(e) => warn!("Error: temp_interface -> pipeline_db_and_party_add_player_from_db_to_party [{:?}]", e),
            };
            info!("Call [ pipeline_db_and_party_add_player_from_db_to_party ]: Finished...");
        }
    };

    if keys.just_released(KeyCode::Numpad3) {
        info!("just_released: Numpad3");  
        {
        }
    };

    if keys.just_released(KeyCode::Numpad2) &! ( keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight)){
        info!("just_released: Numpad2");  
        {
            match dbi.pipeline_db_and_party_add_new_synced_player_local(&mut commands, &db, &mut party, &player_query, &plugin, "PlayerLocal") {
                Ok(()) => {},
                Err(e) => warn!("Error: temp_interface -> pipeline_db_and_party_add_new_synced_player_local:  [{:?}]", e),
            };
            info!("Call [ pipeline_db_and_party_add_new_synced_player_local ]: Finished...");
        }
    };

    if keys.just_released(KeyCode::Numpad2) && ( keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight)) {
        info!("just_released: Numpad2 + Shift");  
        {        
            match dbi.pipeline_db_and_party_add_new_synced_player_ai_local(&mut commands, &db, &mut party, &player_query, &plugin, "PlayerAiLocal"){
                Ok(()) => {},
                Err(e) => warn!("Error: temp_interface -> pipeline_db_and_party_add_new_synced_player_ai_local:  [{:?}]", e),
            };
            info!("Call [ pipeline_db_and_party_add_new_synced_player_ai_local ]: Finished...");
        }
    };

    if keys.just_released(KeyCode::Numpad1) {
        info!("just_released: Numpad1"); 
        {
            if party_count > 1 {
                let stored_uuid = match party.clone_active_player_uuid(&player_query) {
                    Ok(id) => id,
                    Err(e) => {
                        warn!("Error: temp_interface -> party.clone_active_player_uuid() Error:[{:?}]", e);
                        return ()
                    },
                };
                match party.set_active_player_index(active_player_idx, &player_query) {
                    Ok(()) => (),
                    Err(e) => warn!("Error: temp_interface -> party.set_active_player_index() Error:[{:?}]", e),
                };
                println!("active_player_get_player_id -> stored_uuid: [{}]", stored_uuid);
                match party.remove_player(&mut commands, &entity_player_query, &mut plugin, &stored_uuid) {
                    Ok(()) => (),
                    Err(e) => warn!("Error: temp_interface -> party.players_remove_player Error:[{:?}]", e),
                };
                let player_idx = match party.get_active_player_index() {
                    usize => usize.unwrap()
                };
                if player_idx == 0 {
                    match party.set_active_player_index(1, &player_query) {
                        Ok(()) => (),
                        Err(e) => warn!("Error: temp_interface -> party.active_player_set Error:[{:?}]", e),
                    };
                };
                info!("Call [ players_remove_player [{:?}] ]: Finished...", stored_uuid);
            } else {
                warn!("Only one player in the party! Cannot be removed...");
            }
        }
    };

    if keys.just_released(KeyCode::Numpad0) {
        info!("just_released: Numpad0");  
        if active_player > 1 {
            let _ = party.set_active_player_index(active_player_idx, &player_query);
        } else {
            let _ = party.set_active_player_index(party_count, &player_query);
        }
        info!("Call [ party.active_player_set - 1 ]: Finished...");
    };

    if keys.just_released(KeyCode::NumpadDecimal) {
        info!("just_released: NumpadDecimal");  
        if active_player < party_count {
            let _ = party.set_active_player_index(active_player + 1, &player_query);
        } else {
            let _ = party.set_active_player_index(1, &player_query);
        }
        info!("Call [ party.active_player_set + 1 ]: Finished...");
    };
}