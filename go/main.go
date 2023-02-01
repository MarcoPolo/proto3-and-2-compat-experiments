package main

import "fmt"

//go:generate protoc --proto_path=$PWD:$PWD/pb --go_out=./pb/ --go_opt=Mpb/circuitv2proto2.proto=./p2 pb/circuitv2proto2.proto
//go:generate protoc --proto_path=$PWD:$PWD/pb --go_out=./pb/ --go_opt=Mpb/circuitv2proto3.proto=./p3 pb/circuitv2proto3.proto

func main() {
	fmt.Println("Hello, World!")
}
