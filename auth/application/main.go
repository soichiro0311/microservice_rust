package main

import (
	"../controller"
	"../db"
	"../repository"
	"../service"
	"../web"
)

func main() {
	db.Init()
	userRepository := repository.NewUserRepositoryImpl()
	userService := service.NewUserService(userRepository)
	userController := controller.NewUserController(userService)
	web.Init(userController)
}
