![Elide logo](./docs/images/elide-logo.png | width=72)
# elide-router
ReST API backend written in rust for [elide.me](https://console.elide.me), provides:

- Authentication and User management
  - Login
  - Register
  - Delete
  - Editing user info
- Routes management
  - Creating Routes
  - Editing Routes
  - Deleting Routes
- Routing
  - Redirects to the target domain based on a route

## Develop

### Requirements

- `make`
- `docker`
- `docker-compose`

### Instructions

To develop with auto compiliation on changes, run

```sh
make watch
```

When in container, run

```
cargo watch -x run
```

This will run the server at `localhost:9600` and then you can either use `elide-app` or Postman to test the server.

## Stop

If your are in docker container, To stop the server exit the containers terminal by typing `exit` or <kbd>ctrl</kbd> + <kbd>d</kbd>

When in your normal terminal Stop the server by running

```
make stop
```
