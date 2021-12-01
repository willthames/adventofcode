package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
)

func countDecreasingLines(reader io.Reader) int {
	count := 0
	last := -1
	scanner := bufio.NewScanner(reader)
	for scanner.Scan() {
		current, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}
		if current > last && last != -1 {
			count++
		}
		last = current
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}
	return count
}

func main() {
	count := countDecreasingLines(os.Stdin)
	fmt.Println(count)
}
