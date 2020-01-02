package repository

type UserRepository interface {
	UpdateLoginedFlg(userId string, password string)
}
