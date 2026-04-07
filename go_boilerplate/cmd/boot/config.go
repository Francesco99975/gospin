package boot

import (
	"fmt"
	"net"
	"os"

	//===
	"regexp"

	===//

	"github.com/__username__/go_boilerplate/internal/enums"
	//%-"github.com/joho/godotenv"
)

func getLocalIP() string {
	conn, err := net.Dial("udp", "8.8.8.8:80")
	if err != nil {
		panic(err)
	}
	defer func() {
		err := conn.Close()
		if err != nil {
			panic(err)
		}
	}()

	localAddr := conn.LocalAddr().(*net.UDPAddr)
	return localAddr.IP.String()
}

//===
var dsnRegex = regexp.MustCompile(`^postgresql:\/\/([a-zA-Z0-9._%+-]+):([^@]+)@([a-zA-Z0-9.-]+):(\d+)\/([a-zA-Z0-9._-]+)\?sslmode=(disable|require|verify-ca|verify-full)$`)

func isValidDSN(dsn string) bool {
	return dsnRegex.MatchString(dsn)
}

===//

type Config struct {
	Port  string
	Host  string
	GoEnv enums.Environment
	//===
	DSN string
	===//
	NTFY         string
	NTFYToken    string
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
	//===
	Environment.DSN = os.Getenv("DSN")
	if !isValidDSN(Environment.DSN) {
		return fmt.Errorf("invalid DSN: %s", Environment.DSN)
	}
	===//
	Environment.NTFY = os.Getenv("NTFY")
	Environment.NTFYToken = os.Getenv("NTFY_TOKEN")
	Environment.MetricSecret = os.Getenv("METRIC_SECRET")
	Environment.Prometheus = os.Getenv("PROMETHEUS")
	if Environment.GoEnv == enums.Environments.DEVELOPMENT {
		localIP := getLocalIP()
		Environment.URL = fmt.Sprintf("http://%s:%s", localIP, Environment.Port)
	} else {
		Environment.URL = fmt.Sprintf("https://%s", Environment.Host)
	}

	return nil
}
