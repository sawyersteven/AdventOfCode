using AdventOfCode;
using System;

namespace Advent2018
{
    public class Challenge23 : Challenge
    {

        private class Nanobot
        {
            public readonly int X;
            public readonly int Y;
            public readonly int Z;
            public readonly int Radius;
            public readonly int MinDistToZero;
            public readonly int MaxDistToZero;

            public Nanobot(string inputLine)
            {
                string[] parts = inputLine.Split(">, ");
                int[] pos = Array.ConvertAll(parts[0].Split('<')[1].Split(','), int.Parse);
                X = pos[0];
                Y = pos[1];
                Z = pos[2];
                Radius = int.Parse(parts[1].Split('=')[1]);

                int nyDist = Math.Abs(X) + Math.Abs(Y) + Math.Abs(Z);

                MinDistToZero = Math.Abs(nyDist - Radius);
                MaxDistToZero = nyDist + Radius;
            }
        }

        private int ManhattanDistance(Nanobot a, Nanobot b)
        {
            return Math.Abs(a.X - b.X) + Math.Abs(a.Y - b.Y) + Math.Abs(a.Z - b.Z);
        }

        private Nanobot[] bots;
        public override object Task1()
        {
            bots = new Nanobot[input.Length];
            for (int i = 0; i < bots.Length; i++)
            {
                bots[i] = new Nanobot(input[i]);
            }

            Nanobot strongest = bots[0];
            for (int i = 1; i < bots.Length; i++)
            {
                if (bots[i].Radius > strongest.Radius) strongest = bots[i];
            }

            int count = 0;

            foreach (Nanobot n in bots)
            {
                if (ManhattanDistance(n, strongest) <= strongest.Radius)
                {
                    count++;
                }
            }

            return count;
        }


        /*
        This is another tricky puzzle that is easier than it seems at first

        We are tasked with finding a distance from 0,0,0 that has the most
        overlapping nanobot ranges. Not a *point*, but the distance of that
        point. This is an important distinction.

        
        Nanobot ranges are effectively cubes since the Range value uses the
        manhattan distance. The distance we are looking for must be one of
        distances from zero to an edge, since any count will only change
        at the edge of these cubes.

        Consider the distance as a single axis. Cubes exist on this axis as
        a min and max value. Line segments like a Gantt chart. Since we are
        measuing the distance from zero, the min end of this imaginary line
        should be its absolute value in case it dips below zero.

        So the min and max values each get added to a list with a +/- value
        to indicate if it is the beginning or end of our line segment. This
        list of values gets sorted by distance to zero.

        We then go through the list in order, keeping tally of how many cubes
        we are inside using the +/- value. A -1 indicates we are leaving a
        cube, and the +1 indicates entering a cube.

        Then it is a simple matter of finding when this tally is at its
        highest value and returning the distance to the edge that exists
        at this point.

        I'm not 100% convinced this will work with all possible input data.
        If a cube exists solely in negative space it will overlap a cube
        that is the identical positive coordinates using this system and
        will result in a counter of +2 when it should only be +1. So this
        solution may be a combination of dumb luck and the puzzle input not
        having any traps like this included.
        */

        public override object Task2()
        {
            (int, int)[] edges = new (int, int)[bots.Length * 2];
            for (int i = 0; i < bots.Length; i++)
            {
                edges[i * 2] = (bots[i].MinDistToZero, 1);
                edges[(i * 2) + 1] = (bots[i].MaxDistToZero, -1);
            }

            Array.Sort(edges, (a, b) => { return a.Item1 - b.Item1; });

            int currentOverlap = 0;
            int bestOverlapCount = 0;
            int bestOverlapDistance = 0;
            foreach ((int dist, int transition) in edges)
            {
                currentOverlap += transition;
                if (currentOverlap > bestOverlapCount)
                {
                    bestOverlapCount = currentOverlap;
                    bestOverlapDistance = dist;
                }
            }
            return bestOverlapDistance;
        }
    }
}