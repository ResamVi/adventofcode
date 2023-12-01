package main

import (
	"bufio"
	"fmt"
	"os"
	"unicode"
)

func main() {
	f, err := os.Open("data.txt")
	if err != nil {
		panic(err)
	}

	sc := bufio.NewScanner(f)

	priority := 0
	for sc.Scan() {
		line := sc.Text()

		half := len(line) / 2
		left, right := line[:half], line[half:]

		m := make(map[rune]interface{}, len(line))

		for _, char := range left {
			m[char] = struct{}{}
		}

		for _, char := range right {
			_, exists := m[char]
			if !exists {
				continue
			}

			if unicode.IsUpper(char) {
				priority += int(char) - 38
			} else {
				priority += int(char) - 96
			}

			break
		}
	}

	fmt.Println(priority)
}
