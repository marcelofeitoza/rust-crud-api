# Rust CRUD Api

This is a simple CRUD API built with Rust, Prisma-Client-Rust, and Actix.

---

## Running

-   Install Rust
-   Create a `.env` file in the root directory and add the following:

```
MONGODB_URL="<your mongodb url>"
```

-   Run `cargo dev` to start the server


## Endpoints

-   `GET /users` - Get all users

<br/>

-   `GET /users/:id` - Get a user by id
-   -   id is the id of the user - a string

<br/>

-   `POST /users/create` - Create a user
-   -   body should be a json object with the following properties:
-   -   `name` - a string
-   -   `username` - a string
-   -   `email` - a string
-   -   `password` - a string

<br/>

-   `PUT /users/update/:id` - Update a user
-   -   id is the id of the user - a string
-   -   body should be a json object with the following properties:
-   -   `name` - a string
-   -   `username` - a string
-   -   `email` - a string
-   -   `password` - a string

<br/>

-   `DELETE /users/delete/:id` - Delete a user
-   -   id is the id of the user - a string



## License

MIT