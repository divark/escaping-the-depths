Feature: All of the basic features are implemented and working as intended.
    Scenario: The exit door opens when the explorer finds the hidden floor switch.
        Given a 4x4 cave room,
        And a hidden floor switch placed at coordinates 1, 3,
        And an exit door placed at coordinates 4, 2,
        When the explorer is on Tile 1, 3,
        Then the exit door at Tile 4, 2 will be opened.

    Scenario: The exit door opens when a viewer clicks the hidden floor switch.
        Given a 4x4 cave room,
        And a hidden floor switch placed at coordinates 1, 3,
        And an exit door placed at coordinates 4, 2,
        When a viewer clicks on Tile 1, 3,
        Then the exit door at Tile 4, 2 will be opened.

    Scenario: The explorer heads for the exit when the exit door is opened.
        Given a 4x4 cave room,
        And an exit door placed at coordinates 4, 2,
        And the explorer placed at coordinates 2, 2,
        When the exit door at Tile 4, 2 is now open,
        Then the explorer's goal is to reach Tile 4, 2.

    Scenario: The explorer finding treasure increases the current score.
        Given a 4x4 cave room,
        And a piece of treasure worth 500 points placed at coordinates 2, 2,
        When the explorer is on Tile 2, 2,
        Then the current score will be 500 points.

    Scenario: The current score increases when a viewer clicks on some treasure.
        Given a 4x4 cave room,
        And a piece of treasure worth 500 points placed at coordinates 2, 2,
        When a viewer clicks on Tile 2, 2,
        Then the current score will be 500 points.

    Scenario: An armed trap is disarmed when a viewer clicks on it.
        Given a 4x4 cave room,
        And an armed trap placed at coordinates 2, 3,
        When a viewer clicks on Tile 2, 3,
        Then the trap at Tile 2, 3 will be disarmed.

    Scenario: The health of the explorer will go down if the explorer steps on an armed trap.
        Given a 4x4 cave room,
        And an armed trap placed at coordinates 2, 3,
        When the explorer is on Tile 2, 3,
        Then the explorer's health should be 2 out of 3.

    Scenario: The explorer is passed out if all of her health is gone.
        Given a 4x4 cave room,
        And an armed trap placed at coordinates 2, 3,
        And the explorer's initial health set to 1 out of 3,
        When the explorer is on Tile 2, 3,
        Then the explorer's health should be 0 out of 3.
        And the explorer will be passed out.

    Scenario: The explorer is in a new room when she wakes up from being passed out.
        Given a 4x4 cave room,
        And the explorer placed at coordinates 2, 3,
        And the explorer's initial health set to 0 out of 3,
        When 5 seconds have passed,
        Then the explorer will be at Tile 1, 1.
