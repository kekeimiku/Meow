package main

import (
	"fmt"
	"os"
)

type App struct {
	Mem *os.File
}

func main() {
	v := Vec{}
	v1 := v.New([]uint{1, 2, 3, 4, 5}).Pack()
	v1.Remove(2)
	fmt.Println(v1.Small)
}
