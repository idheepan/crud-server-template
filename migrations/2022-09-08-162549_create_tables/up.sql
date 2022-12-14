CREATE table nodes (
   id  bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
   description text
);
create table sensors (
  id bigint generated by default as identity primary key,
  created_at timestamp with time zone default timezone('utc'::text, now()) not null,
  connected_to bigint references nodes not null,
  sensor_type text not null
);
create table sensor_data (
  id bigint generated by default as identity primary key,
  inserted_at timestamp with time zone default timezone('utc'::text, now()) not null,
  measured_at timestamp with time zone default timezone('utc'::text, now()) not null,
  sensor_id bigint references sensors not null,
  values jsonb  not null
);