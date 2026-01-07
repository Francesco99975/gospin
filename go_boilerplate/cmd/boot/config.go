package boot

import (
	"fmt"
	"os"

	"github.com/__username__/go_boilerplate/internal/enums"
	//%-"github.com/joho/godotenv"
)

type Config struct {
	Port  string
	Host  string
	GoEnv enums.Environment
	//==DSN   string
	NTFY         string
	URL          string
	MetricSecret string
	Prometheus   string
}

var Environment = &Config{}

func LoadEnvVariables() error {
	//%-err := godotenv.Load(".env")
	//%-if err != nil {
	//%-	return fmt.Errorf("cannot load environment variables")
	//%-}

	if !enums.IsEnvironmentValid(os.Getenv("GO_ENV")) {
		return fmt.Errorf("invalid environment variable: %s", os.Getenv("GO_ENV"))
	}

	Environment.Port = os.Getenv("PORT")
	Environment.Host = os.Getenv("HOST")
	Environment.GoEnv = enums.GetEnvironmentFromString(os.Getenv("GO_ENV"))
	//==Environment.DSN = os.Getenv("DSN")
	Environment.NTFY = os.Getenv("NTFY")
	Environment.MetricSecret = os.Getenv("METRIC_SECRET")
	Environment.Prometheus = os.Getenv("PROMETHEUS")
	if Environment.GoEnv == enums.Environments.DEVELOPMENT {
		Environment.URL = fmt.Sprintf("http://%s:%s", Environment.Host, Environment.Port)
	} else {
		Environment.URL = fmt.Sprintf("https://%s", Environment.Host)
	}

	return nil
}
