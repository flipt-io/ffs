package main

import (
	"context"
	"fmt"

	flipt "go.flipt.io/flipt/rpc/flipt"
	sdk "go.flipt.io/flipt/sdk/go"
	sdkgrpc "go.flipt.io/flipt/sdk/go/grpc"
	grpc "google.golang.org/grpc"
)

func main() {
	conn, err := grpc.Dial("localhost:9090")
	if err != nil {
		panic(err)
	}

	transport := sdkgrpc.NewTransport(conn)

	client := sdk.New(transport)

	// this is a comment that mentions the FlagKey 'foo' but should not be included in the output
	_, err = client.Flipt().Evaluate(context.TODO(), &flipt.EvaluationRequest{
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

	_, err = client.Flipt().Evaluate(context.TODO(), &flipt.EvaluationRequest{
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

	_, err = client.Flipt().Evaluate(context.TODO(), &flipt.EvaluationRequest{
		EntityId:     "1",
		NamespaceKey: "default",
		FlagKey:      "boz",
		Context: map[string]string{
			"bar": "boz",
		},
	})
	if err != nil {
		panic(err)
	}

	flag, err := client.Flipt().GetFlag(context.TODO(), &flipt.GetFlagRequest{
		Key: "foo",
	})
	if err != nil {
		panic(err)
	}

	fmt.Printf("flag: %v\n", flag)
}
