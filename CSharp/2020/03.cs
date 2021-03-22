using AdventOfCode;
using System;
using System.Numerics;

namespace Advent2020
{
    public class Challenge03 : Challenge
    {

        public override object Task1()
        {
            return FindTrees(new Vector2(3, 1));
        }

        private int FindTrees(Vector2 direction)
        {
            int trees = 0;
            Vector2 current = new Vector2(0, 0);
            while (current.Y < input.Length - 1)
            {
                current += direction;
                if (current.X >= input[0].Length)
                {
                    current.X %= input[0].Length;
                }
                if (input[(int)current.Y][(int)current.X] == '#') trees++;
            }
            return trees;
        }

        public override object Task2()
        {
            uint t = (uint)FindTrees(new Vector2(1, 1));
            t *= (uint)FindTrees(new Vector2(3, 1));
            t *= (uint)FindTrees(new Vector2(5, 1));
            t *= (uint)FindTrees(new Vector2(7, 1));
            t *= (uint)FindTrees(new Vector2(1, 2));

            return t;
        }
    }
}