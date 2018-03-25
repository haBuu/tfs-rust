CREATE TABLE contents (
  id INTEGER AUTO_INCREMENT PRIMARY KEY,
  page_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  content TEXT NOT NULL,
  version INTEGER NOT NULL,
  created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  comment TEXT,
  UNIQUE (page_id, version)
);