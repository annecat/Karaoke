INSERT INTO config (id, name, value)
VALUES
  (2, 'jukebox', 'no'),
  (3,'google_sheet_id', '1KWhp9nuuA4WrbEk2IssQUBVCPjVT6WX9gjuV9qFo7AI')
ON CONFLICT (id)
DO UPDATE SET value = EXCLUDED.value, name = EXCLUDED.name;
