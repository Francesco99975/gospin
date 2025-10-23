package controllers

import (
	"net/http"

	"github.com/__username__/go_boilerplate/internal/helpers"
	"github.com/__username__/go_boilerplate/internal/models"
	"github.com/__username__/go_boilerplate/views"
	"github.com/labstack/echo/v4"
)

func Examples() echo.HandlerFunc {
	return func(c echo.Context) error {
		data := models.GetDefaultSite("Examples")

		data.CSRF = c.Get("csrf").(string)
		data.Nonce = c.Get("nonce").(string)

		html := helpers.MustRenderHTML(views.Examples(data))

		return c.Blob(http.StatusOK, "text/html; charset=utf-8", html)
	}
}
