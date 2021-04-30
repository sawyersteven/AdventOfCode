using System.Collections.Generic;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge09 : Challenge
    {
        Dictionary<string, List<(string, int)>> Map = new Dictionary<string, List<(string, int)>>();
        HashSet<string> cities = new HashSet<string>();

        private void ParseInput()
        {
            foreach (string line in input)
            {
                string[] parts = line.Split(' ');
                if (!Map.ContainsKey(parts[0])) Map[parts[0]] = new List<(string, int)>();
                if (!Map.ContainsKey(parts[2])) Map[parts[2]] = new List<(string, int)>();
                Map[parts[0]].Add((parts[2], int.Parse(parts[4])));
                Map[parts[2]].Add((parts[0], int.Parse(parts[4])));
                cities.Add(parts[0]);
                cities.Add(parts[2]);
            }
        }

        int longest = 0;
        public override object Task1()
        {
            ParseInput();

            int shortest = int.MaxValue;
            foreach (IList<string> path in Permutator.Permutate(new List<string>(cities)))
            {
                int dist = 0;
                for (int i = 0; i < path.Count - 1; i++)
                {
                    if (!Map.ContainsKey(path[i])) continue;
                    foreach ((string city, int d) in Map[path[i]])
                    {
                        if (city == path[i + 1])
                        {
                            dist += d;
                            break;
                        }
                    }
                }
                if (dist > longest) longest = dist;
                if (dist < shortest) shortest = dist;
            }
            return shortest;
        }

        public override object Task2()
        {
            return longest;
        }
    }
}
