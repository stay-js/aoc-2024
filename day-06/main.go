package main

import (
	"os"
	"strings"
)

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

func firstPart(matrix [][]rune) int {
  orientations := [][]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

  height := len(matrix)
  width := len(matrix[0])

  x, y := getStartingPosition(matrix)

  visited := make(map[[2]int]bool)
  orientation := 0

  for y >= 0 && y < height && x >= 0 && x < width {
    visited[[2]int{y, x}] = true 

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

  return len(visited)
}


func secondPart(matrix [][]rune) int {
	orientations := [][]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	height := len(matrix)
	width := len(matrix[0])

	startX, startY := getStartingPosition(matrix)

	simulate := func(tempMatrix [][]rune) bool {
		x, y := startX, startY

		orientation := 0
		visited := make(map[[3]int]bool)

		for {
			if visited[[3]int{y, x, orientation}] {
				return true 
			}

			visited[[3]int{y, x, orientation}] = true

			nextY := y + orientations[orientation][0]
			nextX := x + orientations[orientation][1]

			if nextY < 0 || nextY >= height || nextX < 0 || nextX >= width {
				return false
			}

			if tempMatrix[nextY][nextX] == '#' {
				orientation = (orientation + 1) % 4
				continue
			}

			x, y = nextX, nextY
		}
	}

	validPositions := 0

	for i := 0; i < height; i++ {
		for j := 0; j < width; j++ {
			if matrix[i][j] == '.' {
				tempMatrix := make([][]rune, height)

				for k := range matrix {
					tempMatrix[k] = append([]rune(nil), matrix[k]...)
				}

				tempMatrix[i][j] = '#'

				if simulate(tempMatrix) {
					validPositions++
				}
			}
		}
	}

	return validPositions
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

func main() {
  demoInput := getMatrix("demo-input.txt")
  input := getMatrix("input.txt")

  println("demo-input:")
  println(firstPart(demoInput))
  println(secondPart(demoInput))

  println("\ninput:")
  println(firstPart(input))
  println(secondPart(input))
}
