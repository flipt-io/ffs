(call_expression
  function: (member_expression
    property: (property_identifier) @method (#match? @method "(boolean|variant)")
  )
  arguments: (arguments
      (object
        (pair
          key: (property_identifier) @namespace_key (#eq? @namespace_key "namespaceKey")
          value: (_) @namespace_value
        )
        (pair
          key: (property_identifier) @flag_key (#eq? @flag_key "flagKey")
          value: (string) @flag_value
        )
      )
    ) ? @args
) @call

(call_expression
  function: (member_expression
    object: (member_expression
      property: (property_identifier) @property (#eq? @property "flags")
    )
    property: (property_identifier) @method (#eq? @method "get")
  )
  arguments: (arguments
      (object
        (pair
          key: (property_identifier) @namespace_key (#eq? @namespace_key "namespaceKey")
          value: (_) @namespace_value
        )
        (pair
          key: (property_identifier) @flag_key (#eq? @flag_key "key")
          value: (string) @flag_value
        )
      )
    ) ? @args
) @call
