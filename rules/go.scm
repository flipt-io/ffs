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
     )
  )
) @result