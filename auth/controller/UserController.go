package controller

import (
	"../model"
	"../service"
	"github.com/gin-gonic/gin"
)

type UserController struct {
	service *service.UserService
}

func NewUserController(service *service.UserService) *UserController {
	controller := new(UserController)
	controller.service = service
	return controller
}

func (ctrl UserController) Login(c *gin.Context) {
	req := model.LoginRequest{}
	c.BindJSON(&req)
	ctrl.service.Login(req)
}
