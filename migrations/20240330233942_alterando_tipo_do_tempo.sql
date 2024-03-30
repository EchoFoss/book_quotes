-- Add migration script here
alter table quotes
    alter column inserted_at set data type timestamptz,
    alter column updated_at set data type timestamptz
;
