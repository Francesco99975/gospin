package connections

import (
	"encoding/json"
	"fmt"
)

type Event struct {
	Type    string          `json:"type"`
	Payload json.RawMessage `json:"payload"`
}

type HtmlData struct {
	Id   string `json:"id"`
	Html string `json:"html"`
}

type EventHandler func(event Event, c *Client) error

const (
	EventNewCategory = "newcategory"
)

func SendNewCategoryHandler(event Event, client *Client) error {
	var outgoingEvent Event
	outgoingEvent.Payload = event.Payload
	outgoingEvent.Type = EventNewCategory

	for client := range client.manager.clients {
		// Only send to clients inside the same chatroom
		if client.room != "admin" {
			client.egress <- outgoingEvent
		}
	}
	return nil
}

type SendOtp struct {
	OTP string `json:"otp"`
}

func SendOtpHandler(event Event, client *Client) error {
	var payload SendOtp
	if err := json.Unmarshal(event.Payload, &payload); err != nil {
		return fmt.Errorf("bad payload in request: %v", err)
	}

	// Verify OTP is existing
	if !client.manager.otps.VerifyOTP(payload.OTP) {
		return fmt.Errorf("authauthorized bad otp in request")
	}

	client.room = "admin"
	return nil
}
