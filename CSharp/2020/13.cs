using AdventOfCode;
using System.Collections.Generic;

namespace Advent2020
{
    public class Challenge13 : Challenge
    {

        public override object Task1()
        {
            int timestamp = int.Parse(input[0]);
            int waitTime = int.MaxValue;
            int busID = 0;

            foreach (string part in input[1].Split(','))
            {
                if (part == "x") continue;
                int cycleTime = int.Parse(part);

                int wait = (((timestamp / cycleTime) + 1) * cycleTime) - timestamp;
                if (wait < waitTime)
                {
                    waitTime = wait;
                    busID = cycleTime;
                }
            }
            return waitTime * busID;
        }

        public override object Task2()
        {
            string[] parts = input[1].Split(',');
            List<Bus> busses = new List<Bus>();
            for (int i = 0; i < parts.Length; i++)
            {
                if (parts[i] == "x") continue;
                busses.Add(new Bus() { Offset = i, ID = int.Parse(parts[i]) });
            }

            long answer = busses[0].ID;
            long lcm = busses[0].ID;

            for (int i = 1; i < busses.Count; i++)
            {
                while ((answer + busses[i].Offset) % busses[i].ID != 0)
                {
                    answer += lcm;
                }
                lcm = LCM(lcm, busses[i].ID);
            }
            return answer;
        }

        private long LCM(long a, long b)
        {
            long big, small;
            if (a > b)
            {
                big = a; small = b;
            }
            else
            {
                big = b; small = a;
            }

            for (long i = 1; i <= small; i++)
            {
                if ((big * i) % small == 0)
                {
                    return i * big;
                }
            }
            return small;
        }

        private struct Bus
        {
            public int Offset;
            public int ID;
        }
    }
}