INSERT INTO user_ (username, email, uuid, password_hash) VALUES
('alice', 'alice@example.com', '019b1e35-f2a3-7270-bf69-559af5be14b2', '$argon2id$v=19$m=19456,t=2,p=1$z8LFtcIvrrl1QhQoYJs/Yw$IB6h0SqWw+sZuExdsx7Rofdy/IbQrCImB08goR27Tgk'),
('bob', 'bob@example.com', '019b1e36-3b8c-7f82-b845-6bfeb72466ce', '$argon2id$v=19$m=19456,t=2,p=1$mzO6Qx8ZH4/wrj14ZgKiuA$7bxNWCgsIVEfPgtueFbjbi8mDjbAHMYAHOGpxTJnEpQ'),
('carol', 'carol@example.com', '019b1e36-7706-76e2-b9ce-b37916ddfc99', '$argon2id$v=19$m=19456,t=2,p=1$5rw/7uIJIKMnyqNrYQt92Q$DJVEfgbaZtkflsmDEkSoR3uDQmujI4T73cWq9hOBgVI');

INSERT INTO room_ (owner, name, uuid) VALUES
(1, 'General Discussion', '5dc599ee-1f5c-40c2-a22a-e40780d2d960'),
(2, 'Tech Talk', '6b14fe7b-2171-4464-95af-4888062b1b6d'),
(1, 'Random Memes', 'fb794f59-6b2d-4daa-8980-dc5255862657');

INSERT INTO membership_ (user_id, room) VALUES
(1, 1),  -- Alice in General Discussion
(2, 1),  -- Bob in General Discussion
(2, 2),  -- Bob in Tech Talk
(3, 1),  -- Carol in General Discussion
(1, 3);  -- Alice in Random Memes

INSERT INTO message_ (sender, room, message_type, content) VALUES
(1, 1, 'text', 'Hey everyone, hows it going?'),
(2, 1, 'text', 'All good! Just trying to get through some work.'),
(3, 1, 'text', 'Hello! How are you guys?'),
(2, 2, 'text', 'Anyone seen the new tech updates?'),
(1, 3, 'image', 'Heres a funny meme I found!'),
(3, 1, 'text', 'I love how active this room is!');
