var (
	store  = &storeMock{}
	logger = zaptest.NewLogger(t)
	s      = &Server{
		logger: logger,
		store:  store,
	}
)

store.On("GetFlag", mock.Anything, mock.Anything, "foo").Return(enabledFlag, nil)

store.On("GetEvaluationRules", mock.Anything, mock.Anything, "foo").Return([]*storage.EvaluationRule{}, nil)

// this is a comment that mentions the flagKey 'foo' but should not be included in the output
resp, err := s.Evaluate(context.TODO(), &flipt.EvaluationRequest{
	EntityId: "1",
	FlagKey:  "foo",
	Context: map[string]string{
		"bar": "boz",
	},
})

resp, err = s.Evaluate(context.TODO(), &flipt.EvaluationRequest{
	EntityId: "1",
	FlagKey:  "bar",
	Context: map[string]string{
		"bar": "boz",
	},
})

resp, err = s.Evaluate(context.TODO(), &flipt.EvaluationRequest{
	EntityId: "1",
	FlagKey:  "boz",
	Context: map[string]string{
		"bar": "boz",
	},
})

flag, err := s.GetFlag(context.TODO(), &flipt.GetFlagRequest{
	Key: "foo",
})
