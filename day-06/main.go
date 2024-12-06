package main

import (
	"os"
	"strings"
)

func firstPart(matrix [][]rune) int {
  orientations := [][]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

  height := len(matrix)
  width := len(matrix[0])

  var x, y int

  for i := 0; i < height; i++ {
    for j := 0; j < width; j++ {
      if matrix[i][j] == '^' {
        x = j
        y = i
        break
      }
    }
  }

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
    
    y = nextY
    x = nextX
  }

  return len(visited)
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

  println("\ninput:")
  println(firstPart(input))
}
