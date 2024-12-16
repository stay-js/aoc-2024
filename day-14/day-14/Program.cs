using day_14;

const int TIME = 100;

const int DEMO_WIDTH = 11;
const int DEMO_HEIGHT = 7;

const int INPUT_WIDTH = 101;
const int INPUT_HEIGHT = 103;

string demoInput = await File.ReadAllTextAsync("demo-input.txt");
string input = await File.ReadAllTextAsync("input.txt");

Console.WriteLine("demo-input:");
Console.WriteLine(FirstPart(demoInput, DEMO_WIDTH, DEMO_HEIGHT, TIME));

Console.WriteLine("\ninput:");
Console.WriteLine(FirstPart(input, INPUT_WIDTH, INPUT_HEIGHT, TIME));

static int FirstPart(string input, int width, int height, int time)
{
    var robots = input.Split('\n').Select(x => new Robot(x)).ToList();

    for (int _ = 0; _ < time; _++)
    {
        robots.ForEach(x => x.UpdatePosition(width, height));
    }

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
