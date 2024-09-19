package main

import (
	"bytes"
	"encoding/base64"
	"encoding/binary"
	"fmt"
	"log"

	"github.com/mr-tron/base58"
)

func main() {
	// Base64-encoded token account data
	dataBase64 := "BpuIV/6rgYT7aH9jRhjANdrEOdwa6ztVmKDwAAAAAAGyNpDX0HWNHV2LiVDOx6m018ea6P+1xroNvWKhmDeTW0lAXwPOPAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQEAAADwHR8AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"

	// Decode the base64 string
	decodedData, err := base64.StdEncoding.DecodeString(dataBase64)
	if err != nil {
		log.Fatal("Error decoding base64: ", err)
	}

	// Create a byte reader
	reader := bytes.NewReader(decodedData)
	fmt.Println(len(decodedData))

	// Define the layout structure for the token account
	type TokenAccount struct {
		Mint                 [32]byte // 32 bytes
		Owner                [32]byte // 32 bytes
		Amount               uint64   // 8 bytes
		DelegateOption       uint32   // 4 bytes
		Delegate             [32]byte // 32 bytes
		State                uint8    // 1 byte
		IsNativeOption       uint64   // 8 bytes
		RentExemptReserve    uint64   // 8 bytes
		CloseAuthorityOption uint32   // 4 bytes
		CloseAuthority       [32]byte // 32 bytes
	}

	// Initialize the TokenAccount structure
	var tokenAccount TokenAccount

	// Unpack the binary data into the structure
	err = binary.Read(reader, binary.LittleEndian, &tokenAccount)
	if err != nil {
		log.Fatal("Error deserializing data: ", err)
	}

	ownerBase58 := base58.Encode(tokenAccount.Owner[:])
	fmt.Printf("Owner (Base58 Address): %s\n", ownerBase58)
	MintBase58 := base58.Encode(tokenAccount.Mint[:])
	fmt.Printf("MintBase58 (Base58 Address): %s\n", MintBase58)

	fmt.Printf("Mint: %x\n", tokenAccount.Mint)
	fmt.Printf("Owner: %x\n", tokenAccount.Owner)
	fmt.Printf("Amount: %d\n", tokenAccount.Amount)
	fmt.Printf("Delegate Option: %d\n", tokenAccount.DelegateOption)
	fmt.Printf("Delegate: %x\n", tokenAccount.Delegate)
	fmt.Printf("State: %d\n", tokenAccount.State)
	fmt.Printf("Is Native Option: %d\n", tokenAccount.IsNativeOption)
	fmt.Printf("Rent Exempt Reserve: %d\n", tokenAccount.RentExemptReserve)
	fmt.Printf("Close Authority Option: %d\n", tokenAccount.CloseAuthorityOption)
	fmt.Printf("Close Authority: %x\n", tokenAccount.CloseAuthority)
}
