-- postgresql

DROP TABLE IF EXISTS collectibles;

CREATE TABLE collectibles (
    kind VARCHAR(255) NOT NULL,

    name VARCHAR(255) NOT NULL PRIMARY KEY,
    quote VARCHAR(255),
    quality INT,

    unlock VARCHAR(255), 
    item_type VARCHAR(255),
    recharge_time VARCHAR(255),
    item_pool VARCHAR(255),

    description VARCHAR(2048) NOT NULL,

    wiki_link VARCHAR(255) NOT NULL
);