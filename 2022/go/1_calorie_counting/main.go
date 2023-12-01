package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	b, err := os.ReadFile("data.txt")
	if err != nil {
		panic(err)
	}

	elves := make(map[int]int)

	i := 0
	for _, str := range strings.Split(string(b), "\n") {
		// New line signifies a new elf
		if str == "" {
			i++
			continue
		}

		// Add to elf's calories
		num, err := strconv.Atoi(str)
		if err != nil {
			panic(err)
		}

		elves[i] += num
	}

	// Get max
	max := 0
	for _, calories := range elves {
		if max < calories {
			max = calories
		}
	}

	fmt.Println(max)
}
