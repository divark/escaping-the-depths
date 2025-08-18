Feature: All of the basic features are implemented and working as intended.
    Scenario: The exit door opens when the explorer finds the hidden floor switch.
        Given a 4x4 cave room,
        And a hidden floor switch placed at coordinates 1, 3,
        And an exit door placed at coordinates 3, 2,
        And an explorer placed at coordinates 1, 3,
        When the cave room is rendered,
        Then the exit door will be opened.

    Scenario: The exit door opens when a viewer clicks the hidden floor switch.
        Given a 4x4 cave room,
        And a hidden floor switch placed at coordinates 1, 2,
        And an exit door placed at coordinates 4, 5,
        When the cave room is rendered,
        When a viewer clicks with UV coordinates 0.474334717, 0.479106963,
        Then the exit door will be opened.

    Scenario: The explorer heads for the exit when the exit door is opened.
        Given a 4x4 cave room,
        And a hidden floor switch placed at coordinates 1, 3,
        And an exit door placed at coordinates 4, 2,
        And an explorer placed at coordinates 1, 3,
        When the cave room is rendered,
        Then the exit door will be opened.
        And the explorer's goal is to reach Tile 4, 2.

    Scenario: The explorer reaches the exit door when opened.
        Given a 4x4 cave room,
        And a hidden floor switch placed at coordinates 1, 3,
        And an exit door placed at coordinates 4, 2,
        And an explorer placed at coordinates 1, 3,
        When the cave room is rendered,
        And the explorer has finished exiting,
        Then the explorer should be on Tile 4, 2.

    Scenario: A new room should be made when the explorer reaches the exit door.
        Given a 4x4 cave room,
        And a hidden floor switch placed at coordinates 1, 3,
        And an exit door placed at coordinates 2, 5,
        And an explorer placed at coordinates 1, 3,
        When the cave room is rendered,
        And the explorer has reached Tile 2, 5,
        And the explorer has left the room,
        Then the explorer should be on Tile 2, 1.
        And the current room count should be 2.

    Scenario: The explorer is wandering when the exit door is shut.
        Given a 4x4 cave room,
        And an exit door placed at coordinates 3, 2,
        And an explorer placed at coordinates 1, 3,
        When the cave room is rendered,
        Then the explorer should be visiting all other tiles in the room.

    Scenario: The explorer stops wandering when the exit door is opened.
        Given a 4x4 cave room,
        And an exit door placed at coordinates 3, 2,
        And a hidden floor switch placed at coordinates 1, 3,
        And an explorer placed at coordinates 3, 3,
        When the cave room is rendered,
        And the explorer has reached Tile 1, 3,
        Then the explorer's goal is to reach Tile 3, 2.

    Scenario: The explorer finding treasure increases the current score.
        Given a 4x4 cave room,
        And some piece of treasure worth 500 points placed on coordinates 2, 2,
        And an explorer placed at coordinates 2, 2,
        When the cave room is rendered,
        Then the current score will be 500 points.

    Scenario: The current score increases when a viewer clicks on some treasure.
        Given a 3x3 cave room,
        And some piece of treasure worth 500 points placed on coordinates 2, 1,
        When the cave room is rendered,
        When a viewer clicks with UV coordinates 0.49375, 0.4667,
        Then the current score will be 500 points.

    Scenario: An armed trap is disarmed when a viewer clicks on it.
        Given a 3x3 cave room,
        And an armed trap placed at coordinates 2, 1,
        When the cave room is rendered,
        When a viewer clicks with UV coordinates 0.49375, 0.4667,
        Then the trap at Tile 2, 1 will be disarmed.

    Scenario: The health of the explorer will go down if the explorer steps on an armed trap.
        Given a 4x4 cave room,
        And an armed trap placed at coordinates 2, 3,
        And an explorer placed at coordinates 2, 3,
        When the cave room is rendered,
        Then the explorer's health should be 2 out of 3.
        And the trap at Tile 2, 3 will be disarmed.

    Scenario: The explorer is passed out if all of her health is gone.
        Given a 4x4 cave room,
        And an armed trap placed at coordinates 2, 3,
        And the explorer's initial health set to 1 out of 3,
        And an explorer placed at coordinates 2, 3,
        When the cave room is rendered,
        Then the explorer's health should be 0 out of 3.
        And the explorer will be passed out.

    Scenario: The explorer is in a new room when she wakes up from being passed out.
        Given a 4x4 cave room,
        And the explorer's initial health set to 0 out of 3,
        And an explorer placed at coordinates 2, 3,
        When the cave room is rendered,
        And the game over timer has elapsed,
        Then the explorer's health should be 3 out of 3.
        Then the explorer should be on Tile 1, 1.

    Scenario: The current statistics becomes the new record when the explorer is passed out.
        Given a 4x4 cave room,
        And the explorer's initial health set to 0 out of 3,
        And an explorer placed at coordinates 2, 3,
        And the initial room count is 2,
        And the initial current score is 500,
        When the cave room is rendered,
        And the game over timer has elapsed,
        Then the current score will be 0 points.
        And the current room count should be 1.
        And the record score will be 500 points.
        And the record room count should be 2.

    Scenario: Tile scaling is honored for viewer clicks.
        Given a 3x3 cave room,
        And a hidden floor switch placed at coordinates 1, 3,
        And an exit door placed at coordinates 3, 4,
        And a tile scale of 2,
        When the cave room is rendered,
        When a viewer clicks with UV coordinates 0.4904, 0.5853,
        Then the exit door will be opened.
