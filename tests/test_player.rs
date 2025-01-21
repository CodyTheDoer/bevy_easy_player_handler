#[cfg(test)]
mod tests {
    use bevy_easy_player_handler::*;
    use bevy_easy_shared_definitions::ErrorTypePlayerHandler;
    use uuid::Uuid;

    const PLAYER_EMAIL: &str = "test@example.com";
    const PLAYER_USERNAME: &str = "test_user";
    const ALT_PLAYER_EMAIL: &str = "another_test@example.com";
    const ALT_PLAYER_USERNAME: &str = "another_user";
    const PLAYER_TYPE_AI_LOCAL: PlayerType = PlayerType::PlayerAiLocal;
    const PLAYER_TYPE_AI_REMOTE: PlayerType = PlayerType::PlayerAiRemote;
    const PLAYER_TYPE_LOCAL: PlayerType = PlayerType::PlayerLocal;
    const PLAYER_TYPE_MAIN: PlayerType = PlayerType::PlayerMain;
    const PLAYER_TYPE_REMOTE: PlayerType = PlayerType::PlayerRemote;

    // --- macro --- //

    macro_rules! player_new_all_data {
        ($player_type_new:expr, $target_type:expr) => {{
            let email = Some(PLAYER_EMAIL.to_string());
            let username = Some(PLAYER_USERNAME.to_string());
            let uuid = Some(Uuid::now_v7());

            let player = $player_type_new(email.clone(), username.clone(), uuid, $target_type);
            let player_email = player.get_player_email()?.to_owned();
            let player_type = player.get_player_type()?.to_owned();
            let player_username = player.get_player_username()?.to_owned();
            let player_uuid = player.get_player_id()?.to_owned();

            assert_eq!(Some(player_email), email);
            assert_eq!(Some(player_type), Some($target_type));
            assert_eq!(Some(player_username), username);
            assert_eq!(Some(player_uuid), uuid);
        }};
    }

    macro_rules! player_new_uuid_missing {
        ($player_type_new:expr, $target_type:expr) => {{
            let email = Some(ALT_PLAYER_EMAIL.to_string());
            let username = Some(ALT_PLAYER_USERNAME.to_string());
            let player = $player_type_new(email.clone(), username.clone(), None, $target_type);

            let player_email = player.get_player_email()?.to_owned();
            let player_type = player.get_player_type()?.to_owned();
            let player_username = player.get_player_username()?.to_owned();
            let player_uuid = player.get_player_id()?.to_owned();

            assert_eq!(Some(player_email), email);
            assert_eq!(Some(player_type), Some($target_type));
            assert_eq!(Some(player_username), username);

            // Check that a new UUID was generated
            assert!(player_uuid.is_nil() == false);
        }};
    }

    macro_rules! player_set {
        ($player_type_new:expr, $target_type:expr) => {{
            let email = Some(PLAYER_EMAIL.to_string());
            let username = Some(PLAYER_USERNAME.to_string());
            let uuid = Some(Uuid::now_v7());
    
            let mut player = $player_type_new(email.clone(), username.clone(), uuid.clone(), $target_type);
    
            let new_uuid = Uuid::new_v4();

            player.set_player_email(ALT_PLAYER_EMAIL)?;
            player.set_player_username(ALT_PLAYER_USERNAME)?;
            player.set_player_id(new_uuid.clone())?;
    
            let player_email = player.get_player_email()?.to_owned();
            let player_username = player.get_player_username()?.to_owned();
            let player_uuid = player.get_player_id()?.to_owned();

            assert_eq!(Some(player_email), Some(ALT_PLAYER_EMAIL.to_string()));
            assert_eq!(Some(player_username), Some(ALT_PLAYER_USERNAME.to_string()));
            assert_eq!(Some(player_uuid), Some(new_uuid));            
        }};
    }

    // --- PlayerAiLocal Tests --- //

    #[test]
    fn test_player_ai_local_new_all_data() -> Result<(), ErrorTypePlayerHandler> {
        player_new_all_data!(PlayerAiLocal::new, PLAYER_TYPE_AI_LOCAL);
        Ok(())
    }

    #[test]
    fn test_player_ai_local_new_uuid_missing() -> Result<(), ErrorTypePlayerHandler> {
        player_new_uuid_missing!(PlayerAiLocal::new, PLAYER_TYPE_AI_LOCAL);
        Ok(())
    }

    #[test]
    fn test_player_ai_local_set() -> Result<(), ErrorTypePlayerHandler> {
        player_set!(PlayerAiLocal::new, PLAYER_TYPE_AI_LOCAL);
        Ok(())
    }

    // --- PlayerAiRemote Tests --- //

    #[test]
    fn test_player_ai_remote_new_all_data() -> Result<(), ErrorTypePlayerHandler> {
        player_new_all_data!(PlayerAiRemote::new, PLAYER_TYPE_AI_REMOTE);
        Ok(())
    }

    #[test]
    fn test_player_ai_remote_new_uuid_missing() -> Result<(), ErrorTypePlayerHandler> {
        player_new_uuid_missing!(PlayerAiRemote::new, PLAYER_TYPE_AI_REMOTE);
        Ok(())
    }

    #[test]
    fn test_player_ai_remote_set() -> Result<(), ErrorTypePlayerHandler> {
        player_set!(PlayerAiRemote::new, PLAYER_TYPE_AI_REMOTE);
        Ok(())
    }

    // --- PlayerLocal Tests --- //

    #[test]
    fn test_player_local_new_all_data() -> Result<(), ErrorTypePlayerHandler> {
        player_new_all_data!(PlayerLocal::new, PLAYER_TYPE_LOCAL);
        Ok(())
    }

    #[test]
    fn test_player_local_new_uuid_missing() -> Result<(), ErrorTypePlayerHandler> {
        player_new_uuid_missing!(PlayerLocal::new, PLAYER_TYPE_LOCAL);
        Ok(())
    }

    #[test]
    fn test_player_local_set() -> Result<(), ErrorTypePlayerHandler> {
        player_set!(PlayerLocal::new, PLAYER_TYPE_LOCAL);
        Ok(())
    }

    // --- PlayerMain Tests --- //

    #[test]
    fn test_player_main_new_all_data() -> Result<(), ErrorTypePlayerHandler> {
        player_new_all_data!(PlayerMain::new, PLAYER_TYPE_MAIN);
        Ok(())
    }

    #[test]
    fn test_player_main_new_uuid_missing() -> Result<(), ErrorTypePlayerHandler> {
        player_new_uuid_missing!(PlayerMain::new, PLAYER_TYPE_MAIN);
        Ok(())
    }

    #[test]
    fn test_player_main_set() -> Result<(), ErrorTypePlayerHandler> {
        player_set!(PlayerMain::new, PLAYER_TYPE_MAIN);
        Ok(())
    }

    // --- PlayerRemote Tests --- //

    #[test]
    fn test_player_remote_new_all_data() -> Result<(), ErrorTypePlayerHandler> {
        player_new_all_data!(PlayerRemote::new, PLAYER_TYPE_REMOTE);
        Ok(())
    }

    #[test]
    fn test_player_remote_new_uuid_missing() -> Result<(), ErrorTypePlayerHandler> {
        player_new_uuid_missing!(PlayerRemote::new, PLAYER_TYPE_REMOTE);
        Ok(())
    }

    #[test]
    fn test_player_remote_set() -> Result<(), ErrorTypePlayerHandler> {
        player_set!(PlayerRemote::new, PLAYER_TYPE_REMOTE);
        Ok(())
    }
}