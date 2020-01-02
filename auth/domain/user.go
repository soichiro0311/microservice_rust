package domain

type UserInfo struct {
	UserId     string `gorm:"primary_key"`
	Password   string
	LoginedFlg bool
}

func NewUser(userId string, password string) UserInfo {
	user := new(UserInfo)
	user.UserId = userId
	user.Password = password
	return *user
}

func (user *UserInfo) Login() {
	user.LoginedFlg = true
}
