CREATE TABLE IF NOT EXISTS current_playlist (
  id serial PRIMARY KEY,
  artist TEXT NOT NULL,
  title TEXT NOT NULL,
  lyrics_url TEXT,
  singer TEXT
);
