using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge04 : Challenge
    {

        public override object Task1()
        {
            int sum = 0;
            foreach (string line in input)
            {
                if (IsValid(line))
                {
                    sum += int.Parse(line.Split('[')[0].Split('-')[^1]);
                }
            }

            return sum;
        }

        private bool IsValid(string roomCode)
        {
            int split = roomCode.LastIndexOf('-');
            char[] letters = roomCode.Substring(0, split).Replace("-", "").ToCharArray();
            Array.Sort(letters);
            List<(char, int)> counts = new List<(char, int)>();

            (char, int) counter = (letters[0], 1);
            for (int i = 1; i < letters.Length; i++)
            {
                if (letters[i] != counter.Item1)
                {
                    counts.Add(counter);
                    counter.Item1 = letters[i];
                    counter.Item2 = 1;
                }
                else
                {
                    counter.Item2++;
                }
            }
            counts.Add(counter);

            counts.Sort((a, b) =>
            {
                if (a.Item2 == b.Item2) return a.Item1 - b.Item1;
                return b.Item2 - a.Item2;
            });
            string checksum = roomCode.Split('[')[1];

            for (int i = 0; i < 5; i++)
            {
                if (counts[i].Item1 != checksum[i]) return false;
            }
            return true;
        }

        public override object Task2()
        {
            foreach (string line in input)
            {
                int roomID = int.Parse(line.Split('[')[0].Split('-')[^1]);
                char[] letters = line.ToCharArray();
                for (int i = 0; i < letters.Length; i++)
                {
                    if (letters[i] == '-')
                    {
                        letters[i] = ' ';
                        continue;
                    }
                    letters[i] = (char)(letters[i] + (roomID % 26));
                    if (letters[i] > 'z') letters[i] -= (char)26;
                }
                if (letters[0] == 'n' && letters[1] == 'o')
                {
                    return roomID;
                }
            }

            return null;
        }
    }
}
