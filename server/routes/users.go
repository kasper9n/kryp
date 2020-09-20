package routes

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

// UsersRegister register user
func register(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"user_id": "someid",
	})
	// return "hwd"
}
