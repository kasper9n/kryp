package main

import (
	"fmt"
	"os"

	"github.com/gin-gonic/gin"
)

func main() {
	fmt.Println("")
	fmt.Println("\033[0;36m" + "Starting server..." + "\033[0m")
	r := gin.New()
	r.Use(gin.Logger())
	r.Use(gin.Recovery())

	r.GET("/ping", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"message": "pong",
		})
	})

	r.Run(":" + os.Getenv("SERVER_PORT"))
}
