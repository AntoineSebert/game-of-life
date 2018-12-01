# Local Hack Day 2018

![license badge](https://img.shields.io/badge/license-CC%20BY--NC--SA%20%204.0-lightgrey.svg)
![build](https://travis-ci.com/AntoineSebert/local_hack_day_2018.svg?branch=master)

## Table of contents

- :fallen_leaf: [Motivation](#motivation)
- 💐 [Technical choices]
- :ear_of_rice: [Getting started]
  - :hibiscus: [Prerequisites]
  - :cherry_blossom: [Installing]
- :sunflower: [Running the tests]
  - :tulip: [End to end tests]
  - :blossom: [Coding style tests]
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

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

```
git clone 
make ... ?
run dandelion
```

### Prerequisites

What things you need to install the software and how to install them.
rustc
cargo

```
Give examples
```

### Installing

A step by step series of examples that tell you how to get a development env running.

Say what the step will be

```
Give the example
```

And repeat

```
until finished
```

End with an example of getting some data out of the system or using it for a little demo.

## Running the tests

Explain how to run the automated tests for this system.

### End to end tests

Explain what these tests test and why.

```
cargo test
```

### Coding style tests

Explain what these tests test and why.

```
Give an example
```

## Built tool

* [cargo](https://doc.rust-lang.org/cargo/) - the official Rust build tool

## Documentation

...

## File hierarchy

--{root}				- ... directory.
  +--readme.txt                 - This file.
  +--documentation.html         - Shortcut to the web documentation page.
  +--license.txt                - GPL license text.
  +--demos/                     - Demo projects, one directory per platform.
  +--docs/                      - Documentation.
  |  +--common/                 - Documentation common build resources.
  |  +--hal/                    - Builders for HAL.
  |  +--nil/                    - Builders for NIL.
  |  +--rt/                     - Builders for RT.
  +--ext/                       - External libraries, not part of ChibiOS/RT.
  +--os/                        - ChibiOS components.
  |  +--common/                 - Shared OS modules.
  |  |  +--abstractions/        - API emulator wrappers.

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