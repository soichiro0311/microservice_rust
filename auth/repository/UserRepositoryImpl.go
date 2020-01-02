package repository

import (
	"../db"

	"../domain"
	"github.com/jinzhu/gorm"
)

type UserRepositoryImpl struct {
	db *gorm.DB
}

func NewUserRepositoryImpl() *UserRepositoryImpl {
	repository := new(UserRepositoryImpl)
	repository.db = db.GetDB()
	return repository
}

func (repository *UserRepositoryImpl) UpdateLoginedFlg(userId string, password string) {
	userBefore := domain.UserInfo{}
	userBefore.UserId = userId
	userBefore.Password = password
	userAfter := userBefore
	repository.db.First(&userAfter)
	userAfter.LoginedFlg = true
	repository.db.Model(&userBefore).Update(&userAfter)
}
