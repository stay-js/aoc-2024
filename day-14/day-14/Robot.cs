using Position = (int x, int y);

namespace day_14
{
    public class Robot
    {
        public Position Position { get; set; }
        public Position Velocity { get; set; }

        public Robot(string line)
        {
            string[] parts = line.Split(' ');

            Position = ParsePosition(parts[0]);
            Velocity = ParsePosition(parts[1]);
        }

        public void UpdatePosition(int width, int height)
        {
            Position = (
                (Position.x + Velocity.x + width) % width,
                (Position.y + Velocity.y + height) % height
            );
        }

        private static Position ParsePosition(string input)
        {
            int[] position = input.Split('=')[1].Split(',').Select(int.Parse).ToArray();

            return (position[0], position[1]);
        }
    }
}
