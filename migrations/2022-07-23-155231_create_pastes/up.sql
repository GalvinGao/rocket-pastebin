create table pastes (
    id serial primary key,
    slug varchar(64) not null,
    delete_token varchar(255) not null,
    content text not null,
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp,
    deleted_at timestamp
);
create index delete_token_index on pastes (delete_token);
create index created_at_index on pastes (created_at);
create index updated_at_index on pastes (updated_at);