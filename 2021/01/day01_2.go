package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
)

func countDecreasingWindows(readings []int) int {
	count := 0
	last := -1
	if len(readings) < 3 {
		return 0
	}
	fmt.Println(readings[0])
	fmt.Println(readings[1])
	for idx := range readings[2:] {
		current := readings[idx] + readings[idx+1] + readings[idx+2]
		fmt.Printf("%d %d", readings[idx+2], current)
		if current > last && last != -1 {
			fmt.Println(" (increased)")
			count++
		} else {
			fmt.Println("")
		}
		last = current
	}
	return count
}

func readlines(reader io.Reader) []int {
	scanner := bufio.NewScanner(reader)
	var result []int
	for scanner.Scan() {
		current, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}
		result = append(result, current)
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}
	return result
}

func main() {
	lines := readlines(os.Stdin)
	count := countDecreasingWindows(lines)
	fmt.Println(count)
}
