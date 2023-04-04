# Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

```sh
git clone https://github.com/AntoineSebert/game-of-life.git && cd game-of-life
cargo build
cargo run -- -x 6 -y 6 --generations 10 --coordinates 1:1,2:2,1:2,2:1,3:3,3:4,4:3,4:4
```

The help is accessible using

# Running the tests

You can run the tests using:

```sh
cargo test
```

# Examples

With random population:

![](render1680549023156.gif)

With specified population:

```sh
cargo run -- -x 4 -y 4 --generations 2 --coordinates 1:1,2:2,1:2,2:1
```

```
⚫⚫⚫⚫
⚫⚪⚪⚫
⚫⚪⚪⚫
⚫⚫⚫⚫
```

```sh
cargo run -- -x 6 -y 6 --generations 10 --coordinates 1:1,2:2,1:2,2:1,3:3,3:4,4:3,4:4
```

```
⚫⚫⚫⚫⚫⚫
⚫⚪⚪⚫⚫⚫
⚫⚪⚪⚫⚫⚫
⚫⚫⚫⚪⚪⚫
⚫⚫⚫⚪⚪⚫
⚫⚫⚫⚫⚫⚫
```