-- Add migration script here

create table if not exists quotes (
    id uuid primary key,
    book varchar not null,
    quote text not null,
    inserted_at timestamp not null,
    updated_at timestamp not null,
    unique (book, quote)
);
