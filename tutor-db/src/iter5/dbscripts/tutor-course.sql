CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
drop table if exists ezy_course_c4 cascade;
drop table if exists ezy_tutors cascade;
drop table if exists tutors cascade;

create table tutors
(
    id         uuid UNIQUE  NOT NULL,
    name       varchar(200) NOT NULL,
    pic_url    varchar(200) NOT NULL,
    profile    text         NOT NULL,
    created_at timestamp    NOT NULL default now(),
    updated_at timestamp    NOT NULL default now(),
    deleted_at timestamp,
    PRIMARY KEY (id, created_at, updated_at),
    UNIQUE (id, created_at, updated_at)
);

create table ezy_course_c4
(
    id          uuid UNIQUE  NOT NULL,
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
    UNIQUE (id, tutor_id, created_at, updated_at),
    CONSTRAINT fk_tutor
        FOREIGN KEY (tutor_id)
            REFERENCES tutors (id)
            ON DELETE cascade
);

/**
  LOAD DUMMY DATA
 */

insert into tutors
values ('d709c2c9-eeb8-4b6b-a63d-25ef38c78e61', 'Rogerio Bacon',
        'https://i.pravatar.cc/150?u=d709c2c9-eeb8-4b6b-a63d-25ef38c78e61',
        'Roger Bacon (Ilchester, c. 1214-Oxford, 1294) fue un filósofo, protocientífico y teólogo escolástico inglés, de la orden franciscana (tradicionalmente, su nombre se cita seguido por las siglas O.F.M.). Es conocido por el sobrenombre de Doctor Mirabilis (‘Doctor Admirable’, en latín).')
insert into tutors
values ('d709c2c9-eeb8-4b6b-a63d-25ef38c78e62', 'Baruch Spinoza',
        'https://i.pravatar.cc/150?u=d709c2c9-eeb8-4b6b-a63d-25ef38c78e62',
        'Baruch Spinoza (Ámsterdam, 24 de noviembre de 1632-La Haya, 21 de febrero de 1677) fue un filósofo neerlandés de origen sefardí hispano-portugués. También se le conoce como Baruj, Bento, Benito, Benedicto o Benedictus (de) Spinoza o Espinosa, según las distintas traducciones de su nombre basadas en las hipótesis sobre su origen. Heredero crítico del cartesianismo, es considerado uno de los tres grandes racionalistas de la filosofía del siglo xvii, junto al francés René Descartes y el alemán Gottfried Leibniz, con quien además tuvo una pequeña correspondencia.')

insert into ezy_course_c4
values ('70c57639-680a-44e8-a15b-e879d38aa856', 'd709c2c9-eeb8-4b6b-a63d-25ef38c78e61', 'Rudimentos de la óptica',
        '3 o 4 cosas que tenés que saber al mirar por un vidrio cóncavo', null, null, '72 años y medio', 80,
        'Inglés del siglo 12', 'Level 74 y 3/4', '2020-03-10 14:25:50', '2020-03-10 14:25:50', now(), null);
insert into ezy_course_c4
values (uuid_generate_v4(), 'd709c2c9-eeb8-4b6b-a63d-25ef38c78e61', 'El bolazo de la pólvora',
        'Le expansión de los gases ante un blast', null, null, '45 years', 80, 'Chino Tradicional', 'Level 1', now(),
        now(), now());
insert into ezy_course_c4
values (uuid_generate_v4(), 'd709c2c9-eeb8-4b6b-a63d-25ef38c78e62', 'Curriculum Contranaturae',
        '3 o 4 cosas que tenés que saber sobre la ética', null, null, 'con 2 minutos sobra', 5,
        'Español Presocrático 😂', 'Level 78', now(), now(), now());