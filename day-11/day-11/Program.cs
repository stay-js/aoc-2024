string demoInput = await File.ReadAllTextAsync("demo-input.txt");
string input = await File.ReadAllTextAsync("input.txt");

Console.WriteLine("demo-input:");
Console.WriteLine(CalculateStonesAfterNBlinks(demoInput, 25));

Console.WriteLine("\ninput:");
Console.WriteLine(CalculateStonesAfterNBlinks(input, 25));
Console.WriteLine(CalculateStonesAfterNBlinks(input, 75));

static long CalculateStonesAfterNBlinks(string input, int n)
{
    var stoneCounts = new Dictionary<string, long>();

    foreach (string stone in input.Split())
    {
        AddStone(stoneCounts, stone, 1);
    }

    for (int _ = 0; _ < n; _++)
    {
        var newStoneCounts = new Dictionary<string, long>();

        foreach (var (stone, count) in stoneCounts)
        {
            if (stone == "0")
            {
                AddStone(newStoneCounts, "1", count);
                continue;
            }

            if (stone.Length % 2 == 0)
            {
                int half = stone.Length / 2;
                AddStone(newStoneCounts, stone[..half], count);
                AddStone(newStoneCounts, long.Parse(stone[half..]).ToString(), count);
                continue;
            }

            string newStone = (long.Parse(stone) * 2024).ToString();
            AddStone(newStoneCounts, newStone, count);
        }

        stoneCounts = newStoneCounts;
    }

    return stoneCounts.Values.Sum();
}

static void AddStone(Dictionary<string, long> stoneCounts, string stone, long count)
{
    if (!stoneCounts.ContainsKey(stone)) stoneCounts[stone] = 0;
    stoneCounts[stone] += count;
}
