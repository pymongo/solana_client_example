/*
go get github.com/gagliardetto/solana-go@v1.11.0
go get github.com/gagliardetto/solana-go/rpc@v1.11.0
go get github.com/gagliardetto/binary@v0.8.0
*/
package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"github.com/gagliardetto/solana-go"
	"github.com/gagliardetto/solana-go/rpc"
	"github.com/joho/godotenv"
	bin "github.com/gagliardetto/binary"
)

type GreetingHello struct {
	Counter uint32
}

func main() {
	log.SetFlags(log.Lmicroseconds | log.Lshortfile)
	err := godotenv.Load()
	if err != nil {
		log.Fatalln(err)
	}
	program_id_b58 := os.Getenv("greeting_hello")
	program_id, err := solana.PublicKeyFromBase58(program_id_b58)
	if err != nil {
		log.Fatalln(err)
	}

	payer, err := solana.PrivateKeyFromSolanaKeygenFile(fmt.Sprintf("%s/.config/solana/id.json", os.Getenv("HOME")))
	if err != nil {
		log.Fatalln(err)
	}


	key, err := solana.CreateWithSeed(payer.PublicKey(), "hello_rust", program_id)
	if err != nil {
		log.Fatalln(err)
	}

	client := rpc.New(rpc.DevNet.RPC)

	data, err := client.GetAccountInfo(context.Background(), key)
	if err != nil {
		log.Fatalln(err)
	}
	borshDec := bin.NewBorshDecoder(data.GetBinary())
	var counter GreetingHello
	err = borshDec.Decode(&counter)
	if err != nil {
		log.Fatalln(err)
	}
	log.Println(counter.Counter)
	// client.GetLatestBlockhash(context.Background(), rpc.CommitmentConfirmed)

	log.Println(program_id, payer)
}
