(call_expression
  function: (_ (field_identifier) @_name (#match? @_name "(GetFlag|Evaluate)"))
  arguments: (_ 
    (unary_expression
      operand: (_
        body: (_ 
          (keyed_element
            (field_identifier)? @_namespaceKey (#match? @_namespaceKey "NamespaceKey")
            (interpreted_string_literal) @namespaceValue
          )
          (keyed_element
            (field_identifier) @_flagKey (#match? @_flagKey "(Key|FlagKey)")
            (interpreted_string_literal) @flagValue
          )
        ) @arg
      ) 
    ) 
  )
) @call
