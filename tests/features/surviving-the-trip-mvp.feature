Feature: All of the basic features are implemented for Surviving the Trip.
    Scenario: The campsite tiled map is rendered.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        # This is derived from the following:
        # world_tile_width = map_width / tile_width
        # world_tile_height = map_height / tile_width
        #
        # map_width = 1280, tile_width = 32 => 1280 / 32 = 40
        # map_height = 720, tile_width = 32=> 720 / 32 = 22.5 or about 23
        # map_depth = number of layers = 3 (Overworld, Trees and Details, Bus)
        Then the size of the map should be 40 by 23 by 3.

    Scenario: The Hunger bar ticks down as time passes.
        Given a hunger bar with a duration of 10 seconds,
        When 5 seconds have passed,
        Then the hunger bar should be at 50%.
        And all campers should still be alive.

    Scenario: The game ends when the hunger bar reaches zero.
        Given a hunger bar with a duration of 10 seconds,
        When 10 seconds have passed,
        Then the hunger bar should be at 0%.
        And all campers should be dead.

    Scenario: Some player should have a prompt when attempting an objective.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And "Player 1" attempts the 2nd objective,
        Then the prompt should have the description "There seem to be fish swimming in there."
        And the prompt should ask "What do you do?" with the options "Try spearfishing with a stick nearby, Look for dead fish along the river, Collect some water in your flask and continue elsewhere."

    Scenario: A list of objectives should show up depending on the loaded map.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        Then there should be two objectives.
        And the 1st objective should be "Seek sticks."
        And the 2nd objective should be "Find food."

    Scenario: An attempted objective shows up on the contributions list.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And "Player 1" attempts the 1st objective,
        Then "Player 1 found food!" should be in the contributions list.

    Scenario: A completed objective shows up on the contributions list.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And "Player 1" succeeds in the 1st objective,
        Then "Player 1 found food!" should be in the contributions list.

    Scenario: A camper heads into the meadows when a player attempts an objective.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And "Player 1" attempts the 1st objective,
        Then the camper for "Player 1" should appear outside of the bus.
        And the camper for "Player 1" should be heading into the meadows.

    Scenario: A camper heads back from the meadows when completing an objective.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And "Player 1" succeeds in the 1st objective,
        Then the camper for "Player 1" should appear outside of the meadows.
        And the camper for "Player 1" should be heading back to the bus.

    Scenario: A camper heads back from the meadows when failing an objective.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And "Player 1" fails in the 1st objective,
        Then the camper for "Player 1" should appear outside of the meadows.
        And the camper for "Player 1" should be heading back to the bus.

    Scenario: All campers head back to the bus when all objectives are completed.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And "Player 1" attempts the 1st objective,
        And "Player 2" attempts the 1st objective,
        And all objectives are completed,
        Then the camper for "Player 1" should appear outside of the meadows.
        And the camper for "Player 1" should be heading back to the bus.
        And the camper for "Player 2" should appear outside of the meadows.
        And the camper for "Player 2" should be heading back to the bus.

    Scenario: The bus leaves when all campers are finished with their objectives and back in the bus.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And all objectives are completed,
        And all campers are on the bus,
        Then the bus should be heading to the exit.
