package main

import (
	"flag"
	"fmt"
	"log"
	"net/http"

	"github.com/NYTimes/gziphandler"
)

func main() {
	var port = flag.String("port", "8081", "input port")
	var path = flag.String("path", ".", "input path")
	flag.Parse()

	fmt.Println("127.0.0.1:" + *port)
	fmt.Println("path: " + *path)

	err := http.ListenAndServe(":"+*port,
		gziphandler.GzipHandler(http.FileServer(http.Dir(*path))))
	if err != http.ErrServerClosed {
		log.Fatal(err)
	}
}
