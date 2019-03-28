# Pre-Interview Test

## Overview

This app is a REST API for the pre-interview test.

### Endpoints

 - `/` - Returns *Hello World!* 200 OK
 - `/metadata` - Returns a json description of the app
 - `/health` - returns 204 No Content

### Design Considerations

I went for a REST API because it fits the requirements well and is easy to consume and use. On a actual project, I would also consider gRPC if it is an internal API and if the environment/use-case fits.

For the language, I went for rust to explore what web api development is like in rust today. On a actual project, I would probably still consider using rust on a small scale, especially when there are opportunities to really make use of rust's type system, safety guarantees and/or performance.

### CI

Travis is setup to run tests and clippy (linter) on all PRs, cron, tags and pushes to `master` branch. Pushes to master and tags will trigger the job that builds the docker image and publishes it to [dockerhub](image).

## Getting started

### Requirements
 - git
 - [rustc/cargo](rustup) - when building locally/running tests
 - or Docker

### Run Locally

[Cargo](rustup) is required to build the app locally or run tests.

```bash
$ git clone git@github.com:sanisoclem/preint-test.git
$ cd preint-test
$ cargo build
$ cargo test # run tests
$ cargo run # run app
```

### Run in Docker

This repository uses [clux/muslrust](muslrust) to build the binaries against musl instead of glibc. This simplifies the setup of the build environment and lets us use `alpine` or `scratch` as base. This app creates a <5MB image.

```bash
$ git clone git@github.com:sanisoclem/preint-test.git
$ cd preint-test
$ docker run -v $PWD:/volume --rm -t clux/muslrust cargo build --release # builds the app in the current dir
$ docker build . -t app
$ docker run -p 8000:8000 -t app
```

### Configuration

By default, the app runs in `http://localhost:8000`. This can be changed by using environment variables:

```bash
export ROCKET_ADDRESS="0.0.0.0"
export ROCKET_PORT=8000
```

### Testing

There is almost no logic in this app so instead of unit tests, I opted for integration tests to test the route config. To run tests locally:

```bash
$ cargo test
```

## Risks / Issues

 - There is no throttling so someone can just flood you with requests.
 - No CORS header, so the API cannot be used by other sites (if intended to be used)
 - No metrics.
 - [Rocket](rocket) is pre `1.0`, so API is still unstable and there are some missing features (like OpenAPI, async, [TLS](TLS) and logging is a bit [lacking](logging)). In a real production api, I would probably put `nginx` or an API Gateway in front.
 - Rocket requires nightly rust and that comes with a whole new set of caveats.
 - No client code generation, http documentation.
 - My experience in these technologies is limited so there may be best practices that I have not considered.

[rocket]: http://rocket.rs
[logging]: https://github.com/SergioBenitez/Rocket/issues/21
[TLS]: https://rocket.rs/v0.4/guide/configuration/#configuring-tls
[rustup]: https://rustup.rs/
[muslrust]: https://github.com/clux/muslrust/
[image]: https://hub.docker.com/r/potatoengineering/rustaroo-api