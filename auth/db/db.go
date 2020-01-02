package db

import (
	"fmt"

	"../domain"
	"github.com/jinzhu/gorm"
	_ "github.com/lib/pq"
)

var (
	db  *gorm.DB
	err error
)

func Init() {
	db, err = gorm.Open("postgres", "host=127.0.0.1 port=5432 user=gorm dbname=gorm password=gorm sslmode=disable")
	fmt.Print(db)
	migrate()
	initMaster()
	if err != nil {
		panic(err)
	}
}

func migrate() {
	db.AutoMigrate(&domain.UserInfo{})
}

func GetDB() *gorm.DB {
	return db
}

func initMaster() {
	user1 := domain.NewUser("Mike", "Mike")
	user2 := domain.NewUser("John", "John")
	db.Save(&user1)
	db.Save(&user2)
}
