# monitaringu-rei

Simple tool to start a program and monitor the files it creates.

## Requirements

- [Meson](http://mesonbuild.com/)
- [Ninja](https://ninja-build.org/) (version 1.7 or newer)
- [Rust](https://rust-lang.org)

## Installation

To configure and build the project, execute:

```sh
mkdir _build
meson setup --buildtype=release . _build
meson compile -C _build
```

To install the tools to system directories, execute:

```sh
meson install -C _build
```

## Usage

To monitor the create of a test file every seconds in ``/tmp/foo``:

```sh
monitaringu-rei-core -D /tmp/foo -- sh -c 'while true; do touch /tmp/foo/test-$RANDOM; sleep 1; done'
```

## License

This project is primarily distributed under the terms of the MIT license.

See LICENSE-MIT for details.
