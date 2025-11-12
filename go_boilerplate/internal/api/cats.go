package api

import (
	"fmt"
	"net/http"

	"github.com/__username__/go_boilerplate/internal/models"
	"github.com/__username__/go_boilerplate/internal/helpers"
	"github.com/labstack/echo/v4"
)

func GetCats() echo.HandlerFunc {
	return func(c echo.Context) error {
		cats, err := models.GetCats()
		if err != nil {
			return helpers.SendReturnedGenericJSONError(c, helpers.GenericError{Code: http.StatusNotFound, Message: fmt.Sprintf("Error fetching categories <-- %v", err.Error()), UserMessage: "No categories where found at this time...", Errors: []string{err.Error()}}, nil)
		}

		return c.JSON(http.StatusOK, cats)
	}
}

