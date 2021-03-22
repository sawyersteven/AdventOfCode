using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2020
{
    public class Challenge05 : Challenge
    {

        private int GetRow(string code)
        {
            int l = 0;
            int u = 127;

            foreach (char c in code)
            {
                int half = (u + 1 - l) / 2;
                if (c == 'F') u -= half;
                else l += half;
            }
            if (u != l)
            {
                throw new System.Exception("Invlaid row bounds: {l}:{u} for code {code}");
            }
            return u;
        }

        private int GetSeat(string code)
        {
            int l = 0;
            int u = 7;

            foreach (char c in code)
            {
                int half = (u + 1 - l) / 2;
                if (c == 'L') u -= half;
                else l += half;
            }
            if (u != l)
            {
                throw new System.Exception("Invlaid seat bounds: {l}:{u} for code {code}");
            }
            return u;
        }

        public override object Task1()
        {
            int highID = 0;

            foreach (string line in input)
            {
                int row = GetRow(line.Substring(0, 7));
                int seat = GetSeat(line.Substring(7));

                int id = row * 8 + seat;

                if (id > highID) highID = id;
            }

            return highID;
        }

        public override object Task2()
        {
            List<int> ids = new List<int>();

            foreach (string line in input)
            {
                int row = GetRow(line.Substring(0, 7));
                int seat = GetSeat(line.Substring(7));

                ids.Add(row * 8 + seat);
            }

            ids.Sort();
            for (int i = 0; i < ids.Count - 1; i++)
            {
                if (ids[i] + 2 == ids[i + 1])
                {
                    return ids[i] + 1;
                }
            }

            return null;
        }
    }
}