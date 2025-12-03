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
        Then the size of the map should be 40 by 23 by 4.

    Scenario: The Hunger bar ticks down as time passes.
        Given a hunger bar set to 100% full,
        And the hunger bar decreases by 10% every second,
        When 5 seconds have passed,
        Then the hunger bar should be at 50%.
        And all campers should be alive.

    Scenario: The game ends when the hunger bar reaches zero.
        Given a hunger bar set to 100% full,
        And the hunger bar decreases by 10% every second,
        When 10 seconds have passed,
        Then the hunger bar should be at 0%.
        And all campers should be dead.

    Scenario: A list of objectives should show up depending on the loaded map.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        Then there should be 2 objectives.
        And the 1st objective should be called 'Seek sticks.'
        And the 2nd objective should be called 'Find food.'

    Scenario: A landmark should have a description and options.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,

        Then the name of the 1st landmark should be 'River'.
        And the description of the 1st landmark should be 'You stumble upon a river with water violently moving to the east.'

        And the objective of the 1st scenario from the 1st landmark should be 'Find food.'
        And the description of the 1st scenario from the 1st landmark should be 'There seem to be fish swimming in there. What do you do?'

        And the description of the 1st choice from the 1st scenario in the 1st landmark should be 'Try spearfishing with a stick nearby.'
        And the success result of the 1st choice from the 1st scenario in the 1st landmark should be 'You manage to impale a pretty big fish. Nice!'
        And the failure result of the 1st choice from the 1st scenario in the 1st landmark should be 'You try and try, but these fish keep avoiding your spear. Some water splashes you in the groin, and you feel quite ashamed.'

    Scenario: A failed objective should not show up on the contributions list.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And 'Player 1' fails the 2nd scenario's objective,
        Then 'Player 1 found food!' should not be in the contributions list.

    Scenario: A completed objective shows up on the contributions list.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And 'Player 1' succeeds the 2nd scenario's objective,
        Then 'Player 1 found food!' should be in the contributions list.

    Scenario: A camper heads into the meadows when a player attempts an objective.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And 'Player 1' attempts the 1st objective,
        Then the camper for 'Player 1' should appear outside of the bus.
        And the camper for 'Player 1' should be heading into the meadows.

    Scenario: A camper is no longer seen once they finish traveling to any destination.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And 'Player 1' attempts the 1st objective,
        And 'Player 1' finishes traveling,
        Then there should be 0 campers present.

    Scenario: A camper heads back from the meadows when completing an objective.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And 'Player 1' succeeds the 1st scenario's objective,
        Then the camper for 'Player 1' should appear outside of the meadows.
        And the camper for 'Player 1' should be heading back to the bus.

    Scenario: A camper heads back from the meadows when failing an objective.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And 'Player 1' fails the 1st scenario's objective,
        Then the camper for 'Player 1' should appear outside of the meadows.
        And the camper for 'Player 1' should be heading back to the bus.

    Scenario: All campers head back to the bus when all objectives are completed.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And 'Player 1' attempts the 1st objective,
        And 'Player 2' attempts the 1st objective,
        And all objectives are completed,
        Then the camper for 'Player 1' should appear outside of the meadows.
        And the camper for 'Player 1' should be heading back to the bus.
        And the camper for "Player 2" should appear outside of the meadows.
        And the camper for "Player 2" should be heading back to the bus.

    Scenario: The bus leaves when all campers are finished with their objectives and back in the bus.
        Given a campsite map called 'campsite_1.tmx',
        When the campsite map is rendered,
        And all objectives are completed,
        And all campers are on the bus,
        Then the bus should be heading to the exit.
