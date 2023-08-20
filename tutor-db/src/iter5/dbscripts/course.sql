CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
drop table if exists ezy_course_c4;

create table ezy_course_c4
(
    course_id uuid NOT NULL PRIMARY KEY,
    tutor_id uuid NOT NULL,
    course_name varchar(140) NOT NULL,
    posted_time timestamp NOT NULL default now()
);

/**
  LOAD DUMMY DATA
 */

 insert into ezy_course_c4 values (uuid_generate_v4(), 'd709c2c9-eeb8-4b6b-a63d-25ef38c78e61', 'First Course', '2020-03-10 14:25:50');
 insert into ezy_course_c4 values (uuid_generate_v4(), 'd709c2c9-eeb8-4b6b-a63d-25ef38c78e61', 'Second Course', now());