using System;
using System.Collections.Generic;
using AdventOfCode;
using Grids;


/*
This one is a doozy. More tedious than anything because one small typo or
mistake can be very hard to find when handling this many iterations of this
much data.

I left my comments to explain some of my process where it might not be
super obvious why there are 87 nested for loops.

This is also not sure fast so I left some console printing to let you
know it is still chugging along
*/

namespace Advent2021
{
    public class Challenge19 : Challenge
    {
        private const int matchThreshold = 12;

        private class Scanner
        {
            public readonly int ID = 0;
            public Vector3Int Location { get; private set; }
            public int Rotation { get; private set; }
            public HashSet<Vector3Int>[] Beacons = new HashSet<Vector3Int>[24];
            public HashSet<Vector3Int> ProperOrientation => Beacons[Rotation];

            public Scanner(int id)
            {
                ID = id;
                for (int i = 0; i < Beacons.Length; i++)
                {
                    Beacons[i] = new HashSet<Vector3Int>();
                }
            }

            public void AddBeacon(Vector3Int v)
            {
                int i = 0;
                foreach (Vector3Int rv in RotateAll(v))
                {
                    Beacons[i].Add(rv);
                    i++;
                }
            }

            // Sets location, rotation, proper orientation set, and anchor point
            public void SetLocation(Vector3Int loc, int rotation)
            {
                Location = loc;
                Rotation = rotation;
                HashSet<Vector3Int> h = new HashSet<Vector3Int>();
                foreach (Vector3Int v in Beacons[rotation])
                {
                    h.Add(loc + v);
                }
                Beacons[rotation].Clear();
                Beacons[Rotation].UnionWith(h);
            }

            // 90 on Y
            public void RotateY(ref Vector3Int v) => (v.x, v.z) = (-v.z, v.x);

            // 90 on X
            public void RotateX(ref Vector3Int v) => (v.y, v.z) = (v.z, -v.y);

            // 90 on Z
            public void RotateZ(ref Vector3Int v) => (v.x, v.y) = (v.y, -v.x);

            public IEnumerable<Vector3Int> RotateAll(Vector3Int v)
            {
                // top
                yield return v;
                RotateZ(ref v);
                yield return v;
                RotateZ(ref v);
                yield return v;
                RotateZ(ref v);
                yield return v;

                // front
                RotateX(ref v);
                yield return v;
                RotateY(ref v);
                yield return v;
                RotateY(ref v);
                yield return v;
                RotateY(ref v);
                yield return v;

                // side
                RotateZ(ref v);
                yield return v;
                RotateX(ref v);
                yield return v;
                RotateX(ref v);
                yield return v;
                RotateX(ref v);
                yield return v;

                // back
                RotateZ(ref v);
                yield return v;
                RotateY(ref v);
                yield return v;
                RotateY(ref v);
                yield return v;
                RotateY(ref v);
                yield return v;

                // other side
                RotateZ(ref v);
                yield return v;
                RotateX(ref v);
                yield return v;
                RotateX(ref v);
                yield return v;
                RotateX(ref v);
                yield return v;

                // bottom
                RotateY(ref v);
                yield return v;
                RotateZ(ref v);
                yield return v;
                RotateZ(ref v);
                yield return v;
                RotateZ(ref v);
                yield return v;
            }
        }


        private List<Scanner> lostScanners;
        public override void ParseInput()
        {
            lostScanners = new List<Scanner>();
            Scanner current = null;
            int id = 0;
            foreach (string line in input)
            {
                if (string.IsNullOrWhiteSpace(line))
                {
                    lostScanners.Add(current);
                }
                else if (line[1] == '-')
                {
                    current = new Scanner(id);
                    id++;
                }
                else
                {
                    int[] nums = Array.ConvertAll(line.Split(','), int.Parse);
                    current.AddBeacon(new Vector3Int(nums[0], nums[1], nums[2]));
                }
            }
            lostScanners.Add(current);
        }

        List<Scanner> foundScanners;
        public override object Task1()
        {
            foundScanners = new List<Scanner>(lostScanners.Count);
            foundScanners.Add(lostScanners[0]);
            lostScanners.RemoveAt(0);

            while (lostScanners.Count != 0)
            {
                for (int i = 0; i < foundScanners.Count; i++)
                {
                    Scanner anchor = foundScanners[i];
                    foreach (Scanner lost in lostScanners)
                    {
                        if (FindMatch(anchor, lost))
                        {
                            lostScanners.Remove(lost);
                            foundScanners.Add(lost);
                            break;
                        }
                    }
                }
            }

            HashSet<Vector3Int> masterBeaconSet = new HashSet<Vector3Int>();
            foreach (Scanner s in foundScanners)
            {
                foreach (Vector3Int b in s.ProperOrientation) masterBeaconSet.Add(b);
            }
            return masterBeaconSet.Count;
        }

        private bool FindMatch(Scanner anchorScanner, Scanner lostScanner)
        {
            for (int rotation = 0; rotation < lostScanner.Beacons.Length; rotation++)
            {
                (bool match, Vector3Int scannerLoc) = FindScannerLocation(anchorScanner.ProperOrientation, lostScanner.Beacons[rotation]);
                if (match)
                {
                    Console.WriteLine($"Match {lostScanner.ID} to {anchorScanner.ID}");
                    lostScanner.SetLocation(scannerLoc, rotation);
                    return true;
                }
            }
            return false;
        }

        private (bool, Vector3Int) FindScannerLocation(HashSet<Vector3Int> anchorBeacons, HashSet<Vector3Int> lostScannerBeacons)
        {
            // Iter through all the beacons in the anchor set because the 
            // location of the lostScanner can only be found if the anchorBeacon
            // is one of the 12 overlapping matches.
            foreach (Vector3Int anchorBeacon in anchorBeacons)
            {
                // Get the offset of each unknown beacon to the anchor beacon.
                // Use this offset to create a new set of beacons that have this
                // offset applied. If 12 of these also exist in the known anchor
                // beacons we have a match against this anchor scanner.
                foreach (Vector3Int lostScannerBeacon in lostScannerBeacons)
                {
                    int count = 0;
                    Vector3Int lostScannerLoc = anchorBeacon - lostScannerBeacon;
                    foreach (Vector3Int freeBeacon in lostScannerBeacons)
                    {
                        if (anchorBeacons.Contains(lostScannerLoc + freeBeacon)) count++;
                    }
                    if (count >= matchThreshold)
                    {
                        return (true, lostScannerLoc);
                    }
                }
            }
            return (false, Vector3Int.Zero);
        }

        public override object Task2()
        {
            long longest = 0;
            for (int i = 0; i < foundScanners.Count - 1; i++)
            {
                for (int j = i + 1; j < foundScanners.Count; j++)
                {
                    long d = foundScanners[i].Location.ManhattanDistance(foundScanners[j].Location);
                    if (d > longest) longest = d;
                }
            }
            return longest;

        }
    }
}
