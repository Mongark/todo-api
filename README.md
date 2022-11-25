# Todo API
A simple Rust API for managing a todo list. It use Rocket for the API, and a MongoDB driver that connects to a local database. A database instance can be created by running the `spinup.sh` bash script.

## Routes

`POST /todo` Posts a todo in the database. Format: { name: String, checked: bool }.

`GET /todo/:path` Gets a todo with and id of 'path' from the database.
