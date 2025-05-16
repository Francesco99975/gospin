package helpers

import (
	"unicode"
	"fmt"
	"strconv"
	"strings"

	"golang.org/x/text/currency"
	"golang.org/x/text/language"
	"golang.org/x/text/message"
)

func Capitalize(s string) string {
	return string(unicode.ToUpper(rune(s[0]))) + s[1:]
}

func FormatPrice(price float64, curr string) (string, error) {
	p := message.NewPrinter(language.English)
	cur, err := currency.ParseISO(curr)
	if err != nil {
		return "", fmt.Errorf("failed to parse currency: %w", err)
	}
	return p.Sprintf("%v", cur.Amount(price)), nil
}

func ParseNumberString(input string) (int64, error) {
	multipliers := map[string]float64{
		"K": 1_000,
		"M": 1_000_000,
		"B": 1_000_000_000,
		"T": 1_000_000_000_000,
	}

	lastChar := input[len(input)-1:]
	multiplier, hasSuffix := multipliers[lastChar]

	var numStr string
	if hasSuffix {
		numStr = input[:len(input)-1]
	} else {
		numStr = input
		multiplier = 1
	}

	num, err := strconv.ParseFloat(numStr, 64)
	if err != nil {
		return 0, fmt.Errorf("invalid number format: %s", input)
	}

	result := int64(num * multiplier)
	return result, nil
}

func NormalizeFloatStrToIntStr(number string) string {
	number = strings.ReplaceAll(number, ",", "")
	number = strings.ReplaceAll(number, " ", "")
	if !strings.Contains(number, ".") {
		number = strings.ReplaceAll(number, "%", "00")
	} else {
		number = strings.ReplaceAll(number, "%", "")
		number = strings.ReplaceAll(number, ".", "")
	}
	number = strings.ReplaceAll(number, "$", "")

	number = strings.ReplaceAll(number, "(", "")
	number = strings.ReplaceAll(number, ")", "")

	return number
}
