# ps-data
An app for data collection.

## Setup
Steps to set up and run this app.

### Rust
This app is built with Rust, so you'll need to install Rust to run it.

Learn how to install Rust [here](https://www.rust-lang.org/tools/install).

### Database Installation
This app uses a Postgres 15 database.
To simplify setup, we're going to run postgres in a docker container, so you'll need to install docker and docker-compose.

First install Docker.
You can find instructions [here](https://docs.docker.com/get-docker/).

Next, install `docker-compose`.
Instructions can be found [here](https://docs.docker.com/compose/install/).

### Cargo Make
Cargo Make is used to manage tasks for this project.

Learn how to install Cargo Make [here](https://github.com/sagiegurari/cargo-make#installation).

### Database Setup
For docker to initialize the database for this project, it will need some metadata.
We will provide this through a file called `.env`.
Create this file in the root of the project.
Within the file, add the following key-value pairs:

```
DB_PROTOCOL=postgres
POSTGRES_PORT=5432
POSTGRES_USER=psdata_app
POSTGRES_PASSWORD=passw0rd
POSTGRES_DB=psdata_db
```

You may replace the values for DB_USER, DB_PASSWORD, and DB_NAME with whatever you like.
When you start up the project for the first time, these settings will be used to configure your database.

## Running the App
To start the app, run `cargo make start`.
This will start the database and the backend service.

To test if it's running, try clicking this link: http://localhost:8080.

You should see "Server is running." printed on the screen.

## Shutting Down the App
After you shut down the app, be sure to run `cargo make stop_db` to shutdown the database