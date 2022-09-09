# CRUD Server template
## Features
This templates uses diesel and rocket
Creates multiple tables and has references
Various PG datatypes including Timestamptz, Text, nullable fields and JSONB
Uses Diesel 2.0

## Setup
Add the path of libpq as an environment variable `PQ_LIB_DIR: C:\Program Files\PostgreSQL\14\lib`

Install `diesel-cli`

Run `diesel setup --database-url='postgres://postgres:11332@localhost/rws'`

Rocket db pools do not support diesel version 2.0 yet. The change is simple enough. In Cargo.toml file at Rocket\contrib\sync_db_pools\lib\Cargo.toml, change diesel version to 2.0 from 1.0 and build.

Set the build rocket lib and sync_db_pools lib path


