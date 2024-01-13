\c houseplants_a postgres

GRANT ALL ON SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public to truuser;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public to truuser;

drop table if exists interesting_fact cascade;

create table interesting_fact
(
  fact_id serial primary key,
  member_id INT not null,
  content varchar(2000) not null,
  ref_to_origin varchar(200),
  posted_time TIMESTAMP default now()
);

/* Load seed data */ 
insert into interesting_fact 
(
  fact_id, member_id, content, ref_to_origin, posted_time
)
values(1, 1,
 '(May be) the oldest pot plant in the world. It is a giant cycad or Encephalartos altensteinii, native to the Eastern Cape province of South Africa and brought to the UK in 1775 by the botanist Francis Masson.',
 'Google search',
 '2023-04-12 05:40:02');

insert into interesting_fact
(
  fact_id, member_id, content, ref_to_origin, posted_time
)
values( 2, 1,
  'The largest plant by wood volume and mass is the giant sequoia (Sequoiadendron giganteum), native to Sierra Nevada and California; it grows to an average height of 70–85 m (230–279 ft) and 5–7 m (16–23 ft) in diameter.',
  'https://en.wikipedia.org/wiki/List_of_largest_plants',
  '2023-04-12 05:45:02');


GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public to truuser;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public to truuser;
