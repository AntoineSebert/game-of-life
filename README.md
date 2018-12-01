# Local Hack Day 2018

![license badge](https://img.shields.io/badge/license-CC%20BY--NC--SA%20%204.0-lightgrey.svg)
![build](https://travis-ci.com/AntoineSebert/local_hack_day_2018.svg?branch=master)

## Table of contents

- :fallen_leaf: [Motivation](#motivation)
- 💐 [Technical choices]
- :hibiscus: [Prerequisites]
- :ear_of_rice: [Getting started]
- :sunflower: [Running the tests]
- :maple_leaf: [Built tool]
- 🌲 [Documentation]
- 🌹 [File hierarchy]
- :seedling: [Contributing](#contributing)
- :cactus: [Versioning](#versioning)
- :leaves: [Authors](#authors)
- :four_leaf_clover: [License](#license)

## Motivation

This project is an entry to the [Local Hack Day 2018](https://localhackday.mlh.io/). It aims to implement [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) inside Conway's Game of Life while giving zero fucks, or even a negative amount of fucks.

## Technical choices

* configuration files : toml
* documents format : Markdown
* Rust
* CLI (no GUI)

## Prerequisites

**Rust**: Available at https://www.rust-lang.org. Follow the instructions to install the Rust compiler and the **Cargo** package manager.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

```
git clone https://github.com/AntoineSebert/local_hack_day_2018.git
cargo build
cargo run
```

## Running the tests

Explain how to run the automated tests for this system.

```
cargo test
```

## Built tool

* [cargo](https://doc.rust-lang.org/cargo/) - the official Rust build tool

## Documentation

...

## File hierarchy

```
--{root}							- root directory
	+--.gitignore					- ignored files and repositories
	+--.travis.yml					- CI service configuration
	+--Cargo.lock					- internal build service configuration for project
	+--Cargo.toml					- custom build service configuration for project
	+--changelog.md					- versioning file
	+--CODE_OF_CONDUCT.md			- code of conduct for the project
	+--CODEOWNERS.md				- developer responsibles for parts of the project
	+--CONTRIBUTING.md				- contribution guidelines
	+--LICENSE.md					- CC BY-NC-SA 4.0 text
	+--PULL_REQUEST_TEMPLATE.md		- pull request template
	+--README.md					- this file
	+--.git/						- git configuration and files
	+--.github/						- Github specific files
	+--docs/						- Documentation
	+--resources/					- various resources
	+--src/							- source files
	+--target/						- build files
	+--tests/						- tests files
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 

## Authors
* [**Antoine/Anthony Sébert**](https://github.com/AntoineSebert) - *Design*
	* I do not declare any conflict of interest. Besides trying to increase humanity's knowledge, this project is necessary for me to complete my cursus at The Robert Gordon University.
* [**Antoine/Anthony Sébert's clone**](https://github.com/AntoineSebert) - *Implementation*
* [**Antoine/Anthony Sébert's evil twin**](https://github.com/AntoineSebert) - *Bug creation*
* [**Antoine/Anthony Sébert's future self**](https://github.com/AntoineSebert) - *Testing*
* [**Antoine/Anthony Sébert from a parallel dimension**](https://github.com/AntoineSebert) - *Documentation*

## License

This project is licensed under the CC BY-NC-SA License - see the [LICENSE.md](LICENSE.md) file for details.