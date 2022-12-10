package main

import (
	"fmt"
	"io"
	"os"
)

type QueueSet struct {
	queue []string
	set   map[string]bool
}

func (qs *QueueSet) Add(r string) {
	qs.queue = append(qs.queue, r)
	if len(qs.queue) > 4 {
		qs.queue = qs.queue[1:5]
	}

	// Yes this is fucking bad but CBA.
	qs.set = make(map[string]bool)
	for _, item := range qs.queue {
		qs.set[item] = true
	}
}

func (qs *QueueSet) IsUnique() bool {
	return len(qs.set) == 4
}

func main() {
	file, err := os.Open("data.txt")
	if err != nil {
		panic(err)
	}

	line, err := io.ReadAll(file)
	if err != nil {
		panic(err)
	}

	qs := QueueSet{
		queue: make([]string, 0),
		set:   make(map[string]bool, 0),
	}

	result := 0
	for i, r := range string(line) {
		if qs.IsUnique() {
			result = i
			break
		}

		qs.Add(string(r))
	}

	fmt.Println(result)
}
