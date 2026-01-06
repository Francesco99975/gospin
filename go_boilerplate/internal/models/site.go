package models

import (
	"fmt"
	"net/http"
	"time"

	"github.com/__username__/go_boilerplate/cmd/boot"
	"github.com/__username__/go_boilerplate/internal/enums"
	"github.com/gorilla/sessions"
)

type Organization struct {
	Context      string         `json:"@context"`
	Type         string         `json:"@type"`
	Name         string         `json:"name"`
	Url          string         `json:"url"`
	Logo         string         `json:"logo"`
	ContactPoint []ContactPoint `json:"contactPoint"`
}

type ContactPoint struct {
	Type        string `json:"@type"`
	Telephone   string `json:"telephone"`
	ContactType string `json:"contactType"`
}

type SEO struct {
	Description string
	Keywords    string
	Author      string
}
type Site struct {
	AppName      string
	Title        string
	Metatags     SEO
	Year         int
	CSRF         string
	Nonce        string
	Organization Organization
	Styles       []string
	SeoScripts   []string
	PageScripts  []string
	JSFile       string
	JSIntegrity  string
	CSSFile      string
	CSSIntegrity string
}

func GetDefaultSite(title string) Site {
	jsFile, jsIntegrity := GetJS()

	cssFile, cssIntegrity := GetCSS()

	return Site{
		AppName:  "GoApp",
		Title:    title,
		Metatags: SEO{Description: "App", Keywords: "tool", Author: "kalairen"},
		Year:     time.Now().Year(),
		Organization: Organization{
			Context:      "https://schema.org",
			Type:         "Organization",
			Name:         "GoApp",
			Url:          boot.Environment.URL,
			Logo:         fmt.Sprintf("%s/assets/images/pwa-512x512.png", boot.Environment.URL),
			ContactPoint: []ContactPoint{{Type: "Person", Telephone: "+1-202-555-0144", ContactType: "customer service"}},
		},
		JSFile:       jsFile,
		JSIntegrity:  jsIntegrity,
		CSSFile:      cssFile,
		CSSIntegrity: cssIntegrity,
	}
}

func GetSessionOptions(remember bool) *sessions.Options {
	sameSite := http.SameSiteDefaultMode
	maxAge := 86400 * 7 // One Week
	if remember {
		maxAge = maxAge * 52 // One Year
	}

	if boot.Environment.GoEnv == enums.Environments.DEVELOPMENT {
		sameSite = http.SameSiteNoneMode

		if remember {
			maxAge = 0 // Infinite
		} else {
			maxAge = 86400 / 24 / 60 * 5 // 5 minutes
		}

	}

	return &sessions.Options{
		Path:     "/",
		MaxAge:   maxAge,
		HttpOnly: true,
		Secure:   boot.Environment.GoEnv != enums.Environments.DEVELOPMENT,
		Domain:   boot.Environment.Host,
		SameSite: sameSite,
	}

}
