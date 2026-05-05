package main

import (
	l36io "github.com/incyashraj/layer6x6/packages/sdk-go/layer36/io"
	l36net "github.com/incyashraj/layer6x6/packages/sdk-go/layer36/net"
)

func main() {
	args := l36io.Args()
	if len(args) == 0 {
		_ = l36io.Eprintln("usage: layer36-go-curl <url>")
		return
	}

	body, err := l36net.GetText(args[0])
	if err != nil {
		_ = l36io.Eprintln("layer36-go-curl: " + err.Error())
		_ = l36io.Eprintln("layer36-go-curl: fetch failed")
		return
	}

	_ = l36io.Print(body)
}
