using System;
using AdventOfCode;
using Grids;

namespace Advent2021
{
    public class Challenge17 : Challenge
    {
        (int, int) targetRangeY;
        (int, int) targetRangeX;

        public override void ParseInput()
        {
            string[] xy = rawInput.Split(": ")[1].Split(", ");
            int[] x = Array.ConvertAll(xy[0].Substring(2).Split(".."), int.Parse);
            int[] y = Array.ConvertAll(xy[1].Substring(2).Split(".."), int.Parse);

            targetRangeY = (y[1], y[0]);
            targetRangeX = (x[0], x[1]);
        }

        // I'm sure there is a more mathematical way to solve this, but this
        // takes less than 0.2ms so I'm probably not putting any more effort
        // into it
        public override object Task1()
        {
            int bestPeak = 0;

            for (int startVel = 0; startVel < Math.Abs(targetRangeY.Item2); startVel++)
            {
                int pos = 0;
                int vel = -startVel;
                while (true)
                {
                    if (pos < targetRangeY.Item1)
                    {
                        if (pos < targetRangeY.Item2) break;
                        int peak = Gauss(startVel);
                        if (peak > bestPeak) bestPeak = peak;
                        break;
                    }
                    pos += vel;
                    vel--;
                }
            }
            return bestPeak;
        }

        // again, surely there is a more proper way to do this, but it works
        // and only takes ~15ms so its good enough
        public override object Task2()
        {
            int xVelMin = 0;
            while (Gauss(xVelMin) <= targetRangeX.Item1) xVelMin++;
            int xVelMax = targetRangeX.Item2;

            int yVelMin = targetRangeY.Item2;
            int yVelMax = targetRangeY.Item2 * -4; // this is arbitrary.  I don't care. I'm tired.

            int count = 0;
            Vector2Int position;
            Vector2Int startVelocity = new Vector2Int(0, 0);
            for (startVelocity.y = yVelMin; startVelocity.y < yVelMax + 1; startVelocity.y++)
            {
                for (startVelocity.x = xVelMin; startVelocity.x < xVelMax + 1; startVelocity.x++)
                {
                    Vector2Int velocity = startVelocity;
                    velocity.y *= -1;
                    position = velocity;
                    while (true)
                    {
                        // outside on far side
                        if (position.y < targetRangeY.Item2 || position.x > targetRangeX.Item2) break;
                        // inside
                        if (position.y <= targetRangeY.Item1 && position.x >= targetRangeX.Item1)
                        {
                            count++;
                            break;
                        }

                        velocity -= new Vector2Int(1, 1);
                        if (velocity.x < 0) velocity.x = 0;
                        position += velocity;

                    }
                }

            }
            return count;
        }

        private int Gauss(int val) => (val * (val + 1)) / 2; // Gauss twice in one year. Neat.
    }
}
