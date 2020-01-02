package web

import (
	"../controller"
	"github.com/gin-gonic/gin"
)

func Init(ctrl *controller.UserController) {
	r := router(ctrl)
	r.Run(":8090")
}

func router(ctrl *controller.UserController) *gin.Engine {
	r := gin.Default()

	u := r.Group("/user")
	{
		u.POST("/login", ctrl.Login)
	}

	return r
}
