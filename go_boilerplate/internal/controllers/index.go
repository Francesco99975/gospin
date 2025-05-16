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

		html := helpers.MustRenderHTML(views.Index(data))

		return c.Blob(http.StatusOK, "text/html; charset=utf-8", html)
	}
}
