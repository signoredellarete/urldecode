# urlencode
Linux urldecode for command line, written in Rust.

## Release Compilation to reduce executable size
```
RUSTFLAGS='-C strip=symbols' cargo build --release
```

## How to compile on Alpine linux

apk add gcc
apk add cc
apk add build-base