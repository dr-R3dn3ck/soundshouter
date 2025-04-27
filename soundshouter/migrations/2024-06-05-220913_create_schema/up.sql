create table category
(
    id   INTEGER     not null
        primary key,
    name VARCHAR(50) not null
        unique
);

create table subcategory
(
    id          INTEGER     not null
        primary key,
    category_id INTEGER     not null
        references category,
    name        VARCHAR(50) not null
        unique,
    unique (category_id, name)
);

create table sound
(
    id             INTEGER      not null
        primary key,
    name           VARCHAR(50)  not null,
    path           VARCHAR(400) not null
        unique,
    duration       FLOAT        not null,
    play_count     INTEGER      not null,
    category_id    INTEGER
        references category,
    subcategory_id INTEGER
        references subcategory
);

