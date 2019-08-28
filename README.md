
# Kreta proxy
![](https://api.travis-ci.com/hazizz/kreta-proxy.svg?branch=master)

This is a proxy for [E-Kréta](https://ekreta.hu/) written in rust

## Getting Started

### Prerequisites

To get started you need a functioning [rust-lang environment](https://www.rust-lang.org/tools/install) on your computer
```
cargo --version
cargo 1.37.0 (9edd08916 2019-08-02)
```

### Installing

Download the code
```
git clone https://github.com/hazizz/kreta-proxy.git
```

Run with cargo
```
cd kreta-proxy
cargo run
```

## Running the tests

The unit tests require you to give a username, password and school url in environmental variables.

```
export USERNAME=username
export PASSWORD=password
export SCHOOL_URL=url
cargo test
```

## Usage

The proxy has non protected end points that can be accessed with HTTP.
Documentation can be found on [postman](https://documenter.getpostman.com/view/5139955/SVfQQoeM).

## Built With

* [Actix](https://actix.rs/)
* [Reqwest](https://docs.rs/reqwest/)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details