CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
drop table if exists ezy_course_c4;

create table ezy_course_c4
(
    id          uuid         NOT NULL,
    tutor_id    uuid         NOT NULL,
    name        varchar(140) NOT NULL,
    description text,
    format      varchar(30),
    structure   varchar(30),
    duration    varchar(30),
    price       INT,
    language    varchar(30),
    level       varchar(30),
    posted_time timestamp    NOT NULL default now(),
    created_at  timestamp    NOT NULL default now(),
    updated_at  timestamp    NOT NULL default now(),
    deleted_at  timestamp,
    PRIMARY KEY (id, created_at, updated_at),
    UNIQUE (id, created_at, updated_at)
);

/**
  LOAD DUMMY DATA
 */

insert into ezy_course_c4
values ('70c57639-680a-44e8-a15b-e879d38aa856', 'd709c2c9-eeb8-4b6b-a63d-25ef38c78e61', 'First Padawan Course', 'Jedy Rudimentaries', null, null, '72 years', 80, 'Hutt', 'Level 1', '2020-03-10 14:25:50', '2020-03-10 14:25:50', now(), null);
insert into ezy_course_c4
values (uuid_generate_v4(), 'd709c2c9-eeb8-4b6b-a63d-25ef38c78e61', 'Second Course', 'Force balance', null, null, '45 years', 80, 'Aurebesh', 'Level 1', now(), now(), now());