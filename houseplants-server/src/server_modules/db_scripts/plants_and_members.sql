\c houseplants_a postgres

GRANT ALL ON SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public to truuser;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public to truuser;

drop table if exists plant cascade;
drop table if exists member cascade;

create table plant
(
plant_id serial primary key,
member_id INT not null,
plant_name varchar(140) not null,
plant_description varchar(2000),
plant_care varchar(2000),
plant_care_difficulty varchar(140),
plant_alternative_name varchar(200),
plant_price INT,
plant_extra_info varchar(1000),
posted_time TIMESTAMP default now()
);

create table member(
    member_id serial primary key,
    member_name varchar(200) not null,
    member_info varchar(200) not null
);

/* Load seed data for testing */
insert into member(member_id, member_name, member_info)
values(1,'Dmitri', 'Dmitri likes orchids');

insert into member(member_id, member_name, member_info)
values(2,'Alice', 'Alice likes succulents');
insert into plant
(plant_id,member_id, plant_name,plant_extra_info, posted_time)
values(1, 1, 'Orchid', 'Very interesting but difficult plant' , '2021-04-12 05:40:00');
insert into plant
(plant_id, member_id, plant_name, plant_extra_info, posted_time)
values(2, 1, 'Cactus', 'Be careful with watering, easy in growing', '2021-04-12 05:45:00');

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public to truuser;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public to truuser;
