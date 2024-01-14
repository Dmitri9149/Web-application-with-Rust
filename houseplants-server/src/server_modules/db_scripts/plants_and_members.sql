\c houseplants_a postgres

-- GRANT ALL ON SCHEMA public TO truuser;
-- GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO truuser;
-- GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public to truuser;
-- GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public to truuser;

drop table if exists plant cascade;
drop table if exists member cascade;

create table plant
(
plant_id serial primary key,
member_id INT not null,
plant_name varchar(150) not null,
plant_description varchar(3000),
plant_care varchar(2000),
plant_care_difficulty varchar(100),
plant_alternative_name varchar(300),
plant_price INT,
plant_extra_info varchar(2000),
posted_time TIMESTAMP default now()
);

create table member(
    member_id serial primary key,
    member_name varchar(100) not null,
    member_info varchar(200) not null
);

/* Load seed data */ 
insert into member(member_id, member_name, member_info)
values(1,'Dmitri', 'Likes orchids');

insert into member(member_id, member_name, member_info)
values(2,'Alice', 'Likes succulents');

insert into plant
(
    plant_id,member_id, plant_name, plant_extra_info, posted_time, 
    plant_description, plant_care, plant_care_difficulty, 
    plant_alternative_name, plant_price 

)
values(1, 1, 'Orchid', 'Very interesting but difficult plant' , '2023-04-12 05:40:02',
'Tropical plant, exists in many colors and forms', 'Takes care', 'Difficult', 
'Orchid', 0);

insert into plant
(
    plant_id,member_id, plant_name, plant_extra_info, posted_time, 
    plant_description, plant_care, plant_care_difficulty, 
    plant_alternative_name, plant_price 

)
values(2, 1, 'Cactus', 'Very interesting and not difficult plant' , '2023-04-12 05:40:50',
'Exists in many forms', 'Be careful with watering', 'Eazy', 
'Cactus', 0);

insert into plant
(
    plant_id,member_id, plant_name, plant_extra_info, posted_time, 
    plant_description, plant_care, plant_care_difficulty, 
    plant_alternative_name, plant_price 

)
values(3, 2, 'Crassula ovata', 'Very interesting and very popular house plant' , '2023-04-12 05:42:50',
'a succulent plant with small pink or white flowers that is native to the KwaZulu-Natal and Eastern Cape provinces of South Africa', 'Be careful with watering', 'Easy', 
'Jade plant, Money plant', 0);

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public to truuser;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public to truuser;
