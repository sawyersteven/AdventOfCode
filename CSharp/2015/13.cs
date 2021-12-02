using System.Collections.Generic;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge13 : Challenge
    {
        Dictionary<string, Dictionary<string, int>> Map = new Dictionary<string, Dictionary<string, int>>();
        HashSet<string> people = new HashSet<string>();

        public override void ParseInput()
        {
            foreach (string line in input)
            {
                string[] parts = line.Split(' ');
                if (!Map.ContainsKey(parts[0])) Map[parts[0]] = new Dictionary<string, int>();
                Map[parts[0]][parts[^1].TrimEnd('.')] = int.Parse(parts[3]) * (parts[2][0] == 'g' ? 1 : -1);
                people.Add(parts[0]);
            }
        }

        public override object Task1()
        {
            return FindBest();
        }

        private int FindBest()
        {
            int best = 0;
            foreach (IList<string> arrangement in Permutator.Permutate<string>(new List<string>(people)))
            {
                int total = 0;
                for (int i = 1; i < arrangement.Count - 1; i++)
                {
                    total += Map[arrangement[i]][arrangement[i - 1]];
                    total += Map[arrangement[i]][arrangement[i + 1]];
                }

                total += Map[arrangement[0]][arrangement[1]];
                total += Map[arrangement[0]][arrangement[^1]];

                total += Map[arrangement[^1]][arrangement[0]];
                total += Map[arrangement[^1]][arrangement[^2]];

                if (total > best) best = total;
            }
            return best;
        }

        public override object Task2()
        {
            Map["me"] = new Dictionary<string, int>();
            foreach (string person in people)
            {
                Map["me"][person] = 0;
                Map[person]["me"] = 0;
            }

            people.Add("me");
            return FindBest();
        }
    }
}
