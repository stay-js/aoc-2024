using day_14;

const int TIME = 100;

const int DEMO_WIDTH = 11;
const int DEMO_HEIGHT = 7;

const int INPUT_WIDTH = 101;
const int INPUT_HEIGHT = 103;

string[] demoInput = await File.ReadAllLinesAsync("demo-input.txt");
string[] input = await File.ReadAllLinesAsync("input.txt");

Console.WriteLine("demo-input:");
Console.WriteLine(FirstPart(demoInput, DEMO_WIDTH, DEMO_HEIGHT, TIME));

Console.WriteLine("\ninput:");
Console.WriteLine(FirstPart(input, INPUT_WIDTH, INPUT_HEIGHT, TIME));

var (seconds, grid) = SecondPart(input, INPUT_WIDTH, INPUT_HEIGHT);
Console.WriteLine(seconds);

Console.WriteLine("\nEaster egg:");
PrintGrid(grid);

static int FirstPart(string[] lines, int width, int height, int time)
{
    var robots = lines.Select(x => new Robot(x)).ToList();

    robots.ForEach(x => x.UpdatePosition(time, width, height));

    int middleX = width / 2;
    int middleY = height / 2;

    int q1 = 0, q2 = 0, q3 = 0, q4 = 0;

    foreach (var robot in robots)
    {
        var (x, y) = robot.Position;

        if (x == middleX || y == middleY) continue;

        if (x < middleX && y < middleY) q1++;
        else if (x >= middleX && y < middleY) q2++;
        else if (x < middleX && y >= middleY) q3++;
        else if (x >= middleX && y >= middleY) q4++;
    }

    return q1 * q2 * q3 * q4;
}

static (int, int[,]) SecondPart(string[] lines, int width, int height)
{
    var robots = lines.Select(x => new Robot(x)).ToList();
    int seconds = 0;

    while (true)
    {
        int[,] grid = new int[height, width];
        bool overlap = false;

        foreach (var robot in robots)
        {
            var (x, y) = robot.PredictPosition(seconds, width, height);
            grid[y, x]++;

            if (grid[y, x] > 1)
            {
                overlap = true;
                break;
            }
        }

        if (!overlap) return (seconds, grid);

        seconds++;
    }
}

static void PrintGrid(int[,] grid)
{
    for (int i = 0; i < grid.GetLength(0); i++)
    {
        for (int j = 0; j < grid.GetLength(1); j++)
        {
            Console.Write(grid[i, j] > 0 ? "#" : ".");
        }

        Console.WriteLine();
    }
}
