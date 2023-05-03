# Flipt Flag Search

## Requirements

- Rust - [install](https://rustup.rs/)

## Usage

```console
A CLI tool to find Flipt feature flags in code

Usage: ffs [OPTIONS] --language <LANGUAGE>

Options:
  -l, --language <LANGUAGE>  [possible values: go]
  -o, --output <OUTPUT>      Path to output file (default STDOUT)
  -d, --dir <DIR>            Path to directory to scan (default .)
  -h, --help                 Print help
  -V, --version              Print version
```

## Building/Running Locally

### Running Debug

With args:

`$ cargo run -- -l go`

### Building Binary

`$ cargo build`

## Limitations

- Currently only supports parsing Go code.

## How it Works

Currently, the CLI tool is split into two parts:

- Parsing
- Validation

### Parsing

The parsing step look for instances of the following method calls:

- `GetFlag`
- `Evaluate`

It accomplishes this by:

1. Using the [Rust tree-sitter bindings](https://github.com/tree-sitter/tree-sitter/tree/master/lib/binding_rust) and [Go tree-sitter grammar](https://github.com/tree-sitter/tree-sitter-go) to build a syntax tree of a set of Go files.
2. We then use a TreeSitter query to look for the nodes that we care about, that match the signature of our known `Evaluate/GetFlag` methods and extract the value of the `FlagKey` and optionally `NamespaceKey` fields.

The query rules are written as an S-Expression: [go.scm](./rules/go.scm)

3. It checks the values of the captured values, which should contain the matching `Key` or `FlagKey`, and `NamespaceKey` values.
4. Finally, it captures the filename and location of the code.

### Validation

The tool takes output of the parser step and validates the results against the Flipt API.

It does this by:

1. Fetching the list of all flags from the Flipt API for the given namespace. It defaults to trying to access Flipt at `http://localhost:8080` but can be configured with the `FLIPT_ENDPOINT` environment variable.
2. It then checks the results of the parser against the list of flags returned from the API.
3. For each flag in code that is not found in the API, it will write the output to `STDOUT` and optionally a file of where the missing flag was found in code.

```json
[
    {
        "namespaceKey": "default",
        "flagKey": "foo",
        "location": {
            "file": "./examples/go/basic.go",
            "startLine": 14,
            "startColumn": 64,
            "endLine": 21,
            "endColumn": 1
        }
    },
    {
        "namespaceKey": "production",
        "flagKey": "bar",
        "location": {
            "file": "./examples/go/basic.go",
            "startLine": 23,
            "startColumn": 63,
            "endLine": 30,
            "endColumn": 1
        }
    },
    {
        "namespaceKey": "default",
        "flagKey": "boz",
        "location": {
            "file": "./examples/go/basic.go",
            "startLine": 32,
            "startColumn": 63,
            "endLine": 39,
            "endColumn": 1
        }
    },
    {
        "namespaceKey": "default",
        "flagKey": "foo",
        "location": {
            "file": "./examples/go/basic.go",
            "startLine": 41,
            "startColumn": 60,
            "endLine": 43,
            "endColumn": 1
        }
    }
]
```

4. An exit code of `1` is returned if any flags are found in code that are not found in the API, otherwise `0` is returned.

## Releasing

### Cross Compiling

#### MacOS

1. For building the binary for Linux on MacOS, you'll need to install the `musl-cross` toolchain:

```console
brew install FiloSottile/musl-cross/musl-cross
rustup target add x86_64-unknown-linux-musl
```

2. Then you can build the binary for Linux:

```console
TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl
```
