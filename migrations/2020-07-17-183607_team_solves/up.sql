-- Create a table for solves
-- for each team

CREATE TABLE solves (
  id SERIAL PRIMARY KEY NOT NULL,
  challenge_id INTEGER NOT NULL,
  team_id INTEGER NOT NULL,
  CONSTRAINT solves_challenge_id_fkey
    FOREIGN KEY (challenge_id) REFERENCES challenges(id),
  CONSTRAINT solves_team_id_fkey
    FOREIGN KEY (team_id) REFERENCES teams(id)
);