INSERT INTO user (username, password_hash, email) VALUES
('alice', '$argon2id$v=19$m=19456,t=2,p=1$W0OzC/dmZQt7/xUJt4E9hA$cYiUC91a5yCQU9tDUadw0FKjUmTRv453cYwu1nfMKUQ', 'alice@example.com'),
('bob', '$argon2id$v=19$m=19456,t=2,p=1$1T7VaQps1X5Wj+TJHt8FIQ$/hA7PSITskjELwfNw+s6BvCJmUA4dDDrSGJvDvHx7Kc', 'bob@example.com'),
('carol', '$argon2id$v=19$m=19456,t=2,p=1$Kw4Re4lggxzDldu3vNl2PA$6DP4MPftfXI77g8EZRXYmWgcnVnAKLq0dkZOb/eBIC8', 'carol@example.com');

INSERT INTO channel (owner, name) VALUES
(1, 'General Discussion'),
(2, 'Tech Talk'),
(1, 'Random Memes');

INSERT INTO membership (user, channel) VALUES
(1, 1),  -- Alice in General Discussion
(2, 1),  -- Bob in General Discussion
(2, 2),  -- Bob in Tech Talk
(3, 1),  -- Carol in General Discussion
(1, 3);  -- Alice in Random Memes

INSERT INTO message (sender, channel, type, content) VALUES
(1, 1, 'text', 'Hey everyone, hows it going?'),
(2, 1, 'text', 'All good! Just trying to get through some work.'),
(3, 1, 'text', 'Hello! How are you guys?'),
(2, 2, 'text', 'Anyone seen the new tech updates?'),
(1, 3, 'image', 'Heres a funny meme I found!'),
(3, 1, 'text', 'I love how active this channel is!');
