# Changelog

## [0.7.0](https://github.com/jearle10/cscart-rs/compare/v0.6.0...v0.7.0) - 2024-06-03

### Added
- Remaining types added

### Other
- udpate readme
- Cargo fix
- Update README.md
- Internal refactoring

## [0.6.0](https://github.com/jearle10/cscart-rs/compare/v0.5.6...v0.6.0) - 2024-05-24

### Other
- More types ([#32](https://github.com/jearle10/cscart-rs/pull/32))

## [0.5.6](https://github.com/jearle10/cscart-rs/compare/v0.5.5...v0.5.6) - 2024-05-19

### Added
- Add OrderDetails type

### Fixed
- update serde_utils deserializers

### Other
- Remove .vscode

## [0.5.5](https://github.com/jearle10/cscart-rs/compare/v0.5.4...v0.5.5) - 2024-05-15

### Fixed
- circular ci

## [0.5.4](https://github.com/jearle10/cscart-rs/compare/v0.5.3...v0.5.4) - 2024-05-15

### Added
- Add product type

### Fixed
- merge main
- merge main

### Other
- get and create offer methods

## [0.5.3](https://github.com/jearle10/cscart-rs/compare/v0.5.2...v0.5.3) - 2024-03-06

### Other
- Update cicd
- release-plz
- release-plz
- release-plz

## [0.5.0](https://github.com/jearle10/cscart-rs/compare/v0.4.2...v0.5.0) (2023-06-18)


### Features

* All entities added - some still untested ([6e03a06](https://github.com/jearle10/cscart-rs/commit/6e03a064e6547bce4c09288ecf1a415fe885ee67))
* Order service added ([4a080d6](https://github.com/jearle10/cscart-rs/commit/4a080d6c67f297fa76cf0cab19e5dcd21e8fae6c))

## [0.4.2](https://github.com/jearle10/cscart-rs/compare/v0.4.1...v0.4.2) (2023-06-15)


### Bug Fixes

* Refactor create method to use anyhow ([7ff5bbb](https://github.com/jearle10/cscart-rs/commit/7ff5bbbfce6eb3210d5791fe0a4dfa8cf1192dbf))

## [0.4.1](https://github.com/jearle10/cscart-rs/compare/v0.4.0...v0.4.1) (2023-02-25)


### Bug Fixes

* Readme emojis removed - crates.io didnt render correctly ([5adc495](https://github.com/jearle10/cscart-rs/commit/5adc4959b2030e36e889112a148b65434a2f84c3))

## [0.4.0](https://github.com/jearle10/cscart-rs/compare/v0.3.0...v0.4.0) (2023-02-17)


### Features

* Added payment method entity to client ([1e8ea14](https://github.com/jearle10/cscart-rs/commit/1e8ea14c0efa7439f83e453844733eddc2de544a))

## [0.3.0](https://github.com/jearle10/cscart-rs/compare/v0.2.0...v0.3.0) (2023-02-17)


### Features

* Add create method to service and refactor ([12ac5ac](https://github.com/jearle10/cscart-rs/commit/12ac5ac4bac324d1f9a5ae0d9e1caf3ae8551133))
* Added 'create' to all supported entities and added user entitiy to api client ([a7302db](https://github.com/jearle10/cscart-rs/commit/a7302db52fbb340f62cc49a0db5fa4eb6ced1d69))
* Added cart entity ([86cfa8a](https://github.com/jearle10/cscart-rs/commit/86cfa8a28b965a4c1aeb4fa8c2f5a61b08fc8087))
* Added product entity to client ([f25876a](https://github.com/jearle10/cscart-rs/commit/f25876a9e4712312f0f84ce7c269aca679e893a6))
* Added vendor entity to client ([e38ce11](https://github.com/jearle10/cscart-rs/commit/e38ce1192a17df45059939e1f00bede8169817c8))

## [0.2.0](https://github.com/jearle10/cscart-rs/compare/v0.1.0...v0.2.0) (2023-02-16)


### Features

* Added category method ([26b143a](https://github.com/jearle10/cscart-rs/commit/26b143ac28bea96d4aecb7d7ca235db34e471264))
* Handler struct and HandlerBuilder for abstracting CRUD api calls ([5e5bfba](https://github.com/jearle10/cscart-rs/commit/5e5bfba5c8e9c159c1ea50a50823bafedfb4ac01))
* Implementations for put, post , delete added ([57c42e3](https://github.com/jearle10/cscart-rs/commit/57c42e370f799bb0ac375d4c5100a4210d558fc3))
* Request builder for making http requests using basic auth ([109d722](https://github.com/jearle10/cscart-rs/commit/109d722a0c45e87e5d43470c7b6070802535013f))


### Bug Fixes

* Added missing json payload for put and post reqwest methods ([bc2399f](https://github.com/jearle10/cscart-rs/commit/bc2399f25590b7e42840917a0539f0cc70c9945b))
* **docs:** Added top level crate documentation to lib.rs ([8af9d8a](https://github.com/jearle10/cscart-rs/commit/8af9d8a4512c752a323d59159de13a4ba9a21634))
* Return Error trait object from all request client methods ([67758c3](https://github.com/jearle10/cscart-rs/commit/67758c38ec9351e42dce34dc32aeebb2cdfe2275))
* Update readme with how to create api client ([bd6c5a6](https://github.com/jearle10/cscart-rs/commit/bd6c5a630d0fe53012c9832a7856802c867221d2))

## 0.1.0 (2023-02-12)


### Features

* Create an api client using Client struct ([39dded0](https://github.com/jearle10/cscart-rs/commit/39dded0db2c017357b64978da4fc074ebb3e4465))


### Bug Fixes

* Added test to check for cscart api key ([4e8c16e](https://github.com/jearle10/cscart-rs/commit/4e8c16e8e730275964e02a54984ba96b1ca1a061))
* Updated api credentials test to check for username and host. Added dotenv file for configuring locally ([3e0c6b0](https://github.com/jearle10/cscart-rs/commit/3e0c6b064e8040936b8e5c7fa56d4cba49cb2918))
* Updated workflow to include username and host api secrets ([511dcb9](https://github.com/jearle10/cscart-rs/commit/511dcb9e59e9bf5778539649ece90f73f0fe4dc4))
