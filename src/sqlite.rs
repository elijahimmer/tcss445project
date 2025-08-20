//! The SQLite Database backend!
//!

use bevy::prelude::*;
use rusqlite::Connection;

const ADD_SCHEMA: &str = "
    BEGIN TRANSACTION;
    PRAGMA foreign_keys = ON;

    -- Schema

    CREATE TABLE pokemon(
      pokemon_id     INTEGER PRIMARY KEY CHECK(pokemon_id > 0),
      name           TEXT NOT NULL UNIQUE COLLATE NOCASE,
      primary_type   TEXT NOT NULL DEFAULT 'Normal',
      secondary_type TEXT DEFAULT NULL
    ) STRICT;

    CREATE TABLE egg_group(
      egg_group_id INTEGER PRIMARY KEY AUTOINCREMENT CHECK(egg_group_id > 0),
      name         TEXT NOT NULL UNIQUE COLLATE NOCASE
    ) STRICT;

    CREATE TABLE pokemon_egg_group(
      pokemon_id   INTEGER,
      egg_group_id INTEGER,
      PRIMARY KEY(pokemon_id, egg_group_id),
      FOREIGN KEY(pokemon_id)   REFERENCES pokemon(pokemon_id) ON DELETE CASCADE ON UPDATE CASCADE,
      FOREIGN KEY(egg_group_id) REFERENCES egg_group(egg_group_id) ON DELETE CASCADE ON UPDATE CASCADE
    ) STRICT;

    CREATE TABLE move(
      move_id  INTEGER PRIMARY KEY CHECK(move_id > 0),
      name     TEXT NOT NULL UNIQUE COLLATE NOCASE,
      type     TEXT NOT NULL DEFAULT 'Normal',
      category TEXT NOT NULL CHECK(category in ('Status', 'Physical', 'Special')) DEFAULT 'Physical',
      power    INTEGER CHECK(power > 0 OR power IS NULL) DEFAULT NULL,
      accuracy INTEGER CHECK(accuracy > 0 OR accuracy IS NULL) DEFAULT 100
    ) STRICT;

    CREATE TABLE pokemon_move(
      pokemon_id INTEGER,
      move_id    INTEGER,
      method     TEXT NOT NULL,
      PRIMARY KEY(pokemon_id, move_id),
      FOREIGN KEY(pokemon_id) REFERENCES pokemon(pokemon_id) ON DELETE CASCADE ON UPDATE CASCADE,
      FOREIGN KEY(move_id)    REFERENCES move(move_id) ON DELETE CASCADE ON UPDATE CASCADE
    ) STRICT;

    -- Data

    -- Insert Egg Groups
    INSERT INTO egg_group (name) VALUES
      ('Monster'), ('Grass'), ('Bug'), ('Normal'), ('Psychic');

    -- Insert Pokémon
    INSERT INTO pokemon (pokemon_id, name, primary_type, secondary_type) VALUES
      (1, 'Bulbasaur', 'Grass', 'Poison'),
      (2, 'Ivysaur', 'Grass', 'Poison'),
      (3, 'Venusaur', 'Grass', 'Poison'),
      (4, 'Charmander', 'Fire', NULL),
      (5, 'Charmeleon', 'Fire', NULL),
      (6, 'Charizard', 'Fire', 'Flying'),
      (7, 'Squirtle', 'Water', NULL),
      (8, 'Wartortle', 'Water', NULL),
      (9, 'Blastoise', 'Water', NULL),
      (10, 'Caterpie', 'Bug', NULL),
      (150, 'MewTwo', 'Psychic', NULL),
      (132, 'Ditto', 'Normal', NULL);

    -- Link Pokémon to Egg Groups
    -- Bulbasaur line: Monster + Grass
    INSERT INTO pokemon_egg_group (pokemon_id, egg_group_id)
      SELECT 1, egg_group_id FROM egg_group WHERE name IN ('Monster', 'Grass');
    INSERT INTO pokemon_egg_group (pokemon_id, egg_group_id)
      SELECT 2, egg_group_id FROM egg_group WHERE name IN ('Monster', 'Grass');
    INSERT INTO pokemon_egg_group (pokemon_id, egg_group_id)
      SELECT 3, egg_group_id FROM egg_group WHERE name IN ('Monster', 'Grass');

    -- Charmander line: Monster + Dragon (Dragon not in top 10, so skipped)
    INSERT INTO pokemon_egg_group (pokemon_id, egg_group_id)
      SELECT 4, egg_group_id FROM egg_group WHERE name = 'Monster';
    INSERT INTO pokemon_egg_group (pokemon_id, egg_group_id)
      SELECT 5, egg_group_id FROM egg_group WHERE name = 'Monster';
    INSERT INTO pokemon_egg_group (pokemon_id, egg_group_id)
      SELECT 6, egg_group_id FROM egg_group WHERE name = 'Monster';

    -- Squirtle line: Monster + Water 1 (not inserted here)
    INSERT INTO pokemon_egg_group (pokemon_id, egg_group_id)
      SELECT 7, egg_group_id FROM egg_group WHERE name = 'Monster';
    INSERT INTO pokemon_egg_group (pokemon_id, egg_group_id)
      SELECT 8, egg_group_id FROM egg_group WHERE name = 'Monster';
    INSERT INTO pokemon_egg_group (pokemon_id, egg_group_id)
      SELECT 9, egg_group_id FROM egg_group WHERE name = 'Monster';

    -- Caterpie line: Bug
    INSERT INTO pokemon_egg_group (pokemon_id, egg_group_id)
      SELECT 10, egg_group_id FROM egg_group WHERE name = 'Bug';

    -- Ditto.
    INSERT INTO pokemon_egg_group (pokemon_id, egg_group_id)
      SELECT 132, egg_group_id FROM egg_group;

    -- Insert Egg Moves (example egg moves from Gen 2+)
    INSERT INTO move (name, type, category, power, accuracy) VALUES
      ('Amnesia', 'Psychic', 'Status', NULL, NULL),
      ('Skull Bash', 'Normal', 'Physical', 130, 100),
      ('Dragon Dance', 'Dragon', 'Status', NULL, NULL),
      ('Fake Out', 'Normal', 'Physical', 40, 100),
      ('Haze', 'Ice', 'Status', NULL, NULL),
      ('Mirror Coat', 'Psychic', 'Special', NULL, 100);

    -- Add egg moves
    -- Bulbasaur line: Amnesia and Skull Bash
    INSERT INTO pokemon_move (pokemon_id, move_id, method)
      SELECT 1, move_id, 'egg' FROM move WHERE name = 'Amnesia';
    INSERT INTO pokemon_move (pokemon_id, move_id, method)
      SELECT 1, move_id, 'egg' FROM move WHERE name = 'Skull Bash';

    -- Charmander line: Dragon Dance
    INSERT INTO pokemon_move (pokemon_id, move_id, method)
      SELECT 4, move_id, 'egg' FROM move WHERE name = 'Dragon Dance';

    -- Squirtle line: Fake Out and Haze
    INSERT INTO pokemon_move (pokemon_id, move_id, method)
      SELECT 7, move_id, 'egg' FROM move WHERE name = 'Fake Out';
    INSERT INTO pokemon_move (pokemon_id, move_id, method)
      SELECT 7, move_id, 'egg' FROM move WHERE name = 'Haze';

    -- Caterpie has no egg moves (it can’t breed in later gens either)
    COMMIT;
    ";

pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn open() -> Result<Self, rusqlite::Error> {
        let db = Self {
            connection: Connection::open_in_memory()?,
        };

        db.connection.execute_batch(ADD_SCHEMA)?;

        Ok(db)
    }
}

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(
            Database::open()
                .inspect_err(|e| error!("Failed to open database with: {e}"))
                .unwrap(),
        );
    }
}
