CREATE TABLE
  login_tokens (
    id INTEGER PRIMARY KEY,
    token VARCHAR(250) UNIQUE NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
  );
