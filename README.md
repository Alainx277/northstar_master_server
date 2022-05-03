
# Northstar Master Server

This is a rewrite of the [original northstar master server](https://github.com/R2Northstar/NorthstarMasterServer) in Rust.

## Development

- [Install Rust](https://www.rust-lang.org/tools/install)
- Install `sqlx-cli`
    ```
    cargo install sqlx-cli
    ```
- Checkout the repo
    ```
    git clone ...
    ```
- Create the database
    ```
    sqlx database setup
    ```
- Run the server (this will take a while for the first compilation)
    ```
    source .env # You can also set the env vars manually
    cargo run
    ```

### Changing the schema

Changes are done using plain SQL migrations (located in [migrations/](migrations)).

Create a migration:
```
sqlx migrate add <name>
```

Run migrations:
```
sqlx migrate run
```

## Testing

### Manually

In `Titanfall2\R2Northstar\mods\Northstar.CustomServers\mod\cfg\autoexec_ns_server.cfg` and `Titanfall2\R2Northstar\mods\Northstar.CustomServers\mod\cfg\autoexec_ns_server.cfg`, change the line from `ns_masterserver_hostname "https://northstar.tf"` to `ns_masterserver_hostname "http://127.0.0.1"`.

Your game will now talk to your local server instead of the offical master server.

### Automated

TBD

## TODO

Things that are missing:

- Bad word filter
- Take ip address from header (proxy support)
- CORS headers
- Server mod pdiffs (mod specific player data)
  - Design is not finished
- Origin username verification
- Some account data lookup endpoints (API structure is questionable)
- Rate limiting (do in proxy?)
- Caching various endpoints

## Future improvements

- Actual response codes
- Store servers in database
- Use distributed database for failover (postgresql?)
- Compress stored player data

With client changes:

- Only upload changed player data
- Use heartbeat instead of update_values again
- More error reporting
- Version API for breaking changes (ex. `/api/v2/...`)
- Server ping
  - Could be collected by master server, but requires regional instances
