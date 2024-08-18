-- Add migration script here
create table history (probe_id text not null, system_info text, time_stamp datetime not null, status_code int not null);
create table probes (id text not null unique,  url text not null)