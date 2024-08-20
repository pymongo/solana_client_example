package main

import (
	"fmt"
	"log"
	"os"

	"github.com/gagliardetto/solana-go"
	"github.com/joho/godotenv"
)

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

	// fmt.Sprintf("%s/.config/solana/id.json", os.Getenv("HOME"))
	payer, err := solana.PrivateKeyFromSolanaKeygenFile(fmt.Sprintf("%s/.config/solana/id.json", os.Getenv("HOME")))
	if err != nil {
		log.Fatalln(err)
	}

	log.Println(program_id, payer)
}
