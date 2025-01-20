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

    // --- PlayerAiLocal Tests --- //

    #[test]
    fn test_player_ai_local_new() -> Result<(), ErrorTypePlayerHandler> {
        // Test case 1: All fields provided
        {
            let email = Some(PLAYER_EMAIL.to_string());
            let username = Some(PLAYER_USERNAME.to_string());
            let uuid = Some(Uuid::now_v7());

            let player = PlayerAiLocal::new(email.clone(), username.clone(), uuid, PLAYER_TYPE_AI_LOCAL);

            let ref_1_player_email = player.get_player_email()?.to_owned();
            let ref_1_player_username = player.get_player_username()?.to_owned();
            let ref_1_player_uuid = player.get_player_id()?.to_owned();
            let ref_1_player_type = player.get_player_type()?.to_owned();

            assert_eq!(Some(ref_1_player_email), email);
            assert_eq!(Some(ref_1_player_username), username);
            assert_eq!(Some(ref_1_player_uuid), uuid);
            assert_eq!(Some(ref_1_player_type), Some(PLAYER_TYPE_AI_LOCAL));
        }
        // Test case 2: Missing UUID (should generate a new one)
        {    
            let email = Some(ALT_PLAYER_EMAIL.to_string());
            let username = Some(ALT_PLAYER_USERNAME.to_string());
            let player = PlayerAiLocal::new(email.clone(), username.clone(), None, PLAYER_TYPE_AI_LOCAL);

            let ref_2_player_email = player.get_player_email()?.to_owned();
            let ref_2_player_username = player.get_player_username()?.to_owned();
            let ref_2_player_type = player.get_player_type()?.to_owned();
            let ref_2_player_uuid = player.get_player_id()?.to_owned();

            assert_eq!(Some(ref_2_player_email), email);
            assert_eq!(Some(ref_2_player_username), username);
            assert_eq!(Some(ref_2_player_type), Some(PLAYER_TYPE_AI_LOCAL));

            // Check that a new UUID was generated
            assert!(ref_2_player_uuid.is_nil() == false);
        }    
        Ok(())
    }

    #[test]
    fn test_player_ai_local_set() -> Result<(), ErrorTypePlayerHandler> {
        let email = Some(PLAYER_EMAIL.to_string());
        let username = Some(PLAYER_USERNAME.to_string());
        let uuid = Some(Uuid::now_v7());

        let mut player = PlayerAiLocal::new(email.clone(), username.clone(), uuid.clone(), PLAYER_TYPE_AI_LOCAL);

        let ref_player_email = player.get_player_email()?.to_owned();
        let ref_player_username = player.get_player_username()?.to_owned();
        let ref_player_uuid = player.get_player_id()?.to_owned();

        player.set_player_email(ALT_PLAYER_EMAIL)?;
        player.set_player_username(ALT_PLAYER_USERNAME)?;
        player.set_player_id(Uuid::new_v4())?;

        assert_eq!(Some(ref_player_email), email);
        assert_eq!(Some(ref_player_username), username);
        assert_eq!(Some(ref_player_uuid), uuid);

        Ok(())
    }

    // --- PlayerAiRemote Tests --- //

    #[test]
    fn test_player_ai_remote_new() -> Result<(), ErrorTypePlayerHandler> {
        // Test case 1: All fields provided
        {
            let email = Some(PLAYER_EMAIL.to_string());
            let username = Some(PLAYER_USERNAME.to_string());
            let uuid = Some(Uuid::now_v7());

            let player = PlayerAiRemote::new(email.clone(), username.clone(), uuid, PLAYER_TYPE_AI_REMOTE);

            let ref_1_player_email = player.get_player_email()?.to_owned();
            let ref_1_player_username = player.get_player_username()?.to_owned();
            let ref_1_player_uuid = player.get_player_id()?.to_owned();
            let ref_1_player_type = player.get_player_type()?.to_owned();

            assert_eq!(Some(ref_1_player_email), email);
            assert_eq!(Some(ref_1_player_username), username);
            assert_eq!(Some(ref_1_player_uuid), uuid);
            assert_eq!(Some(ref_1_player_type), Some(PLAYER_TYPE_AI_REMOTE));
        }
        // Test case 2: Missing UUID (should generate a new one)
        {    
            let email = Some(ALT_PLAYER_EMAIL.to_string());
            let username = Some(ALT_PLAYER_USERNAME.to_string());
            let player = PlayerAiRemote::new(email.clone(), username.clone(), None, PLAYER_TYPE_AI_REMOTE);

            let ref_2_player_email = player.get_player_email()?.to_owned();
            let ref_2_player_username = player.get_player_username()?.to_owned();
            let ref_2_player_type = player.get_player_type()?.to_owned();
            let ref_2_player_uuid = player.get_player_id()?.to_owned();

            assert_eq!(Some(ref_2_player_email), email);
            assert_eq!(Some(ref_2_player_username), username);
            assert_eq!(Some(ref_2_player_type), Some(PLAYER_TYPE_AI_REMOTE));

            // Check that a new UUID was generated
            assert!(ref_2_player_uuid.is_nil() == false);
        }    
        Ok(())
    }

    #[test]
    fn test_player_ai_remote_set() -> Result<(), ErrorTypePlayerHandler> {
        let email = Some(PLAYER_EMAIL.to_string());
        let username = Some(PLAYER_USERNAME.to_string());
        let uuid = Some(Uuid::now_v7());

        let mut player = PlayerAiRemote::new(email.clone(), username.clone(), uuid.clone(), PLAYER_TYPE_AI_REMOTE);

        let ref_player_email = player.get_player_email()?.to_owned();
        let ref_player_username = player.get_player_username()?.to_owned();
        let ref_player_uuid = player.get_player_id()?.to_owned();

        player.set_player_email(ALT_PLAYER_EMAIL)?;
        player.set_player_username(ALT_PLAYER_USERNAME)?;
        player.set_player_id(Uuid::new_v4())?;

        assert_eq!(Some(ref_player_email), email);
        assert_eq!(Some(ref_player_username), username);
        assert_eq!(Some(ref_player_uuid), uuid);

        Ok(())
    }

    // --- PlayerLocal Tests --- //

    #[test]
    fn test_player_local_new() -> Result<(), ErrorTypePlayerHandler> {
        // Test case 1: All fields provided
        {
            let email = Some(PLAYER_EMAIL.to_string());
            let username = Some(PLAYER_USERNAME.to_string());
            let uuid = Some(Uuid::now_v7());

            let player = PlayerLocal::new(email.clone(), username.clone(), uuid, PLAYER_TYPE_LOCAL);

            let ref_1_player_email = player.get_player_email()?.to_owned();
            let ref_1_player_username = player.get_player_username()?.to_owned();
            let ref_1_player_uuid = player.get_player_id()?.to_owned();
            let ref_1_player_type = player.get_player_type()?.to_owned();

            assert_eq!(Some(ref_1_player_email), email);
            assert_eq!(Some(ref_1_player_username), username);
            assert_eq!(Some(ref_1_player_uuid), uuid);
            assert_eq!(Some(ref_1_player_type), Some(PLAYER_TYPE_LOCAL));
        }
        // Test case 2: Missing UUID (should generate a new one)
        {    
            let email = Some(ALT_PLAYER_EMAIL.to_string());
            let username = Some(ALT_PLAYER_USERNAME.to_string());
            let player = PlayerLocal::new(email.clone(), username.clone(), None, PLAYER_TYPE_LOCAL);

            let ref_2_player_email = player.get_player_email()?.to_owned();
            let ref_2_player_username = player.get_player_username()?.to_owned();
            let ref_2_player_type = player.get_player_type()?.to_owned();
            let ref_2_player_uuid = player.get_player_id()?.to_owned();

            assert_eq!(Some(ref_2_player_email), email);
            assert_eq!(Some(ref_2_player_username), username);
            assert_eq!(Some(ref_2_player_type), Some(PLAYER_TYPE_LOCAL));

            // Check that a new UUID was generated
            assert!(ref_2_player_uuid.is_nil() == false);
        }    
        Ok(())
    }

    #[test]
    fn test_player_local_set() -> Result<(), ErrorTypePlayerHandler> {
        let email = Some(PLAYER_EMAIL.to_string());
        let username = Some(PLAYER_USERNAME.to_string());
        let uuid = Some(Uuid::now_v7());

        let mut player = PlayerLocal::new(email.clone(), username.clone(), uuid.clone(), PLAYER_TYPE_LOCAL);

        let ref_player_email = player.get_player_email()?.to_owned();
        let ref_player_username = player.get_player_username()?.to_owned();
        let ref_player_uuid = player.get_player_id()?.to_owned();

        player.set_player_email(ALT_PLAYER_EMAIL)?;
        player.set_player_username(ALT_PLAYER_USERNAME)?;
        player.set_player_id(Uuid::new_v4())?;

        assert_eq!(Some(ref_player_email), email);
        assert_eq!(Some(ref_player_username), username);
        assert_eq!(Some(ref_player_uuid), uuid);

        Ok(())
    }

    // --- PlayerMain Tests --- //

    #[test]
    fn test_player_main_new() -> Result<(), ErrorTypePlayerHandler> {
        // Test case 1: All fields provided
        {
            let email = Some(PLAYER_EMAIL.to_string());
            let username = Some(PLAYER_USERNAME.to_string());
            let uuid = Some(Uuid::now_v7());

            let player = PlayerMain::new(email.clone(), username.clone(), uuid, PLAYER_TYPE_MAIN);

            let ref_1_player_email = player.get_player_email()?.to_owned();
            let ref_1_player_username = player.get_player_username()?.to_owned();
            let ref_1_player_uuid = player.get_player_id()?.to_owned();
            let ref_1_player_type = player.get_player_type()?.to_owned();

            assert_eq!(Some(ref_1_player_email), email);
            assert_eq!(Some(ref_1_player_username), username);
            assert_eq!(Some(ref_1_player_uuid), uuid);
            assert_eq!(Some(ref_1_player_type), Some(PLAYER_TYPE_MAIN));
        }
        // Test case 2: Missing UUID (should generate a new one)
        {    
            let email = Some(ALT_PLAYER_EMAIL.to_string());
            let username = Some(ALT_PLAYER_USERNAME.to_string());
            let player = PlayerMain::new(email.clone(), username.clone(), None, PLAYER_TYPE_MAIN);

            let ref_2_player_email = player.get_player_email()?.to_owned();
            let ref_2_player_username = player.get_player_username()?.to_owned();
            let ref_2_player_type = player.get_player_type()?.to_owned();
            let ref_2_player_uuid = player.get_player_id()?.to_owned();

            assert_eq!(Some(ref_2_player_email), email);
            assert_eq!(Some(ref_2_player_username), username);
            assert_eq!(Some(ref_2_player_type), Some(PLAYER_TYPE_MAIN));

            // Check that a new UUID was generated
            assert!(ref_2_player_uuid.is_nil() == false);
        }    
        Ok(())
    }

    #[test]
    fn test_player_main_set() -> Result<(), ErrorTypePlayerHandler> {
        let email = Some(PLAYER_EMAIL.to_string());
        let username = Some(PLAYER_USERNAME.to_string());
        let uuid = Some(Uuid::now_v7());

        let mut player = PlayerMain::new(email.clone(), username.clone(), uuid.clone(), PLAYER_TYPE_MAIN);

        let ref_player_email = player.get_player_email()?.to_owned();
        let ref_player_username = player.get_player_username()?.to_owned();
        let ref_player_uuid = player.get_player_id()?.to_owned();

        player.set_player_email(ALT_PLAYER_EMAIL)?;
        player.set_player_username(ALT_PLAYER_USERNAME)?;
        player.set_player_id(Uuid::new_v4())?;

        assert_eq!(Some(ref_player_email), email);
        assert_eq!(Some(ref_player_username), username);
        assert_eq!(Some(ref_player_uuid), uuid);

        Ok(())
    }

    // --- PlayerRemote Tests --- //

    #[test]
    fn test_player_remote_new() -> Result<(), ErrorTypePlayerHandler> {
        // Test case 1: All fields provided
        {
            let email = Some(PLAYER_EMAIL.to_string());
            let username = Some(PLAYER_USERNAME.to_string());
            let uuid = Some(Uuid::now_v7());

            let player = PlayerRemote::new(email.clone(), username.clone(), uuid, PLAYER_TYPE_REMOTE);

            let ref_1_player_email = player.get_player_email()?.to_owned();
            let ref_1_player_username = player.get_player_username()?.to_owned();
            let ref_1_player_uuid = player.get_player_id()?.to_owned();
            let ref_1_player_type = player.get_player_type()?.to_owned();

            assert_eq!(Some(ref_1_player_email), email);
            assert_eq!(Some(ref_1_player_username), username);
            assert_eq!(Some(ref_1_player_uuid), uuid);
            assert_eq!(Some(ref_1_player_type), Some(PLAYER_TYPE_REMOTE));
        }
        // Test case 2: Missing UUID (should generate a new one)
        {    
            let email = Some(ALT_PLAYER_EMAIL.to_string());
            let username = Some(ALT_PLAYER_USERNAME.to_string());
            let player = PlayerRemote::new(email.clone(), username.clone(), None, PLAYER_TYPE_REMOTE);

            let ref_2_player_email = player.get_player_email()?.to_owned();
            let ref_2_player_username = player.get_player_username()?.to_owned();
            let ref_2_player_type = player.get_player_type()?.to_owned();
            let ref_2_player_uuid = player.get_player_id()?.to_owned();

            assert_eq!(Some(ref_2_player_email), email);
            assert_eq!(Some(ref_2_player_username), username);
            assert_eq!(Some(ref_2_player_type), Some(PLAYER_TYPE_REMOTE));

            // Check that a new UUID was generated
            assert!(ref_2_player_uuid.is_nil() == false);
        }    
        Ok(())
    }

    #[test]
    fn test_player_remote_set() -> Result<(), ErrorTypePlayerHandler> {
        let email = Some(PLAYER_EMAIL.to_string());
        let username = Some(PLAYER_USERNAME.to_string());
        let uuid = Some(Uuid::now_v7());

        let mut player = PlayerRemote::new(email.clone(), username.clone(), uuid.clone(), PLAYER_TYPE_REMOTE);

        let ref_player_email = player.get_player_email()?.to_owned();
        let ref_player_username = player.get_player_username()?.to_owned();
        let ref_player_uuid = player.get_player_id()?.to_owned();

        player.set_player_email(ALT_PLAYER_EMAIL)?;
        player.set_player_username(ALT_PLAYER_USERNAME)?;
        player.set_player_id(Uuid::new_v4())?;

        assert_eq!(Some(ref_player_email), email);
        assert_eq!(Some(ref_player_username), username);
        assert_eq!(Some(ref_player_uuid), uuid);

        Ok(())
    }
}