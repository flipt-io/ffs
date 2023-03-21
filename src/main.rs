use std::fs;

use tree_sitter::{Parser, Query, QueryCursor};

fn main() {
    let code = r#"
	var (
		store  = &storeMock{}
		logger = zaptest.NewLogger(t)
		s      = &Server{
			logger: logger,
			store:  store,
		})

	store.On("GetFlag", mock.Anything, mock.Anything, "foo").Return(enabledFlag, nil)

	store.On("GetEvaluationRules", mock.Anything, mock.Anything, "foo").Return([]*storage.EvaluationRule{}, nil)

	resp, err := s.Evaluate(context.TODO(), &flipt.EvaluationRequest{
		EntityId: "1",
		FlagKey:  "foo",
		Context: map[string]string{
			"bar": "boz",
		},
	})

	require.NoError(t, err)
	assert.False(t, resp.Match)
	assert.Equal(t, flipt.EvaluationReason_UNKNOWN_EVALUATION_REASON, resp.Reason)
    "#;
    let mut parser = Parser::new();
    let go_lang = tree_sitter_go::language();
    parser
        .set_language(go_lang)
        .expect("Error loading Go grammar");

    let parsed = parser.parse(code, None).unwrap();

    let rules = fs::read_to_string("./rules/go.txt").expect("Unable to read file");

    let query = Query::new(go_lang, &rules).unwrap();

    let mut query_cursor = QueryCursor::new();
    let all_matches = query_cursor.matches(&query, parsed.root_node(), code.as_bytes());
    let flag_key_idx = query.capture_index_for_name("v").unwrap();
    for each_match in all_matches {
        // iterate over all captures called "raise"
        // ignore captures such as "fn-name"
        for capture in each_match
            .captures
            .iter()
            .filter(|c| c.index == flag_key_idx)
        {
            let range = capture.node.range();
            let text = &code[range.start_byte..range.end_byte];
            let line = range.start_point.row;
            let col = range.start_point.column;
            println!("[Line: {}, Col: {}] Found flagKey: `{}`", line, col, text);
        }
    }
}
