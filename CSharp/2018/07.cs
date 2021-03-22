using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge07 : Challenge
    {

        public override object Task1()
        {
            SortedSet<char> onDeck = new SortedSet<char>();
            Dictionary<char, HashSet<char>> requirements = new Dictionary<char, HashSet<char>>();
            foreach (string line in input)
            {
                char prereq = line[36];
                char step = line[5];
                if (!requirements.ContainsKey(prereq)) requirements[prereq] = new HashSet<char>();
                requirements[prereq].Add(line[5]);
                onDeck.Add(step);
            }

            foreach (char c in requirements.Keys)
            {
                if (requirements.ContainsKey(c)) onDeck.Remove(c);
            }

            List<char> complete = new List<char>(requirements.Count);
            while (onDeck.Count > 0)
            {
                char current = onDeck.Min;
                onDeck.Remove(current);

                foreach (var kv in requirements)
                {
                    kv.Value.Remove(current);
                    if (kv.Value.Count == 0)
                    {
                        onDeck.Add(kv.Key);
                        requirements.Remove(kv.Key);
                    }
                }
                complete.Add(current);
            }
            return string.Join("", complete);
        }

        private struct Worker : IComparable
        {
            public char Task;
            public int Time;

            public int CompareTo(object obj)
            {
                Worker other = (Worker)obj;
                if (other.Time == this.Time) return 0;
                return (other.Time > Time) ? -1 : 1;
            }
        }

        public override object Task2()
        {
            Dictionary<char, HashSet<char>> requirements = new Dictionary<char, HashSet<char>>();
            foreach (string line in input)
            {
                char prereq = line[36];
                char step = line[5];
                if (!requirements.ContainsKey(prereq)) requirements[prereq] = new HashSet<char>();
                if (!requirements.ContainsKey(step)) requirements[step] = new HashSet<char>();
                requirements[prereq].Add(line[5]);
            }

            SortedSet<char> onDeck = new SortedSet<char>();
            foreach (char c in requirements.Keys)
            {
                if (requirements[c].Count == 0) onDeck.Add(c);
            }

            int time = 0;
            const int baseTime = 60;

            Worker[] workers = new Worker[5];
            while (true)
            {
                int timeReduction = 0;
                if (onDeck.Count == 0)
                {
                    timeReduction = int.MaxValue;
                    foreach (Worker w in workers)
                    {
                        if (w.Time <= 0) continue;
                        timeReduction = w.Time < timeReduction ? w.Time : timeReduction;
                    }
                }
                else
                {
                    timeReduction = Utils.MinVal(workers).Time;
                    if (timeReduction < 0) timeReduction = 0;
                }

                time += timeReduction;

                bool stillWorking = false;
                for (int i = 0; i < workers.Length; i++)
                {
                    workers[i].Time -= timeReduction;
                    if (workers[i].Time <= 0)
                    {
                        foreach (var kv in requirements)
                        {
                            kv.Value.Remove(workers[i].Task);
                            if (kv.Value.Count == 0)
                            {
                                onDeck.Add(kv.Key);
                                requirements.Remove(kv.Key);
                            }
                        }
                        workers[i].Task = onDeck.Min;
                        workers[i].Time = baseTime + (workers[i].Task - 'A' + 1);
                        onDeck.Remove(workers[i].Task);
                    }
                    if (workers[i].Time > 0) stillWorking = true;
                }

                if (onDeck.Count == 0 && !stillWorking) break;
            }
            return time;
        }
    }
}