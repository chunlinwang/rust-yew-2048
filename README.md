# 2048

## A Rust Tetris.


## Demo


## Features


## Manuals


## Environment Dev:

* Rust >= 1.45.2 
* conrod_core = 0.71.0

## Application learning materials:

* [The Rust Programming Language](https://doc.rust-lang.org/book/)

## Future (TODO LIST)


## Author
* [@Chunlin Wang](https://www.linkedin.com/in/chunlin-wang-b606b159/)

wasm-pack build --target web --out-name wasm --out-dir ./static/wasm
simple-http-server -p 8001

cargo +nightly install miniserve
miniserve ./static --index index.html