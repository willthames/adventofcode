package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func calculateTotal(readings []string) int {
	depth := 0
	distance := 0
	for idx := range readings {
		tokens := strings.SplitN(readings[idx], " ", 2)
		if len(tokens) != 2 {
			panic(fmt.Sprintf("Couldn't split string %s", readings[idx]))
		}
		number, err := strconv.Atoi(tokens[1])
		if err != nil {
			panic(err)
		}
		switch tokens[0] {
		case "forward":
			distance += number
		case "up":
			depth -= number
		case "down":
			depth += number
		}
	}
	return distance * depth
}

func readlines(reader io.Reader) []string {
	scanner := bufio.NewScanner(reader)
	var result []string
	for scanner.Scan() {
		result = append(result, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}
	return result
}

func main() {
	lines := readlines(os.Stdin)
	count := calculateTotal(lines)
	fmt.Println(count)
}
