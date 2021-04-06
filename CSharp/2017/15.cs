using System.Collections.Generic;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge15 : Challenge
    {
        private const long GenAFactor = 16807;
        private const long GenBFactor = 48271;
        private const long ModFactor = 2147483647;

        /* there is undoubtedly a smarter way to handle this than actually
        iterating 40M times. This runs in ~750ms for pt1 and ~1400 for pt2,
        so it isn't *terrible* considering the sheer number of calcs.
        I'll try to get back to this later.
        */
        public override object Task1()
        {
            long GenA = int.Parse(input[0].Split(' ')[^1]);
            long GenB = int.Parse(input[1].Split(' ')[^1]);

            int total = 0;
            for (int i = 1; i < 40_000_000; i++)
            {
                GenA = (GenA * GenAFactor) % ModFactor;
                GenB = (GenB * GenBFactor) % ModFactor;
                if ((GenA & ushort.MaxValue) == (GenB & ushort.MaxValue)) total++;
            }

            return total;
        }

        public override object Task2()
        {
            long GenA = int.Parse(input[0].Split(' ')[^1]);
            long GenB = int.Parse(input[1].Split(' ')[^1]);

            Queue<long> AQueue = new Queue<long>();

            while (AQueue.Count < 5_000_000)
            {
                GenA = (GenA * GenAFactor) % ModFactor;
                if (GenA % 4 == 0) AQueue.Enqueue(GenA & ushort.MaxValue);
            }

            int total = 0;
            for (int i = 0; i < 5_000_000;)
            {
                GenB = (GenB * GenBFactor) % ModFactor;
                if (GenB % 8 == 0)
                {
                    i++;
                    if (AQueue.Dequeue() == (GenB & ushort.MaxValue)) total++;
                }
            }

            return total;
        }
    }
}
