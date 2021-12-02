using AdventOfCode;
using Grids;

namespace Advent2021
{
    public class Challenge02 : Challenge
    {
        public override object Task1()
        {
            Vector2Int position = new Vector2Int(0, 0);

            foreach (string line in input)
            {
                string[] move = line.Split(' ');
                int dist = int.Parse(move[1]);
                switch (move[0])
                {
                    case "forward":
                        position.x += dist;
                        break;
                    case "down":
                        position.y += dist;
                        break;
                    case "up":
                        position.y -= dist;
                        break;
                }
            }

            return position.x * position.y;
        }

        public override object Task2()
        {
            Vector2Int position = new Vector2Int(0, 0);
            int aim = 0;

            foreach (string line in input)
            {
                string[] move = line.Split(' ');
                int dist = int.Parse(move[1]);
                switch (move[0])
                {
                    case "forward":
                        position.x += dist;
                        position.y += dist * aim;
                        break;
                    case "down":
                        aim += dist;
                        break;
                    case "up":
                        aim -= dist;
                        break;
                }
            }

            return position.x * position.y;
        }
    }
}
