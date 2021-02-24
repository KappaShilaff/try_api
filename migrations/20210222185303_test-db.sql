-- Add migration script here
create table test
(
    test_id serial,
    test_text text
);

create unique index test_test_id_uindex
	on test (test_id);

alter table test
    add constraint test_pk
        primary key (test_id);