(call_expression
  function: (member_expression
    property: (property_identifier) @method (#match? @method "(getFlag|evaluate|boolean|variant)")
  )
  arguments: (arguments
      (object
        (pair
          key: (property_identifier) @namespace_key (#eq? @namespace_key "namespaceKey")
          value: (string) @namespace_value
        )
        (pair
          key: (property_identifier) @flag_key (#match? @flag_key "^(key|flagKey)$")
          value: (string) @flag_value
        )
      )
    )
) @call

;; This is the same as above, but with the namespace key omitted for matching optional namespace
(call_expression
  function: (member_expression
    property: (property_identifier) @method (#match? @method "(getFlag|evaluate|boolean|variant)")
  )
  arguments: (arguments
      (object
        (pair
          key: (property_identifier) @flag_key (#match? @flag_key "^(key|flagKey)$")
          value: (string) @flag_value
        )
      )
    )
) @call