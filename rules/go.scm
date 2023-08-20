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
            (interpreted_string_literal) @namespace_value)
          (keyed_element
            (field_identifier) @flag_key (#match? @flag_key "^(Key|FlagKey)$")
            (interpreted_string_literal) @flag_value)
        )
      )
    )
  )
) @call

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
            (interpreted_string_literal) @flag_value)
        )
      )
    )
  )
) @call


