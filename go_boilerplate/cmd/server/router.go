package main

import (
	"context"
	"net/http"
	"path/filepath"
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
	e.Use(middlewares.RateLimiter())
	// Apply Gzip middleware, but skip it for /metrics
	e.Use(middleware.GzipWithConfig(middleware.GzipConfig{
		Level: 5,
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
	e.POST("/csp-violation-report", func(c echo.Context) error {
		log.Warnf("CSP Violation Report: %s", c.Request().RequestURI)
		return c.NoContent(http.StatusOK)
	})

	e.GET("/sw.js", func(c echo.Context) error {
		c.Response().Header().Set("Content-Type", "application/javascript")
		c.Response().Header().Set("Cache-Control", "no-cache")
		return c.File("./static/sw.js")
	})

	e.Static("/assets", "./static")
	e.GET("/assets/dist/*", func(c echo.Context) error {
		c.Response().Header().Set("Cache-Control", "public, max-age=31536000, immutable")
		return c.File(filepath.Join("./static/dist", c.Param("*")))
	})

	//--wsManager := connections.NewManager(ctx)

	//--e.GET("/ws", wsManager.ServeWS)

	web := e.Group("")

	web.Use(middlewares.SecurityHeaders())

	if boot.Environment.GoEnv == enums.Environments.DEVELOPMENT {
		e.Logger.SetLevel(log.DEBUG)
		log.SetLevel(log.DEBUG)

	}

	if boot.Environment.GoEnv == enums.Environments.PRODUCTION {
		e.Logger.SetLevel(log.INFO)
		log.SetLevel(log.INFO)

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
	//==web.GET("/examples/users", controllers.FetchAllUsers())

	//==web.POST("/examples/users", controllers.AddNewUser())
	//==web.PATCH("/examples/users/:id", controllers.ToggeleUserEmail())
	//==web.DELETE("/examples/users/:id", controllers.DeleteUser())

	web.POST("/errors/below", controllers.BelowFormError())
	web.POST("/errors/replace", controllers.ReplaceFormError())
	web.POST("/errors/toast", controllers.ToastFormError())

	apigrp := e.Group("/api")

	apiv1 := apigrp.Group("/v1")
	apiv1.POST("/cats", api.GetCats())


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
