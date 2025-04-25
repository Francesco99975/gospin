package controllers

import (
	"net/http"

	"github.com/__username__/go_boilerplate/internal/helpers"
	"github.com/__username__/go_boilerplate/internal/models"
	"github.com/__username__/go_boilerplate/views"
	"github.com/labstack/echo/v4"
)

func Index() echo.HandlerFunc {
	return func(c echo.Context) error {
		data := models.GetDefaultSite("Home")

		html, err := helpers.RenderHTML(views.Index(data))

		if err != nil {
			return echo.NewHTTPError(http.StatusBadRequest, "Could not parse page home")
		}

		return c.Blob(200, "text/html; charset=utf-8", html)
	}
}
