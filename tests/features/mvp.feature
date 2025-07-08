Feature: All of the basic features are implemented and working as intended.
    Scenario: The exit door opens when the explorer finds the hidden floor switch.
        Given a 4x4 cave room,
        And a hidden floor switch placed at coordinates 1, 3,
        And an exit door placed at coordinates 3, 2,
        When the explorer is on Tile 1, 3,
        Then the exit door will be opened.

    Scenario: The exit door opens when a viewer clicks the hidden floor switch.
        Given a 3x3 cave room,
        And a hidden floor switch placed at coordinates 2, 0,
        And an exit door placed at coordinates 1, 2,
        When a viewer clicks with UV coordinates 0.50625, 0.4667,
        Then the exit door will be opened.

    Scenario: The explorer heads for the exit when the exit door is opened.
        Given a 4x4 cave room,
        And a hidden floor switch placed at coordinates 1, 3,
        And an exit door placed at coordinates 3, 2,
        When the explorer is on Tile 1, 3,
        Then the exit door will be opened.
        And the explorer's goal is to reach Tile 3, 2.

    Scenario: The explorer finding treasure increases the current score.
        Given a 4x4 cave room,
        And some piece of treasure worth 500 points placed on coordinates 2, 2,
        When the explorer is on Tile 2, 2,
        Then the current score will be 500 points.

    Scenario: The current score increases when a viewer clicks on some treasure.
        Given a 3x3 cave room,
        And some piece of treasure worth 500 points placed on coordinates 2, 0,
        When a viewer clicks with UV coordinates 0.50625, 0.4667,
        Then the current score will be 500 points.

    Scenario: An armed trap is disarmed when a viewer clicks on it.
        Given a 3x3 cave room,
        And an armed trap placed at coordinates 2, 0,
        When a viewer clicks with UV coordinates 0.50625, 0.4667,
        Then the trap at Tile 2, 0 will be disarmed.

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
