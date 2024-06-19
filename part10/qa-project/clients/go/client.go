package main

import (
	"context"
	"log"
	"os"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"qa-project/clients/go/pb"
)

var (
	grpcAddress     = "127.0.0.1:50051"
// 	grpcAddress     = "192.168.1.4:50051"
	defaultUserName = "daheige"
)

/**
% go run client.go daheige

*/

func main() {
	// Set up a connection to the server.
	// please note the following settings
	// Deprecated: use WithTransportCredentials and insecure.NewCredentials()
	// instead. Will be supported throughout 1.x.
	// conn, err := grpc.Dial(address, grpc.WithInsecure())
	// so use grpc.WithTransportCredentials(insecure.NewCredentials()) as default grpc.DialOption
	conn, err := grpc.Dial(grpcAddress, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}

	defer conn.Close()

	c := pb.NewQAServiceClient(conn)

	// Contact the server and print out its response.
	name := defaultUserName
	if len(os.Args) > 1 {
		name = os.Args[1]
	}

	ctx := context.Background()
	res, err := c.UserLogin(ctx, &pb.UserLoginRequest{
		Username: name,
		Password: "abc",
	})

	if err != nil {
		log.Fatalf("could not greet: %v", err)
	}

	log.Printf("reply token:%s", res.Token)
}
