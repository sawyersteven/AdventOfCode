using AdventOfCode;
using System.Collections.Generic;
using System;

namespace Advent2020
{
    public class Challenge10 : Challenge
    {
        const int connectRange = 3;
        int[] connections;

        public override object Task1()
        {
            int[] convertedInput = Array.ConvertAll(input, int.Parse);
            Array.Sort(convertedInput);

            connections = new int[convertedInput.Length + 2];
            Array.Copy(convertedInput, 0, connections, 1, convertedInput.Length);
            connections[^1] = convertedInput[^1] + 3;

            int current = connections[0];
            int ones = 0;
            int threes = 0;
            for (int i = 0; i < connections.Length; i++)
            {
                if (connections[i] - current == 1) ones++;
                if (connections[i] - current == 3) threes++;
                current = connections[i];
            }
            return threes * ones;
        }

        public override object Task2()
        {
            Dictionary<int, long> cache = new Dictionary<int, long>(connections.Length);
            foreach (int i in connections) cache.Add(i, 0);
            cache[0] = 1;

            for (int i = 0; i < connections.Length; i++)
            {
                int j = i + 1;
                while (j < connections.Length && connections[j] - connections[i] <= connectRange)
                {
                    cache[connections[j]] += cache[connections[i]];
                    j++;
                }
            }
            return cache[connections[^2]];
        }
    }
}