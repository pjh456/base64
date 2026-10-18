# Base64 CLI

A Rust-based cli for Base64 string encoding and decoding.

## Feature

- Non-streaming IO, with clean output while metting failure.
- 9x Slower than coreutils/base64 **now**
- (todo) SIMD text processing

## Quick Start

just same as coreutils/base64:

```sh
base64 # encoding string from stdin
base64 -d # same as --decode, decoding string from stdin
base64 -d -i # same as --ignore-garbage, only works when -d, ignoring characters out of range
base64 -w n # same as --wrap n, setting width of output and spliting output into several lines with '\n', default n=76
```

## License

MIT. As it just a toy repo to help me learn rust and base64 :)
