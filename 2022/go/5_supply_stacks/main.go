package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Stack struct {
	items []string
}

func (s *Stack) Push(item string) {
	s.items = append(s.items, item)
}

func (s *Stack) Stack(item string) {
	s.items = append([]string{item}, s.items...)
}

func (s *Stack) Pop() string {
	item := s.items[0]
	s.items = s.items[1:]
	return item
}

func main() {
	f, err := os.Open("data.txt")
	if err != nil {
		panic(err)
	}

	content, err := io.ReadAll(f)
	if err != nil {
		panic(err)
	}

	sc := bufio.NewScanner(bytes.NewReader(content))

	// Get number of columns
	r := regexp.MustCompile(`(\d+)\s$`)

	var (
		drawingEnd  = 0
		columnCount = 0
	)

	for sc.Scan() {
		line := sc.Text()

		if strings.HasPrefix(line, " 1") {
			match := r.FindString(line)
			columnCount, err = strconv.Atoi(match[:len(match)-1])
			if err != nil {
				panic(err)
			}

			break
		}
		drawingEnd++
	}

	sc = bufio.NewScanner(bytes.NewReader(content))
	stacks := make([]Stack, columnCount)

	for i := 0; i <= drawingEnd; sc.Scan() {
		line := sc.Text()

		items := toSlice(line)
		for i, item := range items {
			if item != " " {
				stacks[i].Push(item)
			}
		}

		i++
	}

	sc.Scan()

	r = regexp.MustCompile(`move (\d+) from (\d+) to (\d+)`)
	for sc.Scan() {
		line := sc.Text()

		matches := r.FindStringSubmatch(line)
		count, _ := strconv.Atoi(matches[1])
		from, _ := strconv.Atoi(matches[2])
		to, _ := strconv.Atoi(matches[3])

		for i := 0; i < count; i++ {
			item := stacks[from-1].Pop()
			stacks[to-1].Stack(item)
		}
	}

	var result string
	for _, stack := range stacks {
		result += stack.items[0]
	}
	fmt.Println(result)
}

func toSlice(line string) []string {
	result := make([]string, 0)
	for i := 1; i < len(line); i += 4 {
		result = append(result, string(line[i]))
	}

	return result
}
