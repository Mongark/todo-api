# Todo API
A simple API for managing a to-do list. It use Rocket for the API, and a MongoDB driver that connects to a local database.

A database instance can be created by running the `spinup.sh` bash script. It requires docker-compose to work.

## Routes

* `POST /todo` Posts a todo in the database. Format: { name: String, checked: bool }.

* `GET /todo/:path` Gets a todo with id of 'path' from the database.

* `PUT /todo/:path <data>` Updates a todo with id of 'path' from the database with the data in \<data\>.

* `DELETE /todo/:path` Deletes a todo with id of 'path' from the database.

* `Get /todos` Gets all todos in the database.
