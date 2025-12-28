CREATE TABLE IF NOT EXISTS user_ (
  id SERIAL PRIMARY KEY,
  uuid UUID UNIQUE,
  email TEXT UNIQUE,
  username TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS friendship_ (
  user_first INT NOT NULL REFERENCES user_(id) ON DELETE CASCADE,
  user_second INT NOT NULL REFERENCES user_(id) ON DELETE CASCADE,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  PRIMARY KEY (user_first, user_second)
);

CREATE TABLE IF NOT EXISTS friend_request_ (
  sender INT NOT NULL REFERENCES user_(id) ON DELETE CASCADE,
  receiver INT NOT NULL REFERENCES user_(id) ON DELETE CASCADE,
  sent_at TIMESTAMP NOT NULL DEFAULT now(),
  PRIMARY KEY (sender, receiver),
  CHECK (sender <> receiver)
);

CREATE TABLE IF NOT EXISTS room_ (
  id SERIAL PRIMARY KEY,
  uuid UUID UNIQUE,
  owner INT NOT NULL REFERENCES user_(id),
  name TEXT NOT NULL,
  global BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS membership_ (
  user_id INT REFERENCES user_(id),
  room INT REFERENCES room_(id),
  PRIMARY KEY (user_id, room)
);

CREATE TABLE IF NOT EXISTS room_invite_ (
  sender INT NOT NULL REFERENCES user_(id) ON DELETE CASCADE,
  receiver INT NOT NULL REFERENCES user_(id) ON DELETE CASCADE,
  room INT NOT NULL,
  sent_at TIMESTAMP NOT NULL DEFAULT now(),
  PRIMARY KEY (sender, receiver),
  CHECK (sender <> receiver)
);

CREATE TABLE IF NOT EXISTS message_ (
  id BIGSERIAL PRIMARY KEY,
  uuid UUID NOT NULL,
  sender INT REFERENCES user_(id) NOT NULL,
  room INT REFERENCES room_(id) NOT NULL,
  message_type VARCHAR(32) NOT NULL,
  content TEXT NOT NULL,
  sent_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE ws_token_ (
  token TEXT PRIMARY KEY,
  room_id INT NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL
);

-- Timestamp creation
CREATE OR REPLACE FUNCTION create_notification_timestamp()
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
EXECUTE FUNCTION create_notification_timestamp();

CREATE OR REPLACE TRIGGER insert_room_invite
BEFORE INSERT ON room_invite_
FOR EACH ROW
EXECUTE FUNCTION create_notification_timestamp();
