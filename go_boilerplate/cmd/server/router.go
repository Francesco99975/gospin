package main

import (
	"bytes"
	"context"
	"net/http"
	"strings"
	"time"

	"github.com/__username__/go_boilerplate/cmd/boot"
	"github.com/__username__/go_boilerplate/internal/api"

	//--"github.com/__username__/go_boilerplate/internal/connections"

	"github.com/__username__/go_boilerplate/internal/controllers"
	"github.com/__username__/go_boilerplate/internal/middlewares"
	"github.com/__username__/go_boilerplate/internal/models"
	"github.com/__username__/go_boilerplate/views"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/labstack/gommon/log"
)

func createRouter(ctx context.Context) *echo.Echo {
	e := echo.New()
	e.Use(middleware.Logger())
	e.Use(middleware.RemoveTrailingSlash())
	e.GET("/healthcheck", func(c echo.Context) error {
		time.Sleep(5 * time.Second)
		return c.JSON(http.StatusOK, "OK")
	})

	e.Static("/assets", "./static")

	//--wsManager := connections.NewManager(ctx)

	//--e.GET("/ws", wsManager.ServeWS)

	web := e.Group("")

	if boot.Environment.GoEnv == "development" {
		e.Logger.SetLevel(log.DEBUG)
		log.SetLevel(log.DEBUG)
		web.Use(middlewares.SecurityHeadersDev())
	}

	if boot.Environment.GoEnv == "production" {
		e.Logger.SetLevel(log.INFO)
		log.SetLevel(log.INFO)
		web.Use(middlewares.SecurityHeaders())
	}

	web.Use(middleware.CSRFWithConfig(middleware.CSRFConfig{
		TokenLookup:    "form:_csrf,header:X-CSRF-Token",
		CookieName:     "csrf_token",
		CookiePath:     "/",
		CookieHTTPOnly: true,
		Skipper: func(c echo.Context) bool {
			// Skip CSRF for the /webhook route
			return c.Path() == "/webhook"

		},
	}))

	//--go wsManager.Run()

	web.GET("/", controllers.Index())

	apigrp := e.Group("/api")

	apiv1 := apigrp.Group("/v1")
	apiv1.GET("/", api.PlaceholderGet())

	e.HTTPErrorHandler = serverErrorHandler

	return e
}

func serverErrorHandler(err error, c echo.Context) {
	// Default to internal server error (500)
	code := http.StatusInternalServerError
	var message interface{} = "An unexpected error occurred"

	// Check if it's an echo.HTTPError
	if he, ok := err.(*echo.HTTPError); ok {
		code = he.Code
		message = he.Message
	}

	// Check the Accept header to decide the response format
	if strings.Contains(c.Request().Header.Get("Accept"), "application/json") {
		// Respond with JSON if the client prefers JSON
		_ = c.JSON(code, map[string]interface{}{
			"error":   true,
			"message": message,
			"status":  code,
		})
	} else {
		// Prepare data for rendering the error page (HTML)
		data := models.GetDefaultSite("Error")

		// Buffer to hold the HTML content (in case of HTML response)
		buf := bytes.NewBuffer(nil)

		// Render based on the status code
		if code >= 500 {
			_ = views.ServerError(data, err).Render(context.Background(), buf)
		} else {
			_ = views.ClientError(data, err).Render(context.Background(), buf)
		}
		// Respond with HTML (default) if the client prefers HTML
		_ = c.Blob(code, "text/html; charset=utf-8", buf.Bytes())
	}
}
