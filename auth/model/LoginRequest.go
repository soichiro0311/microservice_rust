package model

type LoginRequest struct {
	UserId   string `json:"userid"`
	Password string `json:"password"`
}

func NewLoginRequest(userId string, password string) *LoginRequest {
	req := new(LoginRequest)
	req.UserId = userId
	req.Password = password
	return req
}
