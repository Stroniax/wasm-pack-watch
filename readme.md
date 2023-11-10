# wasm-pack-watch

This is a super-simple rust application designed to be run inside a
docker container in a development compose stack. The application
watches your rust source code and runs `wasm-pack build` when it
detects changes.

## Usage

Use the docker image in your compose file. Map the `/watch` volume to your rust
project directory, and map the `/build` volume to your desired target directory.

```yml
# docker-compose.yml
services:
  wasm:
    image: wasm-pack-watch
    volumes:
      - /path/to/project:/watch
      - /path/to/output:/build
```
