using System;
using System.Collections.Generic;
using System.Numerics;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2021
{
    public class Challenge22 : Challenge
    {
        /*
            Part 1 is a simple brute-force. I left it this way to show how slow
            it is to iterate through a small portion of 3d space.

            The proper solution to part 2 likely involves some set theory that
            I'm not in the mood to learn. 

            Its easier to explain part 2 in 2D. Imagine a stack of cards. Lay a
            card on a grid. For every card that it overlaps, add another card
            that indicates a negative region. If the card itself is a negative
            region do the same but do not add the card to the stack and add 
            another addition card for each overlap.

            This creates a fair amount of regions, but summing their volume can
            be done as they are added and we don't need several GB of memory to 
            hold all the points.

        */

        public class Cuboid
        {
            public readonly bool On = true;
            public readonly long Minx;
            public readonly long Maxx;
            public readonly long Miny;
            public readonly long Maxy;
            public readonly long Minz;
            public readonly long Maxz;
            public readonly long Volume;

            public Cuboid(long minx, long maxx, long miny, long maxy, long minz, long maxz, bool isOn)
            {
                Minx = minx;
                Miny = miny;
                Minz = minz;
                Maxx = maxx;
                Maxy = maxy;
                Maxz = maxz;
                On = isOn;
                Volume = Math.Abs((Maxx - Minx + 1) * (Maxy - Miny + 1) * (Maxz - Minz + 1)) * (On ? 1 : -1);
            }
        }

        private bool Intersects(Cuboid a, Cuboid b)
        {
            return a.Maxx >= b.Minx && a.Minx <= b.Maxx &&
                   a.Maxy >= b.Miny && a.Miny <= b.Maxy &&
                   a.Maxz >= b.Minz && a.Minz <= b.Maxz;
        }

        private Cuboid Intersection(Cuboid a, Cuboid b)
        {
            return new Cuboid(Math.Max(a.Minx, b.Minx), Math.Min(a.Maxx, b.Maxx), Math.Max(a.Miny, b.Miny), Math.Min(a.Maxy, b.Maxy), Math.Max(a.Minz, b.Minz), Math.Min(a.Maxz, b.Maxz), !a.On);
        }

        private Cuboid[] cubes;
        public override void ParseInput()
        {
            cubes = new Cuboid[input.Length];

            for (int i = 0; i < input.Length; i++)
            {
                string line = input[i];
                string[] xyz = line.Split(' ')[1].Split(',');

                long[] x = Array.ConvertAll(xyz[0].Substring(2).Split(".."), long.Parse);
                long[] y = Array.ConvertAll(xyz[1].Substring(2).Split(".."), long.Parse);
                long[] z = Array.ConvertAll(xyz[2].Substring(2).Split(".."), long.Parse);

                cubes[i] = new Cuboid(x[0], x[1], y[0], y[1], z[0], z[1], line[1] == 'n');
            }
        }

        public override object Task1()
        {
            Vector3Int regionMin = new Vector3Int(-50, -50, -50);
            Vector3Int regionMax = new Vector3Int(50, 50, 50);

            HashSet<Vector3Int> onNodes = new HashSet<Vector3Int>();
            Vector3Int v = Vector3Int.Zero;
            foreach (Cuboid c in cubes)
            {
                Cuboid cc = new Cuboid(Math.Max(regionMin.x, c.Minx), Math.Min(regionMax.x, c.Maxx), Math.Max(regionMin.y, c.Miny), Math.Min(regionMax.y, c.Maxy), Math.Max(regionMin.z, c.Minz), Math.Min(regionMax.z, c.Maxz), c.On);
                for (v.x = (int)cc.Minx; v.x <= cc.Maxx; v.x++)
                {
                    for (v.y = (int)cc.Miny; v.y <= cc.Maxy; v.y++)
                    {
                        for (v.z = (int)cc.Minz; v.z <= cc.Maxz; v.z++)
                        {
                            if (cc.On) onNodes.Add(v);
                            else onNodes.Remove(v);
                        }
                    }
                }
            }
            return onNodes.Count;
        }

        public override object Task2()
        {
            long total = 0;
            List<Cuboid> processed = new List<Cuboid>();
            foreach (Cuboid cube in cubes)
            {
                int k = processed.Count;
                for (int i = 0; i < k; i++)
                {
                    Cuboid c = processed[i];
                    if (!Intersects(c, cube)) continue;
                    Cuboid intcube = Intersection(c, cube);
                    processed.Add(intcube);
                    total += intcube.Volume;
                }
                if (cube.On)
                {
                    processed.Add(cube);
                    total += cube.Volume;
                }
            }
            return total;
        }
    }
}