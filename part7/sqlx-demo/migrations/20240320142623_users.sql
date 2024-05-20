-- Add migration script here
CREATE TABLE `users` (
     `id` int NOT NULL AUTO_INCREMENT,
     `name` varchar(128) NOT NULL,
     `age` int NOT NULL,
     `id_card` varchar(128) NOT NULL,
     `last_update` date NOT NULL,
     PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
