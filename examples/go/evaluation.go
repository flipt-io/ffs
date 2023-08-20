package main

import (
	"context"

	evaluation "go.flipt.io/flipt/rpc/flipt/evaluation"
	sdk "go.flipt.io/flipt/sdk/go"
	sdkhttp "go.flipt.io/flipt/sdk/go/http"
)

func main() {
	transport := sdkhttp.NewTransport("http://localhost:8080")

	client := sdk.New(transport)

	// this is a comment that mentions the FlagKey 'foo' but should not be included in the output
	_, err := client.Evaluation().Boolean(context.TODO(), &evaluation.EvaluationRequest{
		RequestId:    "123",
		EntityId:     "1",
		NamespaceKey: "default",
		FlagKey:      "foo",
		Context: map[string]string{
			"bar": "boz",
		},
	})
	if err != nil {
		panic(err)
	}

	_, err = client.Evaluation().Variant(context.TODO(), &evaluation.EvaluationRequest{
		RequestId:    "123",
		EntityId:     "1",
		NamespaceKey: "production",
		FlagKey:      "bar",
		Context: map[string]string{
			"bar": "boz",
		},
	})
	if err != nil {
		panic(err)
	}

	// defining a slice of requests outside of the function call
	reqs := []*evaluation.EvaluationRequest{
		{
			EntityId:     "1",
			NamespaceKey: "production",
			FlagKey:      "bar",
			Context: map[string]string{
				"bar": "boz",
			},
		},
		{
			EntityId: "1",
			FlagKey:  "bar",
			Context: map[string]string{
				"bar": "boz",
			},
		},
	}

	_, err = client.Evaluation().Batch(context.TODO(), &evaluation.BatchEvaluationRequest{
		RequestId: "123",
		Requests:  reqs,
	})
	// defining a slice of requests outside of the function call
	[]*evaluation.EvaluationRequest{
		{
			EntityId:     "1",
			NamespaceKey: "production",
			FlagKey:      "bar",
			Context: map[string]string{
				"bar": "boz",
			},
		},
		{
			EntityId: "1",
			FlagKey:  "bar",
			Context: map[string]string{
				"bar": "boz",
			},
		},
	}

	// defining a slice of requests inside of the function call
	_, err = client.Evaluation().Batch(context.TODO(), &evaluation.BatchEvaluationRequest{
		RequestId: "123",
		Requests: []*evaluation.EvaluationRequest{
			{
				EntityId:     "1",
				NamespaceKey: "production",
				FlagKey:      "bar",
				Context: map[string]string{
					"bar": "boz",
				},
			},
			{
				EntityId: "1",
				FlagKey:  "bar",
				Context: map[string]string{
					"bar": "boz",
				},
			},
		},
	})
}
