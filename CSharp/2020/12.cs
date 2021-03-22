using AdventOfCode;
using System;
using System.Collections.Generic;
using System.Numerics;

namespace Advent2020
{
    public class Challenge12 : Challenge
    {
        enum Direction
        {
            North,
            East,
            South,
            West
        }

        Dictionary<Direction, char> dirChars = new Dictionary<Direction, char>() {
            { Direction.North, 'N' },
            { Direction.East, 'E' },
            { Direction.South, 'S' },
            { Direction.West, 'W' }
            };

        public override object Task1()
        {
            Direction currentDir = Direction.East;
            Vector2 currentPos = new Vector2(0, 0);

            Vector2 pos;
            int dir;

            foreach (string line in input)
            {
                if (line[0] == 'F')
                {
                    (pos, dir) = GetPosDeltaT1(dirChars[currentDir], int.Parse(line.Substring(1)));
                }
                else
                {
                    (pos, dir) = GetPosDeltaT1(line[0], int.Parse(line.Substring(1)));
                }
                currentDir = (Direction)(((int)currentDir + dir + 4) % 4);
                currentPos += pos;
            }
            return Math.Abs(currentPos.X) + Math.Abs(currentPos.Y);
        }

        private ValueTuple<Vector2, int> GetPosDeltaT1(char command, int value)
        {
            Vector2 pos = new Vector2(0, 0);
            int dirChange = 0;
            switch (command)
            {
                case 'N':
                    pos.Y += value;
                    break;
                case 'S':
                    pos.Y -= value;
                    break;
                case 'E':
                    pos.X += value;
                    break;
                case 'W':
                    pos.X -= value;
                    break;
                case 'L':
                    dirChange = -1 * value / 90;
                    break;
                case 'R':
                    dirChange = 1 * value / 90;
                    break;
            }
            return new ValueTuple<Vector2, int>(pos, dirChange);
        }

        public override object Task2()
        {
            Vector2 currentPos = new Vector2(0, 0);
            Vector2 waypointOffset = new Vector2(10, 1);

            foreach (string line in input)
            {
                int value = int.Parse(line.Substring(1));
                switch (line[0])
                {
                    case 'F':
                        currentPos += value * waypointOffset;
                        break;
                    case 'N':
                        waypointOffset.Y += value;
                        break;
                    case 'E':
                        waypointOffset.X += value;
                        break;
                    case 'S':
                        waypointOffset.Y -= value;
                        break;
                    case 'W':
                        waypointOffset.X -= value;
                        break;
                    case 'L':
                        waypointOffset = RotateV2(waypointOffset, -1 * value / 90);
                        break;
                    case 'R':
                        waypointOffset = RotateV2(waypointOffset, value / 90);
                        break;
                }
            }
            return Math.Abs(currentPos.X) + Math.Abs(currentPos.Y);
        }

        private Vector2 RotateV2(Vector2 v2, int steps)
        {
            if (steps > 0)
            {
                for (int i = 0; i < steps; i++)
                {
                    v2 = new Vector2(v2.Y, -v2.X);
                }
            }
            else
            {
                for (int i = 0; i > steps; i--)
                {
                    v2 = new Vector2(-v2.Y, v2.X);
                }
            }
            return v2;
        }
    }
}