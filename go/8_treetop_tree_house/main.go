package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	// 2  5  5  1  2
	// 7, 1, 3, 4, 9
	// fmt.Println(IsVisble(3, []int{2, 5, 5, 1, 2}))

	karte := make([][]int, 0)

	f, err := os.Open("data.txt")
	if err != nil {
		panic(err)
	}

	sc := bufio.NewScanner(f)

	for sc.Scan() {
		line := sc.Text()

		row := make([]int, 0)
		for _, l := range line {
			n, err := strconv.Atoi(string(l))
			if err != nil {
				panic(err)
			}
			row = append(row, n)
		}
		karte = append(karte, row)
	}
	// for i := range karte {
	// 	for _, tree := range karte[i] {
	// 		fmt.Printf(" %d ", tree)
	// 	}
	// 	fmt.Println()
	// }

	visible := 0
	for i := range karte {
		for j, _ := range karte[i] {
			row := karte[i]
			column := make([]int, 0)
			for k := range karte {
				column = append(column, karte[k][j])
			}
			rowVisible := IsVisble(j, row)
			columnVisible := IsVisble(i, column)

			if rowVisible || columnVisible {
				// fmt.Printf("karte[%d][%d]\n", i, j)
				visible++
			}
		}
	}
	fmt.Println(visible)
}

func IsVisble(index int, heights []int) bool {
	height := heights[index]

	if index == 0 || index == len(heights)-1 {
		return true
	}

	left := true
	for _, neighbour := range heights[:index] {
		if neighbour-height >= 0 {
			left = false
			break
		}
	}

	right := true
	for _, neighbour := range heights[index+1:] {
		if neighbour-height >= 0 {
			right = false
			break
		}
	}

	return left || right
}
