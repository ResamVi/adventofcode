package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Tuple struct {
	left  int
	right int
}

func (t Tuple) Size() int {
	return t.right - t.left
}

func main() {
	f, err := os.Open("data.txt")
	if err != nil {
		panic(err)
	}

	var count int

	sc := bufio.NewScanner(f)
	for sc.Scan() {
		line := sc.Text()

		// We need to identify the bigger and smaller intervals (bigger as in "covers more numbers")
		elves := strings.Split(line, ",")

		var intervals [2]Tuple
		for i, elf := range elves {
			interval := strings.Split(elf, "-")
			left, right := interval[0], interval[1]

			l, err := strconv.Atoi(left)
			if err != nil {
				panic(err)
			}

			r, err := strconv.Atoi(right)
			if err != nil {
				panic(err)
			}

			intervals[i] = Tuple{left: l, right: r}
		}

		var bigger, smaller Tuple = intervals[0], intervals[1]
		if intervals[0].Size() < intervals[1].Size() {
			bigger = intervals[1]
			smaller = intervals[0]
		}

		// That took a lot of lines oof.

		// Next up we check the borders and we're done.
		if bigger.left <= smaller.left && smaller.right <= bigger.right {
			count++
		}
	}

	fmt.Println(count)
}
