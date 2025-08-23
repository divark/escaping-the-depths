Feature: Quality of Life requests from playtesting and feedback.
    Scenario: The player at the bottom right of a small room should not have traps around them.
        Given a player's location of 1, 0,
        And a room size of 2x2,
        When the excluded locations are identified from the player's location,
        Then there should be 4 excluded locations.
        Then location 0, 0 should be marked as excluded.
        Then location 1, 0 should be marked as excluded.
        Then location 1, 1 should be marked as excluded.

    Scenario: The player at the bottom right of a bigger room should not have traps around them.
        Given a player's location of 2, 1,
        And a room size of 4x4,
        When the excluded locations are identified from the player's location,
        Then there should be 9 excluded locations.
        And location 1, 0 should be marked as excluded.
        And location 2, 0 should be marked as excluded.
        And location 3, 0 should be marked as excluded.
        And location 1, 1 should be marked as excluded.
        And location 2, 1 should be marked as excluded.
        And location 3, 1 should be marked as excluded.
        And location 1, 2 should be marked as excluded.
        And location 2, 2 should be marked as excluded.
        And location 3, 2 should be marked as excluded.
