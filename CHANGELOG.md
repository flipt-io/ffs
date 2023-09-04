# Changelog

## [0.0.16](https://github.com/flipt-io/ffs/compare/v0.0.15...v0.0.16) (2023-09-04)


### Features

* Build macos ([#39](https://github.com/flipt-io/ffs/issues/39)) ([8d06d08](https://github.com/flipt-io/ffs/commit/8d06d08671dea01f1a3d5e1c61814597f826ca1b))

## [0.0.15](https://github.com/flipt-io/ffs/compare/v0.0.14...v0.0.15) (2023-08-21)


### Features

* add human readable output; ability to select format ([#33](https://github.com/flipt-io/ffs/issues/33)) ([a91f488](https://github.com/flipt-io/ffs/commit/a91f488539b08cf263bda26ad5e8b98d62ce16c7))
* move ffs to default scan mode ([#35](https://github.com/flipt-io/ffs/issues/35)) ([eee9432](https://github.com/flipt-io/ffs/commit/eee94327d3ae017d2dd2b9e1f3a3003a0b4abb6d))
* support Eval v2 ([#37](https://github.com/flipt-io/ffs/issues/37)) ([664b5b0](https://github.com/flipt-io/ffs/commit/664b5b0436dcfc2b9b0ea8600af5e7022bd4d283))

## [0.0.14](https://github.com/flipt-io/ffs/compare/v0.0.13...v0.0.14) (2023-05-04)


### Features

* allow overriding of exit code in case missing flags were found ([#31](https://github.com/flipt-io/ffs/issues/31)) ([c19f76e](https://github.com/flipt-io/ffs/commit/c19f76e4929ed2ced15ad49c1e30a50b419e36a3))

## [0.0.13](https://github.com/flipt-io/ffs/compare/v0.0.12...v0.0.13) (2023-05-03)


### Features

* reformat errors payload ([#28](https://github.com/flipt-io/ffs/issues/28)) ([95e1dda](https://github.com/flipt-io/ffs/commit/95e1dda54c673243017a682ef2d9533863ef8ee5))

## [0.0.12](https://github.com/flipt-io/ffs/compare/v0.0.11...v0.0.12) (2023-05-03)


### Features

* fetch each scanned flag by namespace and key ([82401ed](https://github.com/flipt-io/ffs/commit/82401ed80893f42a092c6af2f3d7a2cbf58c7b5c))

## [0.0.11](https://github.com/flipt-io/ffs/compare/v0.0.10...v0.0.11) (2023-05-03)


### Features

* update JSON output ([#22](https://github.com/flipt-io/ffs/issues/22)) ([106e669](https://github.com/flipt-io/ffs/commit/106e6695074f7bcbba0cf21625682cbff5d515c4))

## [0.0.10](https://github.com/flipt-io/ffs/compare/v0.0.9...v0.0.10) (2023-05-03)


### Bug Fixes

* **rules/go:** ensure exact match on field flag key ([2c831df](https://github.com/flipt-io/ffs/commit/2c831df35d4004d1969bf181da6fce659cc3fe80))

## [0.0.9](https://github.com/flipt-io/ffs/compare/v0.0.8...v0.0.9) (2023-05-02)


### Bug Fixes

* upgrade to use flipt-rust with rustls ([dbc6046](https://github.com/flipt-io/ffs/commit/dbc6046f8c98e6fccb5a7847cb3b66dc3817b8c3))

## [0.0.8](https://github.com/flipt-io/ffs/compare/v0.0.7...v0.0.8) (2023-05-02)


### Miscellaneous Chores

* release 0.0.8 ([4887af3](https://github.com/flipt-io/ffs/commit/4887af34116e96ac35e6651e05457782d0af3e26))

## [0.0.7](https://github.com/flipt-io/ffs/compare/v0.0.7...v0.0.7) (2023-05-02)


### Miscellaneous Chores

* release 0.0.7 ([d6ab590](https://github.com/flipt-io/ffs/commit/d6ab5903a21c53961817e28570939e46ef02102f))

## [0.0.7](https://github.com/flipt-io/ffs/compare/v0.0.6...v0.0.7) (2023-05-02)


### Bug Fixes

* update flipt-rust to support tls ([9844f2f](https://github.com/flipt-io/ffs/commit/9844f2fad00b49e147c01fb59e96c53f3a4c6523))

## [0.0.6](https://github.com/flipt-io/ffs/compare/v0.0.5...v0.0.6) (2023-05-02)


### Features

* bundle rules with binary ([#15](https://github.com/flipt-io/ffs/issues/15)) ([af4b6ca](https://github.com/flipt-io/ffs/commit/af4b6cac27c79c6ca2682d83e3691785c8a255c4))


### Bug Fixes

* off by one error for lines ([98f2ee6](https://github.com/flipt-io/ffs/commit/98f2ee6a53412ce2003351648317359009693927))

## [0.0.5](https://github.com/flipt-io/ffs/compare/v0.0.4...v0.0.5) (2023-04-30)


### Miscellaneous Chores

* release 0.0.5 ([76ac3a7](https://github.com/flipt-io/ffs/commit/76ac3a7161b1d98645fd7922c4f4b7408b30786e))

## [0.0.4](https://github.com/flipt-io/ffs/compare/v0.0.3...v0.0.4) (2023-04-30)


### Miscellaneous Chores

* release 0.0.4 ([b6bb407](https://github.com/flipt-io/ffs/commit/b6bb4077500b91c0e5a262e17869242ef4905f89))

## [0.0.3](https://github.com/flipt-io/ffs/compare/v0.0.2...v0.0.3) (2023-04-30)


### Features

* abstract out language and file, introduce clap ([587a69f](https://github.com/flipt-io/ffs/commit/587a69fc386d3e3bfaba031e4f4d6c571b7daf37))
* add ability to output to stdout or file with JSON ([842f129](https://github.com/flipt-io/ffs/commit/842f12915c14a329decbd41da6fd3d2e119ebe9d))
* basic querying of Flipt server with flag keys ([#3](https://github.com/flipt-io/ffs/issues/3)) ([6a1317b](https://github.com/flipt-io/ffs/commit/6a1317bb5a1c26d930b766e46777a2fe1764e0be))
* initial commit ([eea7409](https://github.com/flipt-io/ffs/commit/eea74096bd1b0e77d74ed20b07f90c8b0a468b78))
* move code to examples folder; setup_panic ([93a9fd7](https://github.com/flipt-io/ffs/commit/93a9fd7e7817a8b32e7f95cf943a57619a6543b1))
* Namespaces ([#4](https://github.com/flipt-io/ffs/issues/4)) ([a10d3cb](https://github.com/flipt-io/ffs/commit/a10d3cb893be703f3a844e8b6494631b6f93dfb4))
* Query for missing flags ([#6](https://github.com/flipt-io/ffs/issues/6)) ([1361c82](https://github.com/flipt-io/ffs/commit/1361c82e4be0ddc54b6a0e01e8dcc67f01fec43d))
* read from env var or default for flipt config ([45925c4](https://github.com/flipt-io/ffs/commit/45925c43ab264c575fbddcd27056ea162608abdf))
* wip rust impl ([3317109](https://github.com/flipt-io/ffs/commit/33171095fc554e449e2f5b6874fa1ee72eec1665))

## [0.0.2](https://github.com/flipt-io/ffs/compare/ffs-v0.0.1...ffs-v0.0.2) (2023-04-30)

### Features

- Add ability to output to stdout or file with JSON ([842f129](https://github.com/flipt-io/ffs/commit/842f12915c14a329decbd41da6fd3d2e119ebe9d))
- Basic querying of Flipt server with flag keys ([#3](https://github.com/flipt-io/ffs/issues/3)) ([6a1317b](https://github.com/flipt-io/ffs/commit/6a1317bb5a1c26d930b766e46777a2fe1764e0be))
- Support Namespaces ([#4](https://github.com/flipt-io/ffs/issues/4)) ([a10d3cb](https://github.com/flipt-io/ffs/commit/a10d3cb893be703f3a844e8b6494631b6f93dfb4))
- Query for missing flags ([#6](https://github.com/flipt-io/ffs/issues/6)) ([1361c82](https://github.com/flipt-io/ffs/commit/1361c82e4be0ddc54b6a0e01e8dcc67f01fec43d))
- Read from env var or default for flipt config ([45925c4](https://github.com/flipt-io/ffs/commit/45925c43ab264c575fbddcd27056ea162608abdf))
