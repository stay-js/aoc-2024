package main

import (
	"os"
	"strings"
)

type Position struct {
	x, y int
}

func getMatrix(fileName string) [][]rune {
	data, err := os.ReadFile(fileName)

	if err != nil {
		panic(err)
	}

	rows := strings.Split(string(data), "\n")
	matrix := make([][]rune, len(rows))

	for i, row := range rows {
		matrix[i] = []rune(row)
	}

	return matrix
}

func getStartingPosition(matrix [][]rune) (int, int) {
	for i, row := range matrix {
		for j, cell := range row {
			if cell == '^' {
				return j, i
			}
		}
	}

	panic("invalid input")
}

func traverseMatrix(matrix [][]rune, onVisit func(Position, int) bool) {
	orientations := [][]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}
	height, width := len(matrix), len(matrix[0])

	x, y := getStartingPosition(matrix)
	orientation := 0

	for y >= 0 && y < height && x >= 0 && x < width {
		if !onVisit(Position{x, y}, orientation) {
			break
		}

		nextY := y + orientations[orientation][0]
		nextX := x + orientations[orientation][1]

		if nextY < 0 || nextY >= height || nextX < 0 || nextX >= width {
			break
		}

		if matrix[nextY][nextX] == '#' {
			orientation = (orientation + 1) % 4
			continue
		}

		x, y = nextX, nextY
	}
}

func firstPart(matrix [][]rune) map[Position]bool {
	visited := make(map[Position]bool)

	traverseMatrix(matrix, func(pos Position, orientation int) bool {
		visited[pos] = true
		return true
	})

	return visited
}

func secondPart(matrix [][]rune, visited map[Position]bool) int {
	isLoop := func(tempMatrix [][]rune) bool {
		visited := make(map[[3]int]bool)
		loopDetected := false

		traverseMatrix(tempMatrix, func(pos Position, orientation int) bool {
			key := [3]int{pos.y, pos.x, orientation}

			if visited[key] {
				loopDetected = true
				return false
			}

			visited[key] = true
			return true
		})

		return loopDetected
	}

	validPositions := 0

	for pos := range visited {
		if matrix[pos.y][pos.x] != '.' {
			continue
		}

		tempMatrix := make([][]rune, len(matrix))

		for i := range matrix {
			tempMatrix[i] = append([]rune(nil), matrix[i]...)
		}

		tempMatrix[pos.y][pos.x] = '#'

		if isLoop(tempMatrix) {
			validPositions++
		}
	}

	return validPositions
}

func main() {
	demoInput := getMatrix("demo-input.txt")
	input := getMatrix("input.txt")

	println("demo-input:")
	visited := firstPart(demoInput)
	println(len(visited))
	println(secondPart(demoInput, visited))

	println("\ninput:")
	visited = firstPart(input)
	println(len(visited))
	println(secondPart(input, visited))
}
