using System.Collections.Generic;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge07 : Challenge
    {
        public override object Task1()
        {
            int count = 0;
            char[] sep = new char[] { '[', ']' };
            foreach (string line in input)
            {
                string[] parts = line.Split(sep);
                bool inside = false;
                bool outside = false;
                for (int i = 0; i < parts.Length; i++)
                {
                    if (i % 2 == 0) outside |= HasABBA(parts[i]);
                    else inside |= HasABBA(parts[i]);
                }
                if (outside & !inside)
                {
                    count++;
                }
            }
            return count;
        }

        private bool HasABBA(string str)
        {
            for (int i = 0; i < str.Length - 3; i++)
            {
                if (str[i] == str[i + 3] && str[i + 1] == str[i + 2] && str[i] != str[i + 1]) return true;
            }
            return false;
        }

        private HashSet<string> FindABA(string str, bool invert = false)
        {
            HashSet<string> abas = new HashSet<string>();
            for (int i = 0; i < str.Length - 2; i++)
            {
                if (str[i] == str[i + 2] && str[i] != str[i + 1])
                {
                    if (!invert) abas.Add(str.Substring(i, 3));
                    else
                    {
                        char[] a = str.Substring(i, 3).ToCharArray();
                        a[0] = a[1];
                        a[1] = a[2];
                        a[2] = a[0];
                        abas.Add(string.Join("", a));
                    }
                }
            }
            return abas;
        }


        public override object Task2()
        {
            int count = 0;
            char[] sep = new char[] { '[', ']' };
            foreach (string line in input)
            {
                string[] parts = line.Split(sep);
                List<string> abas = new List<string>();
                HashSet<string> babs = new HashSet<string>();
                for (int i = 0; i < parts.Length; i += 2)
                {
                    abas.AddRange(FindABA(parts[i]));
                }
                for (int i = 1; i < parts.Length; i += 2)
                {
                    foreach (string bab in FindABA(parts[i], true))
                    {
                        if (abas.Contains(bab))
                        {
                            goto foundMatch;
                        }
                    }
                }
                continue;

            foundMatch:
                count++;
            }
            return count;
        }

    }
}
