package main

import (
	"bytes"
	"context"
	"net/http"
	"strings"
	"time"
	"fmt"

	"github.com/prometheus/client_golang/prometheus/promhttp"

	"github.com/__username__/go_boilerplate/cmd/boot"
	"github.com/__username__/go_boilerplate/internal/api"
	"github.com/__username__/go_boilerplate/internal/enums"
	"github.com/__username__/go_boilerplate/internal/helpers"

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
	// Apply Gzip middleware, but skip it for /metrics
	e.Use(middleware.GzipWithConfig(middleware.GzipConfig{
		Skipper: func(c echo.Context) bool {
			return c.Path() == "/metrics" // Skip compression for /metrics
		},
	}))
	e.Use(middlewares.MonitoringMiddleware())
	e.GET("/metrics", echo.WrapHandler(promhttp.Handler()), middlewares.MetricsAccessMiddleware())
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
		CookieSecure:   boot.Environment.GoEnv == enums.Environments.PRODUCTION,
		CookieSameSite: http.SameSiteLaxMode,
		Skipper: func(c echo.Context) bool {
			// Skip CSRF for the /webhook route
			return c.Path() == "/webhook"

		},
	}))

	//--go wsManager.Run()

	web.GET("/", controllers.Index())

	web.GET("/examples", controllers.Examples())

	apigrp := e.Group("/api")

	apiv1 := apigrp.Group("/v1")
	apiv1.POST("/cats", api.PlaceholderGet())


	e.HTTPErrorHandler = serverErrorHandler

	return e
}

func serverErrorHandler(err error, c echo.Context) {
	// Default to internal server error (500)
	code := http.StatusInternalServerError
	var message any = "Internal Server Error"

	// Check if it's an echo.HTTPError
	if he, ok := err.(*echo.HTTPError); ok {
		code = he.Code
		message = he.Message
	}

	// Check the Accept header to decide the response format
	if strings.Contains(c.Request().Header.Get("Accept"), "application/json") {
		// Respond with JSON if the client prefers JSON
		_ = c.JSON(code, map[string]any{
			"error":   true,
			"message": message,
			"status":  code,
		})
	} else {
		if code == 404 {
			message = "Page Not Found"
		}
		// Prepare data for rendering the error page (HTML)
		data := models.GetDefaultSite("Error")

		html := helpers.MustRenderHTML(views.Error(data, fmt.Sprintf("%d", code), message.(string)))

		// Respond with HTML (default) if the client prefers HTML
		_ = c.Blob(code, "text/html; charset=utf-8", html)
	}
}
