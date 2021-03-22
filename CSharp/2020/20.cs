using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2020
{
    public class Challenge20 : Challenge
    {
        const int North = 0;
        const int East = 1;
        const int South = 2;
        const int West = 3;

        const int TileSize = 10;

        private class Tile
        {
            public uint id;
            // All edges read clockwise around center
            public string[] edges = new string[4];
            public string[] image = new string[TileSize];
            public int EdgeMatches;

            public void FlipVertical()
            {
                string tmp;
                int i, j;
                for (i = 0, j = image.Length - 1; i < image.Length / 2; i++, j--)
                {
                    tmp = image[i];
                    image[i] = image[j];
                    image[j] = tmp;
                }

                tmp = edges[South];
                edges[South] = Reverse(edges[North]);
                edges[North] = Reverse(tmp);
                edges[East] = Reverse(edges[East]);
                edges[West] = Reverse(edges[West]);
            }

            public void FlipHorizontal()
            {
                for (int i = 0; i < image.Length; i++)
                {
                    image[i] = Reverse(image[i]);
                }
                string tmp = edges[East];
                edges[East] = Reverse(edges[West]);
                edges[West] = Reverse(tmp);
                edges[North] = Reverse(edges[North]);
                edges[South] = Reverse(edges[South]);
            }

            public void RotateCW()
            {
                char[,] tmpMap = new char[image.Length, image.Length];
                int col, nCol;
                for (int row = 0; row < image.Length; row++)
                {
                    for (col = image.Length - 1, nCol = 0; col >= 0; col--, nCol++)
                    {
                        tmpMap[row, nCol] = image[col][row];
                    }
                }

                string tmp = edges[North];
                edges[North] = edges[West];
                edges[West] = edges[South];
                edges[South] = edges[East];
                edges[East] = tmp;

                Char2DToImage(tmpMap);
            }

            public void RotateCCW()
            {
                char[,] tmpMap = new char[image.Length, image.Length];
                int col, nCol;
                for (int row = 0; row < image.Length; row++)
                {
                    for (col = image.Length - 1, nCol = 0; col >= 0; col--, nCol++)
                    {
                        tmpMap[nCol, row] = image[row][col];
                    }
                }

                string tmp = edges[North];
                edges[North] = edges[East];
                edges[East] = edges[South];
                edges[South] = edges[West];
                edges[West] = tmp;

                Char2DToImage(tmpMap);
            }

            public void RotateFull()
            {
                RotateCW();
                RotateCW(); // yolo
            }

            private void Char2DToImage(char[,] map)
            {
                char[] newRow = new char[image.Length];
                for (int row = 0; row < image.Length; row++)
                {
                    for (int col = 0; col < image.Length; col++)
                    {
                        newRow[col] = map[row, col];
                    }
                    image[row] = string.Join("", newRow);
                }
            }
        }

        private List<Tile> Tiles;

        private static Dictionary<string, string> reversedEdges = new Dictionary<string, string>();
        private static string Reverse(string s)
        {
            if (reversedEdges.ContainsKey(s)) return reversedEdges[s];

            char[] c = s.ToCharArray();
            Array.Reverse(c);
            reversedEdges[s] = string.Join("", c);
            return reversedEdges[s];
        }

        private List<Tile> ParseInput()
        {
            List<Tile> tiles = new List<Tile>();
            char[] east = new char[TileSize];
            char[] west = new char[TileSize];
            for (int pos = 0; pos < input.Length; pos += TileSize + 1)
            {
                Tile t = new Tile();
                t.id = uint.Parse(input[pos].Substring(5, 4));
                pos++;

                t.edges[North] = input[pos];
                char[] s = input[pos + TileSize - 1].ToCharArray();
                Array.Reverse(s);
                t.edges[South] = string.Join("", s);

                for (int j = 0; j < TileSize; j++)
                {
                    t.image[j] = input[pos + j];
                    east[j] = input[pos + j][TileSize - 1];
                    west[TileSize - j - 1] = input[pos + j][0];
                }

                t.edges[East] = string.Join("", east);
                t.edges[West] = string.Join("", west);
                tiles.Add(t);
            }
            return tiles;
        }

        public override object Task1()
        {
            ulong answer = 1;
            Tiles = ParseInput();

            for (int i = 0; i < Tiles.Count; i++)
            {
                for (int j = i + 1; j < Tiles.Count; j++)
                {
                    foreach (string edge in Tiles[i].edges)
                    {
                        foreach (string edge2 in Tiles[j].edges)
                        {
                            if (edge == edge2 || edge == Reverse(edge2))
                            {
                                Tiles[i].EdgeMatches++;
                                Tiles[j].EdgeMatches++;
                            }
                        }
                    }
                }
            }

            for (int i = 0; i < Tiles.Count; i++)
            {
                if (Tiles[i].EdgeMatches == 2) answer *= Tiles[i].id;
            }

            return answer;
        }

        private string[] AssembleMap()
        {
            int mapEdgeLen = (int)Math.Sqrt(Tiles.Count);

            List<List<string>> map = new List<List<string>>(mapEdgeLen * mapEdgeLen);
            for (int i = 0; i < mapEdgeLen * mapEdgeLen; i++)
            {
                map.Add(new List<string>(TileSize));
            }

            List<Tile> corners = new List<Tile>(4);
            List<Tile> fills = new List<Tile>((mapEdgeLen * mapEdgeLen) - 4);

            foreach (Tile t in Tiles)
            {
                if (t.EdgeMatches == 2) corners.Add(t);
                else fills.Add(t);
            }

            Tile topLeftCorner = corners[0];

            // orient corner
            bool h = false, v = false;
            for (int j = 1; j < fills.Count; j++)
            {
                if (!h)
                {
                    foreach (string edge2 in fills[j].edges)
                    {
                        if (topLeftCorner.edges[West] == edge2 || topLeftCorner.edges[West] == Reverse(edge2))
                        {
                            topLeftCorner.FlipHorizontal();
                            h = true;
                            break;
                        }
                    }
                }
                if (!v)
                {
                    foreach (string edge2 in fills[j].edges)
                    {
                        if (topLeftCorner.edges[North] == edge2 || topLeftCorner.edges[North] == Reverse(edge2))
                        {
                            topLeftCorner.FlipVertical();
                            v = true;
                            break;
                        }
                    }
                }
            }
            map[0] = new List<string>(topLeftCorner.image);

            string edgeToMatch = topLeftCorner.edges[East];
            int nextMatchDirection = East;
            corners.Remove(topLeftCorner);

            // fill map
            for (int mapInd = 1; mapInd < map.Count; mapInd++)
            {
                Tile next;
                if (mapInd == mapEdgeLen - 1 || mapInd == map.Count - 1 || mapInd == map.Count - mapEdgeLen)
                {
                    next = FindMatch(nextMatchDirection, edgeToMatch, corners);
                }
                else
                {
                    next = FindMatch(nextMatchDirection, edgeToMatch, fills);
                }

                map[mapInd] = new List<string>(next.image);
                if ((mapInd + 1) % mapEdgeLen == 0) // at end of row
                {
                    nextMatchDirection = South;
                    int k = mapInd + 1 - mapEdgeLen;
                    var m = map[mapInd + 1 - mapEdgeLen];
                    edgeToMatch = map[mapInd + 1 - mapEdgeLen][TileSize - 1];
                }
                else
                {
                    nextMatchDirection = East;
                    edgeToMatch = next.edges[East];
                }
            }


            // trim tiles
            for (int i = 0; i < map.Count; i++)
            {
                map[i].RemoveAt(0);
                map[i].RemoveAt(map[i].Count - 1);
                for (int j = 0; j < map[i].Count; j++)
                {
                    map[i][j] = map[i][j].Substring(1, map[i][j].Length - 2);
                }
            }

            // render tiles into single map
            List<string> rendered = new List<string>();

            for (int mapRow = 0; mapRow < map.Count; mapRow += mapEdgeLen)
            {
                for (int tileLine = 0; tileLine < TileSize - 2; tileLine++)
                {
                    string row = string.Empty;
                    for (int mapCol = 0; mapCol < mapEdgeLen; mapCol++)
                    {
                        row += map[mapRow + mapCol][tileLine];
                    }
                    rendered.Add(row);
                }
            }

            // foreach (string line in rendered)
            // {
            //     Console.WriteLine(line);
            // }

            return rendered.ToArray();
        }

        private Tile FindMatch(int matchDirection, string edge, List<Tile> candidates)
        {
            foreach (Tile t in candidates)
            {
                for (int side = 0; side < 4; side++)
                {
                    if (edge == t.edges[side])
                    {
                        int rot = (matchDirection - side + 6) % 4;
                        if (rot == 1) t.RotateCW();
                        else if (rot == 2) t.RotateFull();
                        else if (rot == 3) t.RotateCCW();
                        if (matchDirection % 2 != 0) t.FlipVertical();
                        candidates.Remove(t);
                        return t;
                    }
                }
                t.FlipHorizontal();
                for (int side = 0; side < 4; side++)
                {
                    if (edge == t.edges[side])
                    {
                        int rot = (matchDirection - side + 6) % 4;
                        if (rot == 1) t.RotateCW();
                        else if (rot == 2) t.RotateFull();
                        else if (rot == 3) t.RotateCCW();
                        if (matchDirection % 2 != 0) t.FlipVertical();
                        candidates.Remove(t);
                        return t;
                    }
                }
            }
            return null;
        }

        private int FindDragons(string[] map)
        {
            int found = 0;
            int row1Ind = 18;
            int[] row2Indexes = new int[] { 0, 5, 6, 11, 12, 17, 18, 19 };
            int[] row3Indexes = new int[] { 1, 4, 7, 10, 13, 16 };

            for (int row = 0; row < map.Length - 2; row++)
            {
                for (int r1 = row1Ind; r1 < map.Length - 1; r1++)
                {
                    if (map[row][r1] == '#')
                    {
                        bool ok = true;
                        foreach (int r2 in row2Indexes)
                        {
                            if (map[row + 1][r2 - row1Ind + r1] != '#')
                            {
                                ok = false;
                                break;
                            }
                        }
                        if (!ok) continue;
                        foreach (int r3 in row3Indexes)
                        {
                            if (map[row + 2][r3 - row1Ind + r1] != '#')
                            {
                                ok = false;
                                break;
                            }
                        }
                        if (ok) found++;
                    }
                }
            }

            return found;
        }

        public override object Task2()
        {
            string[] map = AssembleMap();

            int fd = FindDragons(map);
            int rot = 0;
            while (fd == 0)
            {
                if (rot % 4 == 1)
                {
                    // Flip Vertical
                    string tmp;
                    int i, j;
                    for (i = 0, j = map.Length - 1; i < map.Length / 2; i++, j--)
                    {
                        tmp = map[i];
                        map[i] = map[j];
                        map[j] = tmp;
                    }
                }

                // Rotate CW
                char[][] tmpMap = new char[map.Length][];
                int col, nCol;
                for (int row = 0; row < map.Length; row++)
                {
                    tmpMap[row] = new char[map.Length];
                    for (col = map.Length - 1, nCol = 0; col >= 0; col--, nCol++)
                    {
                        tmpMap[row][nCol] = map[col][row];
                    }
                }
                for (int row = 0; row < tmpMap.Length; row++)
                {
                    map[row] = string.Join("", tmpMap[row]);
                }
                rot++;

                fd = FindDragons(map);
            }

            ulong dragonPixels = (ulong)fd * 15;
            ulong answer = 0;
            for (int row = 0; row < map.Length; row++)
            {
                for (int c = 0; c < map[0].Length; c++)
                {
                    if (map[row][c] == '#') answer++;
                }
            }

            return answer - dragonPixels;
        }
    }
}