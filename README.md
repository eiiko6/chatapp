# quick rust platform

## Setup

Change the project's name in all files in one command:

```sh
./init.sh new_name
```

## Configuration

- Specify the JWT secret with the `_JWT_SECRET` environment variable.
- Specify the server's port with the `_PORT` environment variable. Defaults to `8080`.
- To enable user registration, pass in `_ALLOW_REGISTRATION=true`.
- `_DATA_DIR=/var/lib/-server/` will set the location for the hosted files to `/var/lib/-server/`.

## Development

You can use this to setup the database:

```sh
./run_db.sh
```
