def first_part(string: str) -> int:
    data = string.split('\n')

    height = len(data)
    width = len(data[0])

    antennas = {}

    for y in range(height):
        for x in range(width):
            if data[y][x] != ".":
                antennas[(x, y)] = data[y][x]

    antinodes = set()

    for antenna in antennas:
        for pair in antennas:
            if antenna == pair or antennas[antenna] != antennas[pair]:
                continue

            x1, y1 = antenna
            x2, y2 = pair
            dx = x2 - x1
            dy = y2 - y1

            antinode1 = (x1 - dx, y1 - dy)
            antinode2 = (x2 + dx, y2 + dy)

            if 0 <= antinode1[0] < width and 0 <= antinode1[1] < height:
                antinodes.add(antinode1)
            if 0 <= antinode2[0] < width and 0 <= antinode2[1] < height:
                antinodes.add(antinode2)

    return len(antinodes)


def main():
    with open("demo-input.txt") as file:
        demo_input = file.read()

    with open("input.txt") as file:
        real_input = file.read()

    print("demo-input:")
    print(first_part(demo_input))

    print("\ninput:")
    print(first_part(real_input))


if __name__ == '__main__':
    main()
