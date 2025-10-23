package helpers

import (
	"fmt"

	"github.com/__username__/go_boilerplate/internal/models"
	"github.com/__username__/go_boilerplate/internal/monitoring"
	"github.com/__username__/go_boilerplate/views"
	"github.com/labstack/echo/v4"
	"github.com/labstack/gommon/log"
)

type GenericError struct {
	Code        int      `json:"code"`
	Message     string   `json:"message"`
	UserMessage string   `json:"userMessage"`
	Errors      []string `json:"errors"`
}

func (ge *GenericError) Stringify() string {
	return fmt.Sprintf("[%d] %s <-- %v", ge.Code, ge.Message, ge.Errors)
}

func SendReturnedGenericJSONError(c echo.Context, err GenericError, r *Reporter) error {
	monitoring.RecordError(fmt.Sprintf("%d", err.Code))
	log.Errorf(err.Stringify())

	if r != nil {
		_ = r.Report(SeverityLevels.ERROR, err.Stringify())
	}

	return c.JSON(err.Code, models.JSONErrorResponse{Code: err.Code, Message: err.UserMessage, Errors: err.Errors})
}

func SendReturnedGenericHTMLError(c echo.Context, err GenericError, r *Reporter) error {
	monitoring.RecordError(fmt.Sprintf("%d", err.Code))
	log.Errorf(err.Stringify())

	if r != nil {
		_ = r.Report(SeverityLevels.ERROR, err.Stringify())
	}

	html := MustRenderHTML(views.Error(models.GetDefaultSite("Error"), fmt.Sprintf("%d", err.Code), err.UserMessage))

	return c.Blob(err.Code, "text/html", html)
}
