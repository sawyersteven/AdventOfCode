using System;
using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2021
{
    public class Challenge13 : Challenge
    {
        private const char dot = '█';
        private const char blank = ' ';

        HashSet<Vector2Int> dots;
        (bool, int)[] folds;
        public override void ParseInput()
        {
            dots = new HashSet<Vector2Int>();

            int i = 0;
            for (i = 0; i < input.Length; i++)
            {
                if (string.IsNullOrWhiteSpace(input[i])) break;
                int[] nums = Array.ConvertAll(input[i].Split(','), int.Parse);
                dots.Add(new Vector2Int(nums[0], nums[1]));
            }

            i++;
            folds = new (bool, int)[input.Length - i]; // X = true, Y = false
            for (int j = 0; j + i < input.Length; j++)
            {
                string[] parts = input[i + j].Split(' ')[^1].Split('=');
                folds[j] = (parts[0][0] == 'x', int.Parse(parts[1]));
            }
        }

        public override object Task1()
        {
            HashSet<Vector2Int> p = new HashSet<Vector2Int>(dots);
            p = folds[0].Item1 ? FoldX(p, folds[0].Item2) : FoldY(p, folds[0].Item2);
            return p.Count;
        }

        private HashSet<Vector2Int> FoldY(HashSet<Vector2Int> dots, int line)
        {
            HashSet<Vector2Int> foldedDots = new HashSet<Vector2Int>();
            foreach (Vector2Int v in dots)
            {
                if (v.y > line) foldedDots.Add(new Vector2Int(v.x, (line * 2) - v.y));
                else foldedDots.Add(v);
            }
            return foldedDots;
        }

        private HashSet<Vector2Int> FoldX(HashSet<Vector2Int> dots, int line)
        {
            HashSet<Vector2Int> foldedDots = new HashSet<Vector2Int>();
            foreach (Vector2Int v in dots)
            {
                if (v.x > line) foldedDots.Add(new Vector2Int((line * 2) - v.x, v.y));
                else foldedDots.Add(v);
            }
            return foldedDots;
        }

        public override object Task2()
        {
            HashSet<Vector2Int> p = new HashSet<Vector2Int>(dots);
            foreach ((bool dir, int line) in folds)
            {
                p = dir ? FoldX(p, line) : FoldY(p, line);
            }

            return '\n' + DrawPoints(p);
        }

        private string DrawPoints(HashSet<Vector2Int> dots)
        {
            int maxX = 0;
            int maxY = 0;
            foreach (Vector2Int v in dots)
            {
                maxX = Math.Max(maxX, v.x);
                maxY = Math.Max(maxY, v.y);
            }

            char[,] code = new char[maxY + 1, maxX + 1];
            code.Fill(blank);
            foreach (Vector2Int v in dots)
            {
                code[v.y, v.x] = dot;
            }
            return code.Render();
        }
    }
}
