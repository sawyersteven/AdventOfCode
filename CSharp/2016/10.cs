using System.Collections.Generic;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge10 : Challenge
    {
        private Dictionary<int, List<int>> bots = new Dictionary<int, List<int>>();

        public override object Task1()
        {
            Queue<int> actionQueue = new Queue<int>();

            Dictionary<int, (int, int)> swapRules = new Dictionary<int, (int, int)>();
            foreach (string line in input)
            {
                string[] parts = line.Split(' ');
                int p1 = int.Parse(parts[1]);
                if (parts[0] == "bot")
                {
                    int lowTarget = int.Parse(parts[6]);
                    // prevents overlap of bot 0 and output 0, but not bot 0 and output 100
                    // because there aren't that many outputs
                    if (parts[5] == "output")
                    {
                        lowTarget -= 100;
                    }
                    swapRules[p1] = (lowTarget, int.Parse(parts[^1]));
                }
                else
                {
                    int p2 = int.Parse(parts[^1]);
                    if (!bots.ContainsKey(p2)) bots[p2] = new List<int>(2);
                    bots[p2].Add(p1);
                    if (bots[p2].Count == 2) actionQueue.Enqueue(p2);
                }
            }

            int answer = 0;
            while (actionQueue.Count > 0)
            {
                int bot = actionQueue.Dequeue();
                bots[bot].Sort();
                if (bots[bot][0] == 17 && bots[bot][1] == 61)
                {
                    answer = bot;
                }

                (int lowTarget, int highTarget) = swapRules[bot];

                if (!bots.ContainsKey(lowTarget)) bots[lowTarget] = new List<int>(2);
                bots[lowTarget].Add(bots[bot][0]);
                if (bots[lowTarget].Count == 2) actionQueue.Enqueue(lowTarget);

                if (!bots.ContainsKey(highTarget)) bots[highTarget] = new List<int>(2);
                bots[highTarget].Add(bots[bot][1]);
                if (bots[highTarget].Count == 2) actionQueue.Enqueue(highTarget);

                bots[bot].Clear();
            }

            return answer;
        }

        public override object Task2()
        {
            return bots[-100][0] * bots[-99][0] * bots[-98][0];
        }
    }
}
