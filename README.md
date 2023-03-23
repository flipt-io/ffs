# Flipt Flag Search

## Requirements

- Rust - [install](https://rustup.rs/)

## Building/Running

### Running Debug

`$ cargo run .`

### Building Binary

`$ cargo build`

## How it Works

It currently only supports parsing Go code to look for instances of the following method calls:

- `GetFlag`
- `Evaluate`

It accomplishes this by:

1. Using the [Rust tree-sitter bindings](https://github.com/tree-sitter/tree-sitter/tree/master/lib/binding_rust) and [Go tree-sitter grammar](https://github.com/tree-sitter/tree-sitter-go) to build a syntax tree of a Go file.

IE: Given the following Go code:

```go
resp, err := s.Evaluate(context.TODO(), &flipt.EvaluationRequest{
	EntityId: "1",
	FlagKey:  "foo",
	Context: map[string]string{
		"bar": "boz",
	},
})
```

It builds a TreeSitter tree like:

```
source_file [0, 0] - [8, 0]
  short_var_declaration [0, 0] - [6, 2]
    left: expression_list [0, 0] - [0, 9]
      identifier [0, 0] - [0, 4]
      identifier [0, 6] - [0, 9]
    right: expression_list [0, 13] - [6, 2]
      call_expression [0, 13] - [6, 2]
        function: selector_expression [0, 13] - [0, 23]
          operand: identifier [0, 13] - [0, 14]
          field: field_identifier [0, 15] - [0, 23]
        arguments: argument_list [0, 23] - [6, 2]
          call_expression [0, 24] - [0, 38]
            function: selector_expression [0, 24] - [0, 36]
              operand: identifier [0, 24] - [0, 31]
              field: field_identifier [0, 32] - [0, 36]
            arguments: argument_list [0, 36] - [0, 38]
          unary_expression [0, 40] - [6, 1]
            operand: composite_literal [0, 41] - [6, 1]
              type: qualified_type [0, 41] - [0, 64]
                package: package_identifier [0, 41] - [0, 46]
                name: type_identifier [0, 47] - [0, 64]
                ...
```

2. We then use a TreeSitter query to look for the nodes that we care about, that match the signature of our known `Evaluate` method and extract the value of the `FlagKey` field.

The query rules are written as an S-Expression, and looks like:

```scheme
(call_expression
	function: (_) @_fn (#match? @_fn "(GetFlag|Evaluate)")
    arguments: (argument_list
     (unary_expression
       (composite_literal
         body: (_
           (keyed_element 
            (interpreted_string_literal) @v
            ) @k (#match? @k "(Key|FlagKey)")
         )         
       )
	)) 
) @result
```

3. It checks the value of the `@v` variable, which should contain the matching `Key` or `FlagKey` value.
4. Finally, it captures the `line` `column` and `filename` and outputs the results to STDOUT:

```
[Line: 15, Col: 11] Found flagKey: `"foo"`
[Line: 23, Col: 11] Found flagKey: `"bar"`
[Line: 31, Col: 11] Found flagKey: `"boz"`
[Line: 38, Col: 6] Found flagKey: `"foo"`
```
