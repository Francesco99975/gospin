package models

import (
	"fmt"
	"net/http"
	"time"

	"github.com/__username__/go_boilerplate/cmd/boot"
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
}

func GetDefaultSite(title string) Site {
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
	}
}

func GetSessionOptions(remember bool) *sessions.Options {
	sameSite := http.SameSiteDefaultMode
	maxAge := 86400 * 7 // One Week
	if remember {
		maxAge = maxAge * 52 // One Year
	}

	if boot.Environment.GoEnv == "development" {
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
		Secure:   !(boot.Environment.GoEnv == "development"),
		Domain:   boot.Environment.Host,
		SameSite: sameSite,
	}

}
