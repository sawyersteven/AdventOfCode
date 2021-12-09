using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2021
{
    public class Challenge08 : Challenge
    {
        public override object Task1()
        {
            int count = 0;
            foreach (string line in input)
            {
                string[] outVals = line.Split(" | ")[1].Split(' ');
                foreach (string o in outVals)
                {
                    int l = o.Length;
                    if (l == 2 || l == 4 || l == 3 || l == 7)
                    {
                        count++;
                    }
                }
            }
            return count;
        }

        // I thought this would be much slower than it is. Strings are actually faster than hashsets...
        public override object Task2()
        {
            int sum = 0;
            string[] orderedSignals = new string[10];
            foreach (string line in input)
            {
                string[] parts = line.Split(" | ");
                string[] inSignals = Array.ConvertAll<string, string>(parts[0].Split(' '), Alphabetize);
                string[] outSignals = Array.ConvertAll<string, string>(parts[1].Split(' '), Alphabetize);

                // find 1,4,7,8
                foreach (string s in inSignals)
                {
                    if (s.Length == 2) orderedSignals[1] = s;
                    else if (s.Length == 4) orderedSignals[4] = s;
                    else if (s.Length == 3) orderedSignals[7] = s;
                    else if (s.Length == 7) orderedSignals[8] = s;
                }

                // find 0,6,9
                foreach (string s in inSignals)
                {
                    if (s.Length != 6) continue;
                    if (!IsSuperSet(s, orderedSignals[7])) orderedSignals[6] = s;
                    else if (IsSuperSet(s, orderedSignals[4])) orderedSignals[9] = s;
                    else orderedSignals[0] = s;
                }

                // find 2,3,5
                foreach (string s in inSignals)
                {
                    if (s.Length != 5) continue;
                    if (IsSuperSet(s, orderedSignals[1])) orderedSignals[3] = s;
                    else if (IsSuperSet(orderedSignals[9], s)) orderedSignals[5] = s;
                    else orderedSignals[2] = s;
                }

                for (int i = 0; i < 4; i++)
                {
                    for (int n = 0; n < orderedSignals.Length; n++)
                    {
                        if (orderedSignals[n] == outSignals[i])
                        {
                            sum += n * ((int)Math.Pow(10, 3 - i));
                            break;
                        }
                    }
                }
            }
            return sum;
        }

        private bool IsSuperSet(string super, string sub)
        {
            foreach (char a in sub)
            {
                if (!super.Contains(a)) return false;
            }
            return true;
        }

        private string Alphabetize(string s)
        {
            char[] ca = s.ToCharArray();
            Array.Sort(ca);
            return string.Join(null, ca);
        }
    }
}
