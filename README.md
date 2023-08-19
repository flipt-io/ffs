# Flipt Flag Search

## Requirements

- Rust - [install](https://rustup.rs/)

## Usage

```console
A CLI tool to find Flipt feature flags in code

Usage: ffs [OPTIONS] --language <LANGUAGE>

Options:
  -l, --language <LANGUAGE>    [possible values: go]
  -o, --output <OUTPUT>        Path to output file [default: STDOUT]
  -f, --format <FORMAT>        [default: text] [possible values: json, text]
  -d, --dir <DIR>              Path to directory to scan [default: .]
  -n, --namespace <NAMESPACE>  Namespace to filter [default: '']
  -h, --help                   Print help
  -V, --version                Print version
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
- Reporting

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

### Reporting

The tool takes output of the parser step and reports the results to the CLI


```json
[
  {
    "message": "Found flag: [key: bar, namespace: production]",
    "flag": {
      "namespaceKey": "production",
      "key": "bar",
      "location": {
        "file": "./examples/go/basic.go",
        "startLine": 36,
        "startColumn": 11,
        "endLine": 43,
        "endColumn": 4
      }
    }
  }
]
```

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
