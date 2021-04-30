using System;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge14 : Challenge
    {
        const int time = 2503;

        // speed, runtime, resttime
        (int, int, int)[] rates;
        private void ParseInput()
        {
            rates = new (int, int, int)[input.Length];

            for (int i = 0; i < rates.Length; i++)
            {
                string[] parts = input[i].Split(' ');
                rates[i] = (int.Parse(parts[3]), int.Parse(parts[6]), int.Parse(parts[13]));
            }
        }

        public override object Task1()
        {
            ParseInput();

            int best = 0;
            foreach ((int spd, int runtime, int resttime) in rates)
            {
                int remaining = time;
                int dist = 0;
                while (true)
                {
                    if (remaining > runtime) dist += spd * runtime;
                    else if (remaining > 0)
                    {
                        dist += spd * remaining;
                        break;
                    }
                    else
                    {
                        break;
                    }
                    remaining -= runtime;
                    remaining -= resttime;
                }
                if (dist > best) best = dist;

            }

            return best;
        }

        public override object Task2()
        {
            int[] locations = new int[rates.Length];
            int[] points = new int[rates.Length];

            for (int second = 0; second < time; second++)
            {
                for (int i = 0; i < rates.Length; i++)
                {
                    (int spd, int runtime, int resttime) = rates[i];
                    if (second % (runtime + resttime) < runtime)
                    {
                        locations[i] += spd;
                    }
                }

                int bestDist = -1;
                for (int i = 0; i < locations.Length; i++)
                {
                    if (locations[i] > bestDist) bestDist = locations[i];
                }

                for (int i = 0; i < locations.Length; i++)
                {
                    if (locations[i] == bestDist) points[i]++;
                }
            }

            int highScore = 0;
            for (int i = 0; i < points.Length; i++)
            {
                if (points[i] > highScore) highScore = points[i];
            }
            return highScore;
        }
    }
}
