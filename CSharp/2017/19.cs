using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2017
{
    public class Challenge19 : Challenge
    {
        private int steps = 0;
        public override object Task1()
        {
            char[,] grid = Utils.InputToCharArray(input, addBorder: true, defaultChar: ' ');

            Vector2Int currentPosition = new Vector2Int(1, 1);
            for (int i = 0; i < input[0].Length; i++)
            {
                if (grid[1, i] == '|')
                {
                    currentPosition.x = i;
                    break;
                }
            }

            List<char> collectedLetters = new List<char>();
            Vector2Int currentDirection = Vector2Int.Up;

            while (true)
            {
                Vector2Int next = currentPosition + currentDirection;
                while (grid[next.y, next.x] != ' ')
                {
                    char n = grid[next.y, next.x];
                    if (n >= 'A' && n <= 'Z') collectedLetters.Add(n);
                    currentPosition = next;
                    steps++;
                    next = currentPosition + currentDirection;
                }

                Vector2Int nextDirection = currentDirection;
                foreach (Vector2Int dir in Vector2Int.CardinalDirections)
                {
                    if (dir == currentDirection * -1) continue;
                    next = currentPosition + dir;
                    if (grid[next.y, next.x] != ' ')
                    {
                        nextDirection = dir;
                        break;
                    }
                }

                if (nextDirection == currentDirection) break;
                currentDirection = nextDirection;
            }

            return string.Join("", collectedLetters);
        }

        public override object Task2()
        {
            return steps + 1;
        }
    }
}
