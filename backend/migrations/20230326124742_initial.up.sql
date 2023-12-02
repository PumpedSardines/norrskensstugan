CREATE TABLE
  users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR(250) UNIQUE NOT NULL,
    password_hash VARCHAR(250)
  )
