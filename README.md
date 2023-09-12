# Flipt Flag Search

![Release](https://img.shields.io/github/release/flipt-io/ffs.svg?style=flat)
![Status](https://img.shields.io/badge/status-expiremental-orange)

Find [Flipt](https://github.com/flipt-io/flipt) feature flags in your codebase

## Install

### Homebrew

```console
brew tap flipt-io/brew
brew install ffs
```

### Cargo

TODO

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
  -c, --context                Display lines of context around flag
  -h, --help                   Print help
  -V, --version                Print version
```

## Building/Running Locally

### Requirements

- Rust - [install](https://rustup.rs/)

### Running Debug

With args:

`$ cargo run -- -l go`

### Building Binary

`$ cargo build`

## Support Languages

- Go
- TypeScript

Other language support is planned, but not yet implemented:

- [ ] Python [#43](https://github.com/flipt-io/ffs/issues/43)
- [ ] Java [#44](https://github.com/flipt-io/ffs/issues/44)
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

- [`GetFlag`](https://www.flipt.io/docs/reference/flags/get-flag)
- [`Boolean`](https://www.flipt.io/docs/reference/evaluation/boolean-evaluation)
- [`Variant`](https://www.flipt.io/docs/reference/evaluation/variant-evaluation)

It accomplishes this by:

1. Using the [Rust tree-sitter bindings](https://github.com/tree-sitter/tree-sitter/tree/master/lib/binding_rust) and Language specific tree-sitter grammar [ex: Go](https://github.com/tree-sitter/tree-sitter-go) to build a syntax tree.
2. Then use a TreeSitter query to look for the nodes that we care about, that match the signature of our known `Evaluate/GetFlag/Boolean/Variant` methods and extract the value of the `FlagKey` and optionally `NamespaceKey` fields.

The query rules are written as an S-Expression: ie: [go.scm](./rules/go.scm)

### Reporting

The tool takes output of the parser step and reports the results to the CLI.

`ffs -l go`

```console
Found 6 results:

- Flag: [ Key: bar, Namespace: production ]
  File: ./examples/go/evaluation.go
  Line: [ Start: 31, End: 39 ]
  Column: [ Start: 11, End: 4 ]
```

Or as JSON: `ffs -l go -f json`

```json
[
  {
    "namespaceKey": "production",
    "key": "bar",
    "context": [
      "\t\tNamespaceKey: \"default\",",
      "\t\tFlagKey:      \"foo\",",
      "\t\tContext: map[string]string{",
      "\t\t\t\"bar\": \"boz\",",
      "\t\t},",
      "\t})",
      "\tif err != nil {",
      "\t\tpanic(err)",
      "\t}",
      "",
      "\t_, err = client.Flipt().Evaluate(context.TODO(), &flipt.EvaluationRequest{",
      "\t\tEntityId:     \"1\",",
      "\t\tNamespaceKey: \"production\",",
      "\t\tFlagKey:      \"bar\",",
      "\t\tContext: map[string]string{",
      "\t\t\t\"bar\": \"boz\",",
      "\t\t},",
      "\t})",
      "\tif err != nil {",
      "\t\tpanic(err)",
      "\t}",
      "",
      "\t_, err = client.Flipt().Evaluate(context.TODO(), &flipt.EvaluationRequest{",
      "\t\tEntityId: \"1\",",
      "\t\tFlagKey:  \"boz\",",
      "\t\tContext: map[string]string{",
      "\t\t\t\"bar\": \"boz\",",
      "\t\t},"
    ],
    "location": {
      "file": "./examples/go/basic.go",
      "startLine": 36,
      "startColumn": 11,
      "endLine": 43,
      "endColumn": 4
    }
  },
```

The JSON format is useful for integrating with other tools, and also provides the `context` code around the flag usage.

You can also get the context output in the text format by providing the `--context` flag:

`ffs -l go --context`

```console
- Flag: [ Key: boz, Namespace: default ]
  File: ./examples/go/basic.go
  Line: [ Start: 48, End: 54 ]
  Column: [ Start: 11, End: 4 ]

/```
  NamespaceKey: "production",
  FlagKey:      "bar",
  Context: map[string]string{
   "bar": "boz",
  },
 })
 if err != nil {
  panic(err)
 }

 _, err = client.Flipt().Evaluate(context.TODO(), &flipt.EvaluationRequest{
  EntityId: "1",
  FlagKey:  "boz",
  Context: map[string]string{
   "bar": "boz",
  },
 })
 if err != nil {
  panic(err)
 }

 flag, err := client.Flipt().GetFlag(context.TODO(), &flipt.GetFlagRequest{
  Key: "foo",
 })
 if err != nil {
  panic(err)
 }
/```
```
