# SmolURL
Allows you to create short URL's and track the dynamics of redirects(clicks).

## Private Key Generation
Private key is required for JWT signature generation. `SmolURL` binary require path to `.pem` file, by default `private.pem` is used.
```sh
openssl genrsa -out private.pem 2048
```

## Build with Docker
```sh
docker build --tag smolurl .
```

## Testing
Run local [PostgreSQL](https://hub.docker.com/_/postgres) instance(docker):
```sh
docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
```

Run tests with:
```sh
RUST_BACKTRACE=1 cargo test -- --nocapture --test-threads=1
```

## API Schema
### `POST /auth/register`
#### Overview
Basic JWT-based registration. Creates `User` entry in database and returns `Authorization Bearer Token`.
#### Schema
```json
{
    "username": "super1337",
    "password": "1337"
}
```

### `POST /auth/login`
#### Overview
Basic JWT-based login system. Validates `Authorization Bearer Token` and password hash.
#### Schema
```json
{
    "username": "super1337",
    "password": "1337"
}
```

### `POST /api/link`
#### Overview
Protected endpoint. Creates new `Link` entry in database and returns it. `Link` entry is associated with `User`. Key(short URL) is generated based on unique `User` id and personal url counter - encoded with base64.
#### Schema
```json
{
    "url": "http://example.com"
}
```

### `GET /api/links`
#### Overview
Protected endpoint. Returns all `User` `Link`'s.

### `GET /api/link/{key}`
#### Overview
Protected endpoint. Returns specific `User` `Link` by unique key. `Link` must be owned by `User`.

### `GET /ping`
#### Overview
It is what it is.

### `Link`
```json
{
    "id": 0,
    "url": "http://example.com",
    "key": "0dzFGd",
    "userid": 0,
    "count": 0
}
```
