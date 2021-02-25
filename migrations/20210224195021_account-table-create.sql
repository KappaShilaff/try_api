-- Add migration script here
create table accounts
(
    uid VARCHAR(255) not null,
    exchange TEXT,
    data_to_sign INTEGER[],
    api_key TEXT,
    sign_key TEXT
);

create unique index accounts_uid_uindex
	on accounts (uid);

alter table accounts
    add constraint accounts_pk
        primary key (uid);