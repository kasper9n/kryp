package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"time"

	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
)

func main() {
	fmt.Println("")
	fmt.Println("[Kryp] Starting server...")

	connectDB()

	r := gin.New()
	r.Use(gin.Logger())
	r.Use(gin.Recovery())

	r.GET("/ping", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"message": "pong",
		})
	})

	fmt.Println("[Kryp] Listening on port " + os.Getenv("SERVER_PORT"))
	http.ListenAndServe(":"+os.Getenv("SERVER_PORT"), r)
}

func connectDB() {
	user := os.Getenv("DB_USERNAME")
	pass := os.Getenv("DB_PASSWORD")
	const host = "db"
	port := os.Getenv("DB_PORT")
	const db = "kryp"
	uri := fmt.Sprintf("mongodb://%s:%s@%s:%s/%s", user, pass, host, port, db)
	const timeoutSecs = 30
	fmt.Printf("[Kryp] Connecting to DB with %vs timeout...\n", timeoutSecs)

	ctx, cancel := context.WithTimeout(context.Background(), timeoutSecs*time.Second)
	defer cancel()

	client, err := mongo.Connect(ctx, options.Client().ApplyURI(uri))
	if err != nil {
		fmt.Printf("[Kryp err1] %s", err)
		panic(err)
	}

	defer func() {
		if err = client.Disconnect(ctx); err != nil {
			panic(err)
		}
	}()

	// Ping the primary
	if err := client.Ping(ctx, readpref.Primary()); err != nil {
		fmt.Printf("[Kryp err2] %s", err)
		panic(err)
	}

	fmt.Println("[Kryp] Connected to db")

}
