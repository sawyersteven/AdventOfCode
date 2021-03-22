using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge12 : Challenge
    {
        const int sliceWidth = 5;
        char[] pots;
        HashSet<string> patterns;
        private void ParseInput(int extraPots)
        {
            pots = new char[input[0].Length + (extraPots * 2)];
            for (int i = 0; i < pots.Length; i++) pots[i] = '.';

            string initState = input[0].Split(": ")[1];
            for (int i = 0; i < initState.Length; i++)
            {
                pots[extraPots + i] = initState[i];
            }

            patterns = new HashSet<string>();
            for (int i = 2; i < input.Length; i++)
            {
                string[] parts = input[i].Split(" => ");
                if (parts[1][0] == '.') continue;
                patterns.Add(parts[0]);
            }
        }

        public override object Task1()
        {
            ParseInput(15);

            RunGeneration(20);

            return CountPlants(15);
        }

        private int CountPlants(int extraPots)
        {
            int total = 0;
            for (int i = 0; i < pots.Length; i++)
            {
                if (pots[i] == '#')
                {
                    total += i - extraPots;
                }
            }
            return total;
        }

        private void RunGeneration(int iterations)
        {
            for (; iterations > 0; iterations--)
            {
                char[] nextGen = new char[pots.Length];

                for (int i = 0; i < nextGen.Length; i++) nextGen[i] = '.';

                string slice;
                for (int i = 2; i < pots.Length - 2; i++)
                {
                    slice = string.Join("", new ArraySegment<char>(pots, i - 2, sliceWidth));
                    if (patterns.Contains(slice))
                    {
                        nextGen[i] = '#';
                    }
                    else
                    {
                        nextGen[i] = '.';
                    }
                }
                Array.Copy(nextGen, pots, pots.Length);
            }
        }

        public override object Task2()
        {
            const int sampleRate = 10;
            const int sampleCount = 50;
            const int extraSpace = 500;

            ParseInput(extraSpace); // mostly arbitrary;

            List<int> results = new List<int>();

            for (int i = 0; i < sampleCount; i++)
            {
                RunGeneration(sampleRate);
                results.Add(CountPlants(extraSpace));
            }

            // Little bit of cheating -- the growth rate stabilizes after ~100 generations
            // take a few samples to see if there is a stable growth rate, then fast-forward time
            int stableGrowthRate = results[results.Count - 1] - results[results.Count - 2];
            for (int i = 0; i < 10; i++)
            {
                int sgr = results[results.Count - i - 1] - results[results.Count - i - 2];
                if (sgr != stableGrowthRate) return "Stable growth rate not reached -- increase simulation length";
            }

            int perSampleRate = stableGrowthRate / sampleRate;
            int simulationLen = sampleCount * sampleRate;

            return results[results.Count - 1] + ((50000000000 - simulationLen) * perSampleRate);
        }
    }
}