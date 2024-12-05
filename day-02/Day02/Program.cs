string[] demoInput = await File.ReadAllLinesAsync("demo-input.txt");
string[] input = await File.ReadAllLinesAsync("input.txt");

const int MIN_DIFF = 1;
const int MAX_DIFF = 3;

Console.WriteLine("demo-input:");
Console.WriteLine(CountSafeMeasurements(demoInput));


Console.WriteLine("\ninput:");
Console.WriteLine(CountSafeMeasurements(input));

static int CountSafeMeasurements(IEnumerable<string> lines) => lines.Sum(line =>
{
    int[] numbers = line.Split().Select(int.Parse).ToArray();
    if (IsSafe(numbers, true) || IsSafe(numbers, false)) return 1;
    return 0;
});

static bool IsSafe(int[] numbers, bool increasing)
{
    for (int i = 1; i < numbers.Length; i++)
    {
        int diff = Math.Abs(numbers[i] - numbers[i - 1]);
        if (diff < MIN_DIFF || diff > MAX_DIFF) return false;
        if (increasing && numbers[i] < numbers[i - 1]) return false;
        if (!increasing && numbers[i] > numbers[i - 1]) return false;
    }

    return true;
}
