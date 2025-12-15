CREATE TABLE IF NOT EXISTS user_ (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE,
    email TEXT UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS room_ (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE,
    owner INT NOT NULL REFERENCES user_(id) ON DELETE CASCADE,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS membership_ (
  user_id INT REFERENCES user_(id),
  room INT REFERENCES room_(id),
  PRIMARY KEY (user_id, room)
);

CREATE TABLE IF NOT EXISTS message_ (
  id BIGSERIAL PRIMARY KEY,
  sender INT REFERENCES user_(id) NOT NULL,
  room INT REFERENCES room_(id) NOT NULL,
  message_type VARCHAR(32) NOT NULL,
  content TEXT NOT NULL,
  sent_at TIMESTAMP
);

-- Message timestamp creation
CREATE OR REPLACE FUNCTION create_message_timestamp()
RETURNS trigger
AS $$
BEGIN
  NEW.sent_at := CURRENT_TIMESTAMP;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER insert_message
BEFORE INSERT ON message_
FOR EACH ROW
EXECUTE FUNCTION create_message_timestamp();
