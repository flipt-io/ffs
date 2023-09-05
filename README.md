# Flipt Flag Search

![Release](https://img.shields.io/github/release/flipt-io/ffs.svg?style=flat)

Find [Flipt](https://github.com/flipt-io/flipt) feature flags in your codebase

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

Currently only supports parsing Go code.

Other language support is planned, but not yet implemented:

- [ ] Python [#43](https://github.com/flipt-io/ffs/issues/43)
- [ ] Java [#44](https://github.com/flipt-io/ffs/issues/44)
- [ ] Javascript/Typescript [#42](https://github.com/flipt-io/ffs/issues/42)
- [ ] Rust [#45](https://github.com/flipt-io/ffs/issues/45)

## Contributing

### Conventional Commits

This project uses [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for commit messages. This allows us to automate the release process and generate a changelog.

### Adding a new language

1. Add a new language to the `SupportedLanguage` enum in [src/types/language.rs](./src/types/language.rs)
1. Add new `match` arms in [src/types/language.rs](./src/types/language.rs) to handle the new language (`From` and `Display` traits).
1. Add a new set of rules for the language in [rules](./rules). The rules are written in [S-Expression](https://en.wikipedia.org/wiki/S-expression) format and the syntax is documented in the [TreeSitter Query docs](https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries). Note: the filename must match the language name in the enum.
1. Add a new set of examples in [examples](./examples) for the new language.

## How it Works

Currently, the CLI tool is split into two parts:

- Parsing
- Reporting

### Parsing

The parsing step look for instances of Flipt evaluation and flag retrieval methods:

- `GetFlag`
- `Evaluate` (v1 evaluation)
- `Boolean` (v2 evaluation)
- `Variant` (v2 evaluation)

It accomplishes this by:

1. Using the [Rust tree-sitter bindings](https://github.com/tree-sitter/tree-sitter/tree/master/lib/binding_rust) and Language specific tree-sitter grammar [ex: Go](https://github.com/tree-sitter/tree-sitter-go) to build a syntax tree.
2. Then use a TreeSitter query to look for the nodes that we care about, that match the signature of our known `Evaluate/GetFlag/Boolean/Variant` methods and extract the value of the `FlagKey` and optionally `NamespaceKey` fields.

The query rules are written as an S-Expression: ie: [go.scm](./rules/go.scm)

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
