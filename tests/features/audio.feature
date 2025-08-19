Feature: Audio can be played for certain aspects of the game.
    Scenario: All tracks can be detected from a folder.
        Given a song directory 'sample-songs',
        When the background player loads the songs from the directory,
        Then the song 'TylerSong3_Normal.wav' is found in the background songs.
        And the song 'song18.mp3' is found in the background songs.
        And the song 'song21.mp3' is found in the background songs.
        And the song 'Cleyton RX - Underwater.wav' is found in the background songs.

    Scenario: A picked song is Asset Server compliant for Bevy.
        Given a song directory 'sample-songs',
        When the background player loads the songs from the directory,
        And song 2 is picked from the background player,
        Then the picked song should be 'sample-songs/song18.mp3'.
