CREATE TABLE IF NOT EXISTS user (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS channel (
    id SERIAL PRIMARY KEY,
    owner INT NOT NULL REFERENCES user(id) ON DELETE CASCADE,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS membership (
  user INT REFERENCES user(id),
  channel INT REFERENCES channel(id),
  PRIMARY KEY (user, channel)
);

CREATE TABLE IF NOT EXISTS message (
  id BIGSERIAL PRIMARY KEY,
  sender INT REFERENCES user(id) NOT NULL,
  channel INT REFERENCES channel(id) NOT NULL,
  type VARCHAR(32) NOT NULL,
  content TEXT NOT NULL
);
