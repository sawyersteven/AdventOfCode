using AdventOfCode;
using System;
using System.Collections.Generic;
using Grids;

namespace Advent2019
{
    public class Challenge15 : Challenge
    {
        private const int N = 1;
        private const int S = 2;
        private const int W = 3;
        private const int E = 4;

        private const char wall = '█';
        private const char empty = '.';
        private const char air = 'A';

        private const int gridSize = 50;

        private int CCW(int d)
        {
            return d switch
            {
                1 => 3,
                2 => 4,
                3 => 2,
                4 => 1,
                _ => throw new ArgumentException()
            };
        }

        private int CW(int d)
        {
            return d switch
            {
                1 => 4,
                2 => 3,
                3 => 1,
                4 => 2,
                _ => throw new ArgumentException()
            };
        }

        char[,] grid;
        Vector2Int[] dirs = new Vector2Int[] { Vector2Int.Up, Vector2Int.Down, Vector2Int.Left, Vector2Int.Right };
        Vector2Int endPosition;
        public override object Task1()
        {
            IntCode.Emulator ICE = new IntCode.Emulator();
            var response = ICE.Boot(IntCode.Tools.ParseCode(input[0]));

            grid = new char[gridSize, gridSize];
            Vector2Int currentPos = new Vector2Int(gridSize / 2, gridSize / 2);
            Vector2Int startPos = currentPos;
            int currentDir = W;

            HashSet<Vector2Int> walls = new HashSet<Vector2Int>();

            int t = 0;
            int changes = 0;
            while (true)
            {
                Vector2Int rightHand = currentPos + dirs[CCW(currentDir) - 1];
                if (grid[rightHand.y, rightHand.x] != wall)
                {
                    currentDir = CCW(currentDir);
                }

                Vector2Int nextPos = currentPos + dirs[currentDir - 1];

                if (nextPos.x == -1 || nextPos.x >= gridSize || nextPos.y == -1 || nextPos.y >= gridSize)
                {
                    currentDir = CW(CW(currentDir));
                    continue;
                }
                ICE.QueueInput(currentDir);
                response = ICE.Run();
                switch (response.Item2)
                {
                    case 0:
                        changes += grid[nextPos.y, nextPos.x] == wall ? 0 : 1;
                        walls.Add(new Vector2Int(nextPos.x, nextPos.y));
                        grid[nextPos.y, nextPos.x] = wall;
                        currentDir = CW(currentDir);
                        break;
                    case 1:
                        changes += grid[nextPos.y, nextPos.x] == empty ? 0 : 1;
                        grid[currentPos.y, currentPos.x] = grid[currentPos.y, currentPos.x] == air ? air : empty;
                        currentPos = nextPos;
                        break;
                    case 2:
                        grid[nextPos.y, nextPos.x] = air;
                        endPosition = nextPos;
                        currentPos = nextPos;
                        break;
                }
                t++;
                if (t % gridSize == 0)
                {
                    if (changes == 0) break;
                    changes = 0;
                }
            }

            AStar a = new AStar();
            return a.FindPath(startPos, endPosition, walls).Length;
        }


        public override object Task2()
        {
            int minutes = 0;

            Queue<Vector2Int> Edges = new Queue<Vector2Int>();
            Queue<Vector2Int> newEdges = new Queue<Vector2Int>();
            Edges.Enqueue(endPosition);

            while (true)
            {
                while (Edges.TryDequeue(out Vector2Int point))
                {
                    foreach (Vector2Int adjacent in AdjacentTiles(point))
                    {
                        if (grid[adjacent.y, adjacent.x] == empty)
                        {
                            grid[adjacent.y, adjacent.x] = air;
                            newEdges.Enqueue(adjacent);
                        }
                    }
                }
                if (newEdges.Count == 0) break;
                while (newEdges.TryDequeue(out Vector2Int point))
                {
                    Edges.Enqueue(point);
                }

                minutes++;
            }

            return minutes;
        }

        private Vector2Int[] AdjacentTiles(Vector2Int origin)
        {
            Vector2Int[] adj = new Vector2Int[4];
            for (int i = 0; i < 4; i++)
            {
                adj[i] = origin + dirs[i];
            }
            return adj;
        }
    }
}