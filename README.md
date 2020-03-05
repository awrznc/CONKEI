# CONKEI

## Dev

```bash
# start to the development environment
$ docker-compose up -d

# login to the development environment
$ winpty docker-compose exec builder bash --login

# build
$ cargo build
```

## Build

```bash
$ docker-compose run builder ../.cargo/bin/cargo build --release --target x86_64-pc-windows-gnu
```
