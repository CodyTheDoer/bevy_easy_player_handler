#[cfg(test)]
mod tests {
    use bevy_easy_player_handler::Party;

    #[test]
    fn test_party_get_active_player_index_data_new() {
        let party = Party::new(); // Create a new Party instance
        let result = party.get_active_player_index(); // Call the function to test

        // Assert that the result is as expected
        assert!(result.is_ok()); // Ensure no error is returned
        assert_eq!(result.unwrap(), 1); // Check that the active player index is 1
    }

    #[test]
    fn test_party_get_active_player_index_data_altered() {
        let mut party = Party::new();
        party.active_player = 2; // Manually set the active player to a new value
        let result = party.get_active_player_index();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2); // Check the altered value
    }
}