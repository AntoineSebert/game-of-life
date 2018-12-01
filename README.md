# Local Hack Day 2018

![license badge](https://img.shields.io/badge/license-CC%20BY--NC--SA%20%204.0-lightgrey.svg)

------

## Table of contents

- :fallen_leaf: [Motivation](#motivation)
- 💐 [Technical choices]
- :ear_of_rice: [Getting started]
  - :hibiscus: [Prerequisites]
  - :cherry_blossom: [Installing]
- :sunflower: [Running the tests]
  - :tulip: [End to end tests]
  - :blossom: [Coding style tests]
- :herb: [Deployment]
- :maple_leaf: [Built tool]
- 🌲 [Documentation]
- 🌹 [File hierarchy]
- :seedling: [Contributing](#contributing)
- :cactus: [Versioning](#versioning)
- :leaves: [Authors](#authors)
- :four_leaf_clover: [License](#license)
- :deciduous_tree: [Acknowledgments](#acknowledgments)

------

## Motivation

This project aims to create a POSIX-compliant microkernel real-time operating system using the Rust language. It addresses a large area of design techniques and algorithms so as to reach three defined goals  :

- **deterministic** execution time of the programs (or subparts of them), allowing the processes to be managed accurately
- **correctness** of the time when a result is expected, to meet or miss the deadlines, where treatment marks the difference between soft and hard RTOS
- **predictability** of the deadline associated with a set of constraints to define an expected state of the system

------

## Technical choices

* configuration files : toml
* data exchange format : JSON
* documents format : Markdown
* Rust
* CLI (no GUI)

------

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See [deployment](#Deployment) for notes on how to deploy the project on a live system.

```
git clone https://github.com/AntoineSebert/dandelion
make ... ?
run dandelion
```

### Prerequisites
What things you need to install the software and how to install them.
Qemu, rustc

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

------

## Running the tests
Explain how to run the automated tests for this system.

### End to end tests
Explain what these tests test and why.

```
Give an example
```

### Coding style tests
Explain what these tests test and why.

```
Give an example
```

------

## Deployment

Add additional notes about how to deploy this on a live system.

------

## Built tool

* [...](http://www.example.com) - …

------

## Documentation

...

------

## File hierarchy

example
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
  |  |  |  +--cmsis_os/         - CMSIS OS emulation layer for RT.
  |  |  |  +--nasa_osal/        - NASA Operating System Abstraction Layer.
  |  |  +--ext/                 - Vendor files used by the OS.
  |  |  +--ports/               - RTOS ports usable by both RT and NIL.
  |  |  +--startup/             - Startup support.
  |  +--ex/                     - EX component.
  |  |  +--dox/                 - EX documentation resources.
  |  |  +--Bosch /              - EX complex drivers for Bosch devices.
  |  |  +--Micron/              - EX complex drivers for Micron devices.
  |  |  +--ST/                  - EX complex drivers for ST devices.
  |  +--hal/                    - HAL component.
  |  |  +--boards/              - HAL board support files.
  |  |  +--dox/                 - HAL documentation resources.
  |  |  +--include/             - HAL high level headers.
  |  |  +--lib/                 - HAL libraries.
  |  |  |  +--complex/          - HAL collection of complex drivers.
  |  |  |  |  +--mfs/           - HAL managed flash storage driver.
  |  |  |  |  +--serial_nor/    - HAL managed flash storage driver.
  |  |  |  +--fallback/         - HAL fall back software drivers.
  |  |  |  +--peripherals/      - HAL peripherals interfaces.
  |  |  |  +--streams/          - HAL streams.
  |  |  +--osal/                - HAL OSAL implementations.
  |  |  |  +--lib/              - HAL OSAL common modules.
  |  |  +--src/                 - HAL high level source.
  |  |  +--ports/               - HAL ports.
  |  |  +--templates/           - HAL driver template files.
  |  |     +--osal/             - HAL OSAL templates.
  |  +--lib/                    - RTOS modules usable by both RT and NIL.
  |  |  +--include/             - OSLIB high level headers.
  |  |  +--src/                 - OSLIB high level source.
  |  |  +--templates/           - OSLIB configuration template files.
  |  +--nil/                    - NIL RTOS component.
  |  |  +--dox/                 - NIL documentation resources.
  |  |  +--include/             - NIL high level headers.
  |  |  +--src/                 - NIL high level source.
  |  |  +--templates/           - NIL configuration template files.
  |  +--rt/                     - RT RTOS component.
  |  |  +--dox/                 - RT documentation resources.
  |  |  +--include/             - RT high level headers.
  |  |  +--src/                 - RT high level source.
  |  |  +--templates/           - RT configuration template files.
  |  +--various/                - Various portable support files.
  +--test/                      - Kernel test suite source code.
  |  +--lib/                    - Portable test engine.
  |  +--hal/                    - HAL test suites.
  |  |  +--testbuild/           - HAL build test and MISRA check.
  |  +--nil/                    - NIL test suites.
  |  |  +--testbuild/           - NIL build test and MISRA check.
  |  +--rt/                     - RT test suites.
  |  |  +--testbuild/           - RT build test and MISRA check.
  |  |  +--coverage/            - RT code coverage project.
  +--testex/                    - EX integration test demos.
  +--testhal/                   - HAL integration test demos.

------

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

------

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 

------

## Authors
* [**Antoine/Anthony Sébert**](https://github.com/AntoineSebert) - *Design*
	* I do not declare any conflict of interest. Besides trying to increase humanity's knowledge, this project is necessary for me to complete my cursus at The Robert Gordon University.
* [**Antoine/Anthony Sébert's clone**](https://github.com/AntoineSebert) - *Implementation*
* [**Antoine/Anthony Sébert's evil twin**](https://github.com/AntoineSebert) - *Bug creation*
* [**Antoine/Anthony Sébert's future self**](https://github.com/AntoineSebert) - *Testing*
* [**Antoine/Anthony Sébert from a parallel dimension**](https://github.com/AntoineSebert) - *Documentation*

------

## License
This project is licensed under the CC BY-NC-SA License - see the [LICENSE.md](LICENSE.md) file for details.

-------

## Acknowledgments
* My supervisor, [Petrovski](https://orcid.org/0000-0002-0987-2791)
* M. Bartlett
* N. Wiratunga
* My friends for being here
* My family