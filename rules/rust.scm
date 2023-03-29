(call_expression
  function: (_
  	field: (_) @_fn (#eq? @_fn "get")
  )
  arguments: (arguments (_) @v)
) @call

(call_expression
  function: (_
  	field: (_) @_fn (#eq? @_fn "evaluate")
  )
  arguments: (arguments (_
  	value: (_
      body: (_
       (field_initializer 
         name: (_) @k (#eq? @k "flag_key")
         value: (_) @v
       )
     )
	  )
  ))
)
