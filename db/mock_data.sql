INSERT INTO user_ (username, email, uuid, password_hash) VALUES
('alice', 'alice@example.com', '019b1e35-f2a3-7270-bf69-559af5be14b2', '$argon2id$v=19$m=19456,t=2,p=1$z8LFtcIvrrl1QhQoYJs/Yw$IB6h0SqWw+sZuExdsx7Rofdy/IbQrCImB08goR27Tgk'),
('bob', 'bob@example.com', '019b1e36-3b8c-7f82-b845-6bfeb72466ce', '$argon2id$v=19$m=19456,t=2,p=1$mzO6Qx8ZH4/wrj14ZgKiuA$7bxNWCgsIVEfPgtueFbjbi8mDjbAHMYAHOGpxTJnEpQ'),
('carol', 'carol@example.com', '019b1e36-7706-76e2-b9ce-b37916ddfc99', '$argon2id$v=19$m=19456,t=2,p=1$5rw/7uIJIKMnyqNrYQt92Q$DJVEfgbaZtkflsmDEkSoR3uDQmujI4T73cWq9hOBgVI');

INSERT INTO room_ (owner, name, global, uuid) VALUES
(1, 'General Discussion', true, '5dc599ee-1f5c-40c2-a22a-e40780d2d960'),
(2, 'Tech Talk', false, '6b14fe7b-2171-4464-95af-4888062b1b6d'),
(1, 'Random Memes', false, 'fb794f59-6b2d-4daa-8980-dc5255862657');

INSERT INTO membership_ (user_id, room) VALUES
(1, 1),  -- Alice in General Discussion
(2, 2),  -- Bob in Tech Talk
(3, 1),  -- Carol in General Discussion
(1, 3);  -- Alice in Random Memes

INSERT INTO message_ (sender, room, message_type, content, uuid) VALUES
(1, 1, 'text', 'Hey everyone, hows it going?', '3ae85002-8a82-479f-b1c9-6faa3dceb2f3'),
(2, 1, 'text', 'All good! Just trying to get through some work.', '8e60aa27-9eef-4f1c-a913-47ac6ea1229b'),
(3, 1, 'text', 'Hello! How are you guys?', 'f2b688f8-6678-465c-8092-9636a9ae2f16'),
(2, 2, 'text', 'Anyone seen the new tech updates?', '20c6b5d4-c8b1-4afe-844c-339e128fc344'),
(1, 3, 'image', 'Heres a funny meme I found!', '7dd79706-9187-47a5-b4f0-86e07cbb4564'),
(3, 1, 'text', 'I love how active this room is!', '9024823f-1b0c-436b-b81d-08dc06ac34df');

INSERT INTO friendship_ (user_first, user_second) VALUES
(1, 3),  -- Alice and Carol
(2, 3);  -- Bob and Carol

INSERT INTO friend_request_ (sender, receiver) VALUES
(2, 1);  -- Bob sent a friend request to Alice

INSERT INTO ws_token_ (token, room_id, expires_at) VALUES
('random_token_1', 1, '2025-12-31T23:59:59Z'),
('random_token_2', 2, '2025-12-31T23:59:59Z');

INSERT INTO room_invite_ (sender, receiver, room) VALUES
(2, 1, 2);
