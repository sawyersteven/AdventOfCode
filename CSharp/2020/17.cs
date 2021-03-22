using AdventOfCode;
using System.Collections.Generic;

// These suppress the warning for Vector4 no overriding GetHashCode and Equals
// If I create these methods the code is much slower, so I'm happy to ignore it
#pragma warning disable CS0660
#pragma warning disable CS0661

namespace Advent2020
{
    public class Challenge17 : Challenge
    {
        const char active = '#';
        const char inactive = '.';

        private char[,,] cube;

        // Mostly arbitrary, but these values are large enough and run in ~100ms and that's good enough
        const int cubeWidth = 24;
        const int idxOffset = cubeWidth / 2;

        Dictionary<Vector3, Vector3[]> neighbors;

        public override object Task1()
        {
            Dictionary<Vector3, char> changes = new Dictionary<Vector3, char>();
            neighbors = new Dictionary<Vector3, Vector3[]>();

            cube = new char[cubeWidth, cubeWidth, cubeWidth];

            foreach (Vector3 n in CubeNodes())
            {
                cube[n.Z, n.X, n.Y] = inactive;
            }

            // copy input
            for (int x = 0; x < input[0].Length; x++)
            {
                for (int y = input.Length - 1; y >= 0; y--)
                {
                    cube[idxOffset, x + idxOffset, y + idxOffset] = input[x][y];
                }
            }

            for (int turn = 0; turn < 6; turn++)
            {
                foreach (Vector3 n in CubeNodes())
                {
                    int count = 0;
                    switch (cube[n.Z, n.X, n.Y])
                    {
                        case active:
                            foreach (Vector3 other in Neighbors(n))
                            {
                                if (cube[other.Z, other.X, other.Y] == active) count++;
                            }
                            if (count > 3 || count < 2) changes[n] = inactive;
                            break;
                        case inactive:
                            foreach (Vector3 other in Neighbors(n))
                            {
                                if (cube[other.Z, other.X, other.Y] == active) count++;
                            }
                            if (count == 3) changes[n] = active;
                            break;
                    }
                }

                foreach (Vector3 key in changes.Keys)
                {
                    cube[key.Z, key.X, key.Y] = changes[key];
                }
                changes.Clear();
            }

            long answer = 0;
            foreach (Vector3 n in CubeNodes())
            {
                if (cube[n.Z, n.X, n.Y] == active) answer++; ;
            }
            return answer;
        }

        private IEnumerable<Vector3> CubeNodes()
        {
            Vector3 v = new Vector3();
            for (int z = 0; z < cubeWidth; z++)
            {
                v.Z = z;
                for (int x = 0; x < cubeWidth; x++)
                {
                    v.X = x;
                    for (int y = 0; y < cubeWidth; y++)
                    {
                        v.Y = y;
                        yield return v;
                    }
                }
            }
        }

        private struct Vector3
        {
            public int X;
            public int Y;
            public int Z;

            public Vector3(int x, int y, int z)
            {
                X = x;
                Y = y;
                Z = z;
            }
        }

        private Vector3[] Neighbors(Vector3 origin)
        {
            if (neighbors.ContainsKey(origin)) return neighbors[origin];

            List<Vector3> n = new List<Vector3>();

            for (int x = origin.X - 1; x <= origin.X + 1; x++)
            {
                if (x < 0 || x >= cubeWidth) continue;
                for (int y = origin.Y - 1; y <= origin.Y + 1; y++)
                {
                    if (y < 0 || y >= cubeWidth) continue;
                    for (int z = origin.Z - 1; z <= origin.Z + 1; z++)
                    {
                        if (z < 0 || z >= cubeWidth) continue;
                        n.Add(new Vector3(x, y, z));
                    }
                }
            }
            n.Remove(origin);
            return n.ToArray();
        }

        private char[,,,] T2cube;

        Dictionary<Vector4, Vector4[]> T2neighbors;
        public override object Task2()
        {
            Dictionary<Vector4, char> changes = new Dictionary<Vector4, char>();
            T2neighbors = new Dictionary<Vector4, Vector4[]>();

            T2cube = new char[cubeWidth, cubeWidth, cubeWidth, cubeWidth];

            foreach (Vector4 n in T2CubeNodes())
            {
                T2cube[n.W, n.Z, n.X, n.Y] = inactive;
            }

            // copy input
            for (int x = 0; x < input[0].Length; x++)
            {
                for (int y = input.Length - 1; y >= 0; y--)
                {
                    T2cube[idxOffset, idxOffset, x + idxOffset, y + idxOffset] = input[x][y];
                }
            }

            for (int turn = 0; turn < 6; turn++)
            {
                foreach (Vector4 n in T2CubeNodes())
                {
                    int count = 0;
                    switch (T2cube[n.W, n.Z, n.X, n.Y])
                    {
                        case active:
                            foreach (Vector4 other in T2Neighbors(n))
                            {
                                if (T2cube[other.W, other.Z, other.X, other.Y] == active) count++;
                                if (count > 3)
                                {
                                    changes[n] = inactive;
                                    break;
                                }
                            }
                            if (count < 2) changes[n] = inactive;
                            break;
                        case inactive:
                            foreach (Vector4 other in T2Neighbors(n))
                            {
                                if (T2cube[other.W, other.Z, other.X, other.Y] == active) count++;
                                if (count > 3) break;
                            }
                            if (count == 3) changes[n] = active;
                            break;
                    }
                }

                foreach (Vector4 key in changes.Keys)
                {
                    T2cube[key.W, key.Z, key.X, key.Y] = changes[key];
                }
                changes.Clear();
            }

            long answer = 0;
            foreach (Vector4 n in T2CubeNodes())
            {
                if (T2cube[n.W, n.Z, n.X, n.Y] == active) answer++; ;
            }
            return answer;
        }

        private struct Vector4
        {
            public int X;
            public int Y;
            public int Z;
            public int W;

            public Vector4(int x, int y, int z, int w)
            {
                X = x;
                Y = y;
                Z = z;
                W = w;
            }

            public static bool operator ==(Vector4 a, Vector4 b)
            {
                return (a.W == b.W && a.Z == b.Z && a.X == b.X && a.Y == b.Y);
            }

            public static bool operator !=(Vector4 a, Vector4 b)
            {
                return (a.W != b.W || a.Z != b.Z || a.X != b.X || a.Y != b.Y);
            }
        }

        private IEnumerable<Vector4> T2CubeNodes()
        {
            Vector4 v = new Vector4();
            for (int z = 0; z < cubeWidth; z++)
            {
                v.Z = z;
                for (int x = 0; x < cubeWidth; x++)
                {
                    v.X = x;
                    for (int y = 0; y < cubeWidth; y++)
                    {
                        v.Y = y;
                        for (int w = 0; w < cubeWidth; w++)
                        {
                            v.W = w;
                            yield return v;
                        }
                    }
                }
            }
        }


        private Vector4[] T2Neighbors(Vector4 origin)
        {
            if (T2neighbors.ContainsKey(origin)) return T2neighbors[origin];

            Vector4[] n = new Vector4[80];
            Vector4 v = new Vector4();
            int i = 0;
            for (int x = origin.X - 1; x <= origin.X + 1; x++)
            {
                if (x < 0 || x >= cubeWidth) continue;
                v.X = x;
                for (int y = origin.Y - 1; y <= origin.Y + 1; y++)
                {
                    if (y < 0 || y >= cubeWidth) continue;
                    v.Y = y;
                    for (int z = origin.Z - 1; z <= origin.Z + 1; z++)
                    {
                        if (z < 0 || z >= cubeWidth) continue;
                        v.Z = z;
                        for (int w = origin.W - 1; w <= origin.W + 1; w++)
                        {
                            if (w < 0 || w >= cubeWidth) continue;
                            v.W = w;
                            if (v == origin) continue;
                            n[i] = v;
                            i++;
                        }
                    }
                }
            }
            return n;
        }


    }
}