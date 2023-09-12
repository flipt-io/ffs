(call_expression
  function: (selector_expression
    field: (field_identifier) @method (#match? @method "(GetFlag|Evaluate|Boolean|Variant)"))
  arguments: (argument_list
    (unary_expression
      operand: (composite_literal
        type: (qualified_type
          name: (type_identifier) @type_name (#match? @type_name "(GetFlagRequest|EvaluationRequest)"))
        body: (literal_value
          (keyed_element
            (field_identifier) @namespace_key (#eq? @namespace_key "NamespaceKey")
            (_) @namespace_value)
          (keyed_element
            (field_identifier) @flag_key (#match? @flag_key "^(Key|FlagKey)$")
            (_) @flag_value)
        )
      )
    )
  ) ? @args
) @call

;; This is the same as above, but with the namespace key omitted for matching optional namespace
(call_expression
  function: (selector_expression
    field: (field_identifier) @method (#match? @method "(GetFlag|Evaluate|Boolean|Variant)"))
  arguments: (argument_list
    (unary_expression
      operand: (composite_literal
        type: (qualified_type
          name: (type_identifier) @type_name (#match? @type_name "(GetFlagRequest|EvaluationRequest)"))
        body: (literal_value
          (keyed_element
            (field_identifier) @flag_key (#match? @flag_key "^(Key|FlagKey)$")
            (_) @flag_value)
        )
      )
    )
  ) ? @args
) @call
