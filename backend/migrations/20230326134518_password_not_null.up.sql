CREATE TABLE
  users_tmp (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR(250) UNIQUE NOT NULL,
    password_hash VARCHAR(250) NOT NULL
  );

INSERT INTO
  users_tmp
SELECT
  *
FROM
  users;

DROP TABLE users;

ALTER TABLE users_tmp
RENAME TO users;
