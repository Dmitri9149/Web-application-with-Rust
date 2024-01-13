\c houseplants_a postgres

GRANT ALL ON SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public to truuser;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public to truuser;

drop table if exists interesting_facts cascade;

create table interesting_facts
(
member_id INT not null,
content varchar(2000) not null,
ref_to_origin varchar(200),
posted_time TIMESTAMP default now()
);

/* Load seed data */ 
insert into interesting_facts(member_id, content, ref_to_origin)
values(1,'Methuselah, a Great Basin bristlecone pine (Pinus longaeva) 
in the White Mountains of California, has been measured by ring count 
to be 4,855 years old.', 'Google search');

insert into interesting_facts(member_id, content, ref_to_origin)
values(1,'The largest by wood volume and mass is the giant sequoia 
(Sequoiadendron giganteum), native to Sierra Nevada and California; 
it grows to an average height of 70–85 m (230–279 ft) and 5–7 m 
(16–23 ft) in diameter', 'https://en.wikipedia.org/wiki/List_of_largest_plants');


GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public to truuser;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public to truuser;
