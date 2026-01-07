package controllers

import (
	//=="bytes"
	//=="context"
	"net/http"
	//=="strconv"
	//=="strings"

	//=="github.com/google/uuid"

	//=="github.com/__username__/go_boilerplate/internal/database"
	//=="github.com/__username__/go_boilerplate/internal/repository"

	"github.com/__username__/go_boilerplate/internal/helpers"
	"github.com/__username__/go_boilerplate/internal/enums"
	"github.com/__username__/go_boilerplate/internal/models"
	"github.com/__username__/go_boilerplate/views"
	//=="github.com/__username__/go_boilerplate/views/components"
	"github.com/labstack/echo/v4"
	//=="github.com/labstack/gommon/log"
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

//===func FetchAllUsers() echo.HandlerFunc {
	return func(c echo.Context) error {
		repo := repository.New(database.Pool())

		users, err := repo.GetAllUsers(context.Background())

		if err != nil {
			return helpers.SendReturnedGenericHTMLError(c, helpers.GenericError{Code: http.StatusInternalServerError, Message: err.Error(), UserMessage: "Error fetching users"}, nil)
		}

		csrf := c.Get("csrf").(string)

		html := helpers.MustRenderHTML(components.UsersList(users, csrf))

		return c.Blob(http.StatusOK, "text/html; charset=utf-8", html)

	}
}

func AddNewUser() echo.HandlerFunc {
	return func(c echo.Context) error {
		repo := repository.New(database.Pool())

		username := c.FormValue("username")
		email := c.FormValue("email")

		user, err := repo.CreateUser(context.Background(), repository.CreateUserParams{
			ID:       uuid.New(),
			Username: username,
			Email:    email,
		})

		if err != nil {
			return helpers.SendReturnedGenericHTMLError(c, helpers.GenericError{Code: http.StatusInternalServerError, Message: err.Error(), UserMessage: "Error creating user"}, nil)
		}

		userCount, err := repo.CountUsers(context.Background())
		if err != nil {
			return helpers.SendReturnedGenericHTMLError(c, helpers.GenericError{Code: http.StatusInternalServerError, Message: err.Error(), UserMessage: "Error counting users"}, nil)
		}

		csrf := c.Get("csrf").(string)

		html := helpers.MustRenderHTML(components.UserItem(user.ID, user.Username, user.Email, csrf))
		html = append(html, helpers.MustRenderHTML(components.UserCountPartial(strconv.FormatInt(int64(int(userCount)), 10), true))...)
		html = append(html, helpers.MustRenderHTML(components.EmptyUserMessage(userCount == 0, true))...)

		return c.Blob(http.StatusOK, "text/html; charset=utf-8", html)
	}
}

func ToggeleUserEmail() echo.HandlerFunc {
	return func(c echo.Context) error {
		repo := repository.New(database.Pool())

		id := c.Param("id")
		log.Debugf("ID: %s", id)
		uid, err := uuid.Parse(id)

		if err != nil {
			return helpers.SendReturnedGenericHTMLError(c, helpers.GenericError{Code: http.StatusInternalServerError, Message: err.Error(), UserMessage: "Error fetching user"}, nil)
		}

		user, err := repo.GetUserByID(context.Background(), uid)

		if err != nil {
			return helpers.SendReturnedGenericHTMLError(c, helpers.GenericError{Code: http.StatusInternalServerError, Message: err.Error(), UserMessage: "Error fetching user"}, nil)
		}
		var newEmail string
		if strings.Contains(user.Email, ".dev") {
			newEmail = strings.Replace(user.Email, ".dev", ".com", 1)
		} else {
			newEmail = strings.Replace(user.Email, ".com", ".dev", 1)
		}

		log.Debugf("New email: %s", newEmail)

		updatedUser, err := repo.UpdateUserEmail(context.Background(), repository.UpdateUserEmailParams{
			ID:    user.ID,
			Email: newEmail,
		})

		if err != nil {
			return helpers.SendReturnedGenericHTMLError(c, helpers.GenericError{Code: http.StatusInternalServerError, Message: err.Error(), UserMessage: "Error updating user"}, nil)
		}

		html := helpers.MustRenderHTML(components.EmailPartial(updatedUser.ID.String(), updatedUser.Email))

		return c.Blob(http.StatusOK, "text/html; charset=utf-8", html)

	}
}

func DeleteUser() echo.HandlerFunc {
	return func(c echo.Context) error {
		repo := repository.New(database.Pool())

		id := c.Param("id")
		uid, err := uuid.Parse(id)

		log.Debugf("ID: %s", id)

		if err != nil {
			return helpers.SendReturnedGenericHTMLError(c, helpers.GenericError{Code: http.StatusInternalServerError, Message: err.Error(), UserMessage: "Error parsing UUID"}, nil)
		}

		rows, err := repo.DeleteUser(context.Background(), uid)

		if err != nil {
			return helpers.SendReturnedGenericHTMLError(c, helpers.GenericError{Code: http.StatusInternalServerError, Message: err.Error(), UserMessage: "Error deleting user"}, nil)
		}
		if rows == 0 {
			return helpers.SendReturnedGenericHTMLError(c, helpers.GenericError{Code: http.StatusNotFound, Message: "User Not found for deletion", UserMessage: "User not found"}, nil)
		}

		userCount, err := repo.CountUsers(context.Background())
		if err != nil {
			return helpers.SendReturnedGenericHTMLError(c, helpers.GenericError{Code: http.StatusInternalServerError, Message: err.Error(), UserMessage: "Error counting users"}, nil)
		}

		// Build response with multiple fragments
		var buf bytes.Buffer

		// 1. Main response: empty div to remove the deleted user from DOM
		// This targets the hx-target specified in the delete button
		buf.WriteString("") // Empty response removes the element

		// 2. OOB: Update user count
		buf.Write(helpers.MustRenderHTML(components.UserCountPartial(strconv.Itoa(int(userCount)), true)))

		// 3. OOB: Show empty message if no users left
		buf.Write(helpers.MustRenderHTML(components.EmptyUserMessage(userCount == 0, true)))

		return c.Blob(http.StatusOK, "text/html; charset=utf-8", buf.Bytes())
	}
}===//

func BelowFormError() echo.HandlerFunc {
	return func(c echo.Context) error {
		return helpers.SendReturnedHTMLErrorMessage(c, helpers.ErrorMessage{
			Error:       helpers.GenericError{Code: http.StatusBadRequest, Message: "Error in form", UserMessage: "I was programmed to be gone in 20 seconds"},
			Box:         enums.Boxes.BELOW,
			Persistance: "20000"}, nil)
	}
}

func ReplaceFormError() echo.HandlerFunc {
	return func(c echo.Context) error {
		return helpers.SendReturnedHTMLErrorMessage(c, helpers.ErrorMessage{
			Error:       helpers.GenericError{Code: http.StatusBadRequest, Message: "Error in form", UserMessage: "I was programmed to stay put"},
			Box:         enums.Boxes.REPLACE,
			Persistance: ""}, nil)
	}
}

func ToastFormError() echo.HandlerFunc {
	return func(c echo.Context) error {
		return helpers.SendReturnedHTMLErrorMessage(c, helpers.ErrorMessage{
			Error:       helpers.GenericError{Code: http.StatusBadRequest, Message: "Error in form", UserMessage: "I was programmed to be gone in 3 seconds"},
			Box:         enums.Boxes.TOAST_TR,
			Persistance: "3000"}, nil)
	}
}
