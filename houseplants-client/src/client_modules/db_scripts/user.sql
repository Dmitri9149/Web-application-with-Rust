\c web_client_db_a postgres

GRANT ALL ON SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO truuser;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public to truuser;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public to truuser;

drop table if exists web_user;

create table web_user
(
username varchar(20) primary key,
member_id INT,
user_password CHAR(100) not null
);

GRANT ALL PRIVILEGES ON TABLE web_user TO truuser;