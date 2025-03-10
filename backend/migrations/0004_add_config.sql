CREATE TABLE IF NOT EXISTS config (
  id serial PRIMARY KEY,
  name TEXT,
  value TEXT
);

-- Insert values
INSERT INTO config (id, name, value)
VALUES
(1, 'open', 'no');

