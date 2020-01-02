package service

import (
	"../model"
	"../repository"
)

type UserService struct {
	repository repository.UserRepository
}

func NewUserService(repository repository.UserRepository) *UserService {
	service := new(UserService)
	service.repository = repository
	return service
}

func (service *UserService) Login(req model.LoginRequest) {
	service.repository.UpdateLoginedFlg(req.UserId, req.Password)
}
