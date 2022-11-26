# ps-data
An app for data collection.

## Setup
Steps to set up and run this app.

### Rust
This app is built with Rust, so you'll need to install Rust to run it.

Learn how to install Rust [here](https://www.rust-lang.org/tools/install).

### Database Installation
This app uses a Postgres 15 database.

Download and install [here](https://www.postgresql.org/download/).

### Cargo Make
Cargo Make is used to manage tasks for this project.

Learn how to install Cargo Make [here](https://github.com/sagiegurari/cargo-make#installation).

### Database Setup
To initialize the database for this project, first provide a filed called `.env` in the root of this project.
Within the file, add the following key-value pairs:

```
DB_PROTOCOL=postgres
POSTGRES_PORT=5432
POSTGRES_USER=psdata_app
POSTGRES_PASSWORD=passw0rd
POSTGRES_DB=psdata_db
```

You may replace the values for DB_USER, DB_PASSWORD, and DB_NAME with whatever you like.

After that, run `cargo make init_db`.
This command will initialize the database with the user, password, and database name specified in the above variables.

## Running the App
To start the app, run `cargo make start_back`.

To test if it's running, try clicking this link: http://localhost:8080.

You should see "Server is running." printed on the screen.
