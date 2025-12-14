# Chat App

## Configuration

- Specify the JWT secret with the `CHATAPP_JWT_SECRET` environment variable.
- Specify the server's port with the `CHATAPP_PORT` environment variable. Defaults to `8080`.
- To enable user registration, pass in `CHATAPP_ALLOW_REGISTRATION=true`.

## Development

You can use this to setup the database:

```sh
./run_db.sh
```
