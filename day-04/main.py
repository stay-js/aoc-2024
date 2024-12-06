def first_part(string: str) -> int:
    TARGET_STRINGS = ["XMAS", "SAMX"]

    data = string.split('\n')

    height = len(data)
    width = len(data[0])

    count = 0

    for i in range(height):
        for j in range(width - 3):
            if data[i][j:j+4] in TARGET_STRINGS:
                count += 1

    for i in range(height - 3):
        for j in range(width):
            if "".join([data[i + x][j] for x in range(4)]) in TARGET_STRINGS:
                count += 1

    for i in range(height - 3):
        for j in range(width - 3):
            if "".join([data[i + x][j + x] for x in range(4)]) in TARGET_STRINGS:
                count += 1

    for i in range(3, height):
        for j in range(width - 3):
            if "".join([data[i - x][j + x] for x in range(4)]) in TARGET_STRINGS:
                count += 1

    return count


def second_part(string: str) -> int:
    TARGET_STRINGS = ["MAS", "SAM"]

    count = 0

    data = string.split('\n')

    height = len(data)
    width = len(data[0])

    for i in range(1, height-1):
        for j in range(1, width-1):
            if data[i][j] == 'A' \
                    and data[i-1][j-1] + 'A' + data[i+1][j+1] in TARGET_STRINGS \
                    and data[i-1][j+1] + 'A' + data[i+1][j-1] in TARGET_STRINGS:
                count += 1

    return count


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
