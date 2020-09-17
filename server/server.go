package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/op/go-logging"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
)

var log = logging.MustGetLogger("example")
var format1 = logging.MustStringFormatter(
	`%{color}[Kryp] %{level:.4s} %{id:03x}%{color:reset} %{message}`,
)

func main() {

	backend := logging.NewLogBackend(os.Stderr, "", 0)
	backendFormatter := logging.NewBackendFormatter(backend, format1)
	backendLeveled := logging.AddModuleLevel(backend)
	backendLeveled.SetLevel(logging.ERROR, "")

	logging.SetBackend(backendFormatter)
	fmt.Println("")
	log.Info("Starting server...")

	log.Error("oh geez an intimate error")
	log.Error("another err ohno")
	log.Info("look at that")

	connectDB()

	r := gin.New()
	r.Use(gin.Logger())
	r.Use(gin.Recovery())

	r.GET("/ping", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"message": "pong",
		})
	})

	http.ListenAndServe(":"+os.Getenv("SERVER_PORT"), r)
	log.Info("Listening on port " + os.Getenv("SERVER_PORT"))
}

func connectDB() {
	user := os.Getenv("DB_USERNAME")
	pass := os.Getenv("DB_PASSWORD")
	const host = "db"
	port := os.Getenv("DB_PORT")
	const db = "kryp"
	uri := fmt.Sprintf("mongodb://%s:%s@%s:%s/%s", user, pass, host, port, db)
	const timeoutSecs = 30
	log.Infof("Connecting to DB with %vs timeout...", timeoutSecs)

	ctx, cancel := context.WithTimeout(context.Background(), timeoutSecs*time.Second)
	defer cancel()

	client, err := mongo.Connect(ctx, options.Client().ApplyURI(uri))
	if err != nil {
		log.Errorf("%s", err)
		panic(err)
	}

	defer func() {
		if err = client.Disconnect(ctx); err != nil {
			panic(err)
		}
	}()

	// Ping the primary
	if err := client.Ping(ctx, readpref.Primary()); err != nil {
		log.Errorf("%s", err)
		panic(err)
	}

	log.Info("Connected to db")

}
