package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"time"

	"github.com/__username__/go_boilerplate/cmd/boot"
	//=="github.com/__username__/go_boilerplate/internal/database"
)

func main() {
	err := boot.LoadEnvVariables()
	if err != nil {
		panic(err)
	}

	// Create a root ctx and a CancelFunc which can be used to cancel retentionMap goroutine
	rootCtx := context.Background()
	ctx, cancel := context.WithCancel(rootCtx)
	defer cancel()

	port := boot.Environment.Port

	//==database.Setup(boot.Environment.DSN)

	e := createRouter(ctx)

	go func() {
		e.Logger.Infof("Running Server on port %s", port)
		e.Logger.Infof("Accessible locally at: http://localhost:%s", port)
		e.Logger.Infof("Accessible on the network at: http://%s:%s", boot.Environment.Host, port)
		e.Logger.Infof("Press Ctrl+C to stop the server and exit.")
		e.Logger.Fatal(e.Start(":" + port))
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, os.Interrupt)
	<-quit
	ctx, cancel = context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	if err := e.Shutdown(ctx); err != nil {
		e.Logger.Fatal(err)
	}
}
