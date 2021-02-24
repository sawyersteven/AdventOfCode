using System;
using System.Collections.Generic;
using System.Numerics;
using System.Text;

namespace Advent2019
{
    public class Challenge12 : Challenge
    {

        public override object Task1()
        {

            Vector3[] moons = new Vector3[input.Count];
            Vector3[] velocities = new Vector3[input.Count];

            for (int i = 0; i < input.Count; i++)
            {
                string[] parts = input[i].Substring(1, input[i].Length - 2).Split(", ");
                moons[i].X = int.Parse(parts[0].Split('=')[1]);
                moons[i].Y = int.Parse(parts[1].Split('=')[1]);
                moons[i].Z = int.Parse(parts[2].Split('=')[1]);
            }

            for (int _ = 0; _ < 1000; _++)
            {

                //Gravity
                for (int i = 0; i < moons.Length - 1; i++)
                {
                    for (int j = i + 1; j < moons.Length; j++)
                    {
                        if (moons[i].X > moons[j].X)
                        {
                            velocities[i].X--;
                            velocities[j].X++;
                        }
                        else if (moons[i].X < moons[j].X)
                        {
                            velocities[i].X++;
                            velocities[j].X--;
                        }

                        if (moons[i].Y > moons[j].Y)
                        {
                            velocities[i].Y--;
                            velocities[j].Y++;
                        }
                        else if (moons[i].Y < moons[j].Y)
                        {
                            velocities[i].Y++;
                            velocities[j].Y--;
                        }

                        if (moons[i].Z > moons[j].Z)
                        {
                            velocities[i].Z--;
                            velocities[j].Z++;
                        }
                        else if (moons[i].Z < moons[j].Z)
                        {
                            velocities[i].Z++;
                            velocities[j].Z--;
                        }
                    }
                }

                // Velocity
                for (int i = 0; i < moons.Length; i++)
                {
                    moons[i] += velocities[i];
                }
            }

            long energy = 0;
            for (int i = 0; i < moons.Length; i++)
            {
                energy += NRG(moons[i]) * NRG(velocities[i]);
            }
            return energy;
        }

        private int NRG(Vector3 v)
        {
            return (int)(Math.Abs(v.X) + Math.Abs(v.Y) + Math.Abs(v.Z));
        }

        public override object Task2()
        {

            return null;
        }
    }
}