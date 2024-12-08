def get_antennas_and_boundaries(string: str):
    data = string.split('\n')

    height = len(data)
    width = len(data[0])

    antennas: dict[tuple[int, int], str] = {}

    for y in range(height):
        for x in range(width):
            if data[y][x] != ".":
                antennas[(x, y)] = data[y][x]

    return antennas, width, height


def calculate_distances_and_create_antinodes(a: tuple[int, int], b: tuple[int, int]):
    x1, y1 = a
    x2, y2 = b
    dx = x2 - x1
    dy = y2 - y1

    antinode1 = (x1 - dx, y1 - dy)
    antinode2 = (x2 + dx, y2 + dy)

    return dx, dy, antinode1, antinode2


def first_part(string: str) -> int:
    antennas, width, height = get_antennas_and_boundaries(string)
    antinodes: set[tuple[int, int]] = set()

    for antenna in antennas:
        for pair in antennas:
            if antenna == pair or antennas[antenna] != antennas[pair]:
                continue

            _, _, antinode1, antinode2 = calculate_distances_and_create_antinodes(antenna, pair)

            if 0 <= antinode1[0] < width and 0 <= antinode1[1] < height:
                antinodes.add(antinode1)
            if 0 <= antinode2[0] < width and 0 <= antinode2[1] < height:
                antinodes.add(antinode2)

    return len(antinodes)


def second_part(string: str) -> int:
    antennas, width, height = get_antennas_and_boundaries(string)
    antinodes: set[tuple[int, int]] = set()

    for antenna in antennas:
        antinodes.add(antenna)

        for pair in antennas:
            if antenna == pair or antennas[antenna] != antennas[pair]:
                continue

            dx, dy, antinode1, antinode2 = calculate_distances_and_create_antinodes(antenna, pair)

            while 0 <= antinode1[0] < width and 0 <= antinode1[1] < height:
                antinodes.add(antinode1)
                antinode1 = (antinode1[0] - dx, antinode1[1] - dy)

            while 0 <= antinode2[0] < width and 0 <= antinode2[1] < height:
                antinodes.add(antinode2)
                antinode2 = (antinode2[0] + dx, antinode2[1] + dy)

    return len(antinodes)


def main():
    with open("demo-input.txt") as file:
        demo_input = file.read()

    with open("input.txt") as file:
        real_input = file.read()

    print("demo-input:")
    print(first_part(demo_input))
    print(second_part(demo_input))

    print("\ninput:")
    print(first_part(real_input))
    print(second_part(real_input))


if __name__ == '__main__':
    main()
