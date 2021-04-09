using System;
using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2017
{
    public class Challenge20 : Challenge
    {
        private class Particle
        {
            public Vector3Int Position;
            public Vector3Int Velocity;
            public Vector3Int Accel;
            public int ID;

            public static Particle Parse(string line, int id)
            {
                Particle p = new Particle();

                p.ID = id;

                //p=<2366,784,-597>, v=<-12,-41,50>, a=<-5,1,-2>
                string[] parts = line.Split(">, ");
                int[] nums = Array.ConvertAll(parts[0].Substring(3).Split(','), int.Parse);
                p.Position.x = nums[0];
                p.Position.y = nums[1];
                p.Position.z = nums[2];

                nums = Array.ConvertAll(parts[1].Substring(3).Split(','), int.Parse);
                p.Velocity.x = nums[0];
                p.Velocity.y = nums[1];
                p.Velocity.z = nums[2];

                nums = Array.ConvertAll(parts[2].Substring(3, parts[2].Length - 4).Split(','), int.Parse);
                p.Accel.x = nums[0];
                p.Accel.y = nums[1];
                p.Accel.z = nums[2];

                return p;
            }

            public void Tick()
            {
                Velocity.x += Accel.x;
                Velocity.y += Accel.y;
                Velocity.z += Accel.z;

                Position.x += Velocity.x;
                Position.y += Velocity.y;
                Position.z += Velocity.z;
            }
        }

        /* I don't love this since it uses a mostly arbitrary polling rate to
        check if the closest point has stablized. But it works and trying to
        optimize for other potential input values seems fruitless.
        */
        public override object Task1()
        {
            Particle[] particles = new Particle[input.Length];
            for (int i = 0; i < input.Length; i++)
            {
                particles[i] = Particle.Parse(input[i], i);
            }

            long shortestDist = long.MaxValue;
            int shortestID = 0;

            for (int i = 0; ; i++)
            {
                foreach (Particle p in particles) p.Tick();

                if (i % 200 == 0)
                {
                    shortestDist = long.MaxValue;
                    int lastShortID = shortestID;
                    foreach (Particle p in particles)
                    {
                        long d = p.Position.ManhattanDistance(Vector3Int.Zero);
                        if (d < shortestDist)
                        {
                            shortestDist = d;
                            shortestID = p.ID;
                        }
                    }
                    if (shortestID == lastShortID) return shortestID;
                }
            }
        }

        /* I feel like there is probably a more math-oriented approach to this
        as well, but since the particles move on a curve the math is something
        I don't have the patience to figure out at the moment. So the 1000 tick
        simulation is completely arbitrary, but it get the right answer.
        */
        public override object Task2()
        {
            HashSet<Particle> particles = new HashSet<Particle>(1000);
            for (int i = 0; i < input.Length; i++)
            {
                particles.Add(Particle.Parse(input[i], i));
            }

            for (int _ = 0; _ < 1000; _++)
            {
                HashSet<Vector3Int> positions = new HashSet<Vector3Int>();
                HashSet<Vector3Int> duplicatePositions = new HashSet<Vector3Int>();

                foreach (Particle p in particles)
                {
                    if (positions.Contains(p.Position)) duplicatePositions.Add(p.Position);
                    positions.Add(p.Position);
                }

                foreach (Particle p in particles)
                {
                    if (duplicatePositions.Contains(p.Position))
                    {
                        particles.Remove(p);
                    }
                    else
                    {
                        p.Tick();
                    }
                }
            }

            return particles.Count;
        }
    }
}
