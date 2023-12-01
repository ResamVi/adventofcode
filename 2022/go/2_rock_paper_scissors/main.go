package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("data.txt")
	if err != nil {
		panic(err)
	}

	selection := map[string]int{
		"X": 1,
		"Y": 2,
		"Z": 3,
	}

	outcome := map[string]int{
		"A X": 3,
		"C X": 6,
		"A Y": 6,
		"B Y": 3,
		"B Z": 6,
		"C Z": 3,
	}

	sc := bufio.NewScanner(file)

	score := 0
	for sc.Scan() {
		response := strings.Split(sc.Text(), " ")[1]
		score += selection[response] + outcome[sc.Text()]
	}

	fmt.Println(score)
}
