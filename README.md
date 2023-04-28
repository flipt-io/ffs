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

## Building/Running

### Running Debug

With args:

`$ cargo run -- -l go`

### Building Binary

`$ cargo build`

## How it Works

It currently only supports parsing Go code to look for instances of the following method calls:

- `GetFlag`
- `Evaluate`

It accomplishes this by:

1. Using the [Rust tree-sitter bindings](https://github.com/tree-sitter/tree-sitter/tree/master/lib/binding_rust) and [Go tree-sitter grammar](https://github.com/tree-sitter/tree-sitter-go) to build a syntax tree of a set of Go files.
2. We then use a TreeSitter query to look for the nodes that we care about, that match the signature of our known `Evaluate/GetFlag` methods and extract the value of the `FlagKey` and optionally `NamespaceKey` fields.

The query rules are written as an S-Expression: [go.scm](./rules/go.scm)

3. It checks the values of the captured values, which should contain the matching `Key` or `FlagKey`, and `NamespaceKey` values.
4. Finally, it captures the filename and location of the code and outputs the results to STDOUT or optionally to a file.

```json
{"namespaceKey":"default","flagKey":"foo","location":{"file":"./examples/go/basic.go","startLine":14,"startColumn":64,"endLine":21,"endColumn":1}}
{"namespaceKey":"production","flagKey":"bar","location":{"file":"./examples/go/basic.go","startLine":23,"startColumn":63,"endLine":30,"endColumn":1}}
{"namespaceKey":"default","flagKey":"boz","location":{"file":"./examples/go/basic.go","startLine":32,"startColumn":63,"endLine":39,"endColumn":1}}
{"namespaceKey":"default","flagKey":"foo","location":{"file":"./examples/go/basic.go","startLine":41,"startColumn":60,"endLine":43,"endColumn":1}}
```
