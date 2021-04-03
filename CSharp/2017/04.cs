using System;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge04 : Challenge
    {
        public override object Task1()
        {
            int count = 0;

            foreach (string line in input)
            {
                string[] words = line.Split(' ');
                bool isValid = true;
                for (int i = 0; i < words.Length - 1; i++)
                {
                    for (int j = i + 1; j < words.Length; j++)
                    {
                        if (words[i] == words[j])
                        {
                            isValid = false;
                            break;
                        }
                    }
                    if (!isValid) break;
                }
                if (isValid) count++;
            }
            return count;
        }

        private bool IsAnagram(string a, string b)
        {
            if (a.Length != b.Length) return false;
            char[] ca = a.ToCharArray();
            Array.Sort(ca);
            char[] cb = b.ToCharArray();
            Array.Sort(cb);
            for (int i = 0; i < ca.Length; i++)
            {
                if (ca[i] != cb[i]) return false;
            }
            return true;
        }

        public override object Task2()
        {
            int count = 0;

            foreach (string line in input)
            {
                string[] words = line.Split(' ');
                bool isValid = true;
                for (int i = 0; i < words.Length - 1; i++)
                {
                    for (int j = i + 1; j < words.Length; j++)
                    {
                        if (IsAnagram(words[i], words[j]))
                        {
                            isValid = false;
                            break;
                        }
                    }
                    if (!isValid) break;
                }
                if (isValid) count++;
            }
            return count;
        }
    }
}
