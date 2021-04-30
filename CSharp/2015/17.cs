using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge17 : Challenge
    {
        public override object Task1()
        {
            int[] containers = Array.ConvertAll(input, int.Parse);
            Array.Sort(containers);

            return CountCombos(containers, 150);
        }

        Dictionary<int, int> sizeCounts = new Dictionary<int, int>();
        private int CountCombos(int[] numbers, int targetSum, IList<int> _partial = null)
        {
            if (_partial == null) _partial = new int[0];
            int s = Sum(_partial);
            if (s == targetSum)
            {
                int l = _partial.Count;
                if (!sizeCounts.ContainsKey(l)) sizeCounts[l] = 0;
                sizeCounts[l]++;
                return 1;
            }
            if (s > targetSum)
            {
                return 0;
            }

            int t = 0;
            for (int i = 0; i < numbers.Length; i++)
            {
                int n = numbers[i];
                ArraySegment<int> remaining = new ArraySegment<int>(numbers, i + 1, numbers.Length - (i + 1));
                var p = new List<int>(_partial);
                p.Add(n);
                t += CountCombos(remaining.ToArray(), targetSum, p);
            }
            return t;

        }

        private int Sum(IList<int> arr)
        {
            int t = 0;
            foreach (int i in arr) t += i;
            return t;
        }

        public override object Task2()
        {
            int smallestSize = int.MaxValue;
            foreach (int key in sizeCounts.Keys)
            {
                if (key < smallestSize) smallestSize = key;
            }

            return sizeCounts[smallestSize];
        }
    }
}
