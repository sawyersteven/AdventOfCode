using System;
using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;

namespace Advent2017
{
    public class Challenge21 : Challenge
    {
        private char[] initialState = new char[] { '.','#','.',
                                                   '.','.','#',
                                                   '#','#','#'};

        private Dictionary<string, char[]> lookupTable = new Dictionary<string, char[]>();
        public override void ParseInput()
        {
            foreach (string line in input)
            {
                string[] parts = line.Split(" => ");
                char[] v = parts[1].Replace("/", "").ToCharArray();
                char[,] k = LineToGrid(parts[0]);

                for (int _ = 0; _ < 4; _++)
                {
                    k = k.RotatedCW();
                    lookupTable[k.Join(',')] = v;
                }
                k.FlipHorizontal();
                for (int _ = 0; _ < 4; _++)
                {
                    k = k.RotatedCW();
                    lookupTable[k.Join('/')] = v;
                }
            }
        }

        private char[,] LineToGrid(string line)
        {
            string[] parts = line.Split('/');

            char[,] grid = new char[parts.Length, parts[0].Length];
            for (int y = 0; y < parts.Length; y++)
            {
                for (int x = 0; x < parts[0].Length; x++)
                {
                    grid[y, x] = parts[y][x];
                }
            }
            return grid;
        }

        static T[][] Divide<T>(T[] array)
        {
            int chunkSize = array.Length % 2 == 0 ? 2 : 3;
            int chunksPerEdge = (int)Math.Sqrt(array.Length) / chunkSize;
            T[][] chunks = new T[chunksPerEdge * chunksPerEdge][];

            int start = 0;
            for (int i = 0; i < chunks.Length; i++)
            {
                chunks[i] = new T[chunkSize * chunkSize];

                int j = 0;
                for (int y = 0; y < chunkSize; y++)
                {
                    for (int x = 0; x < chunkSize; x++, j++)
                    {
                        int ind = start + x + (y * chunkSize * chunksPerEdge);
                        chunks[i][j] = array[ind];
                    }
                }

                start += chunkSize;
                if (start % (chunksPerEdge * chunkSize) == 0)
                {
                    start += chunksPerEdge * chunkSize * (chunkSize - 1);
                }

            }
            return chunks;
        }

        private T[] Join<T>(T[][] chunks)
        {
            int chunkSize = (int)Math.Sqrt(chunks[0].Length);
            int chunksPerEdge = (int)Math.Sqrt(chunks.Length);
            T[] result = new T[chunks.Length * chunks[0].Length];

            int start = 0;
            for (int i = 0; i < chunks.Length; i++)
            {
                int j = 0;
                for (int y = 0; y < chunkSize; y++)
                {
                    for (int x = 0; x < chunkSize; x++, j++)
                    {
                        int ind = start + x + (y * chunkSize * chunksPerEdge);
                        result[ind] = chunks[i][j];
                    }
                }

                start += chunkSize;
                if (start % (chunksPerEdge * chunkSize) == 0)
                {
                    start += chunksPerEdge * chunkSize * (chunkSize - 1);
                }

            }
            return result;
        }

        /* I made this much more difficult than neccesary by trying to use
        char[,] arrays throughout. After I gave up on that and just went
        with single-dimension arrays where possible it was actually fairly
        easy to solve.

        I feel like there is supposed to be a more clever way to solve part 2
        than just running the loop 18 times, but it only takes ~300ms so its
        good enough for me. Perhaps making the lookup table account for all
        orientations in the parsing step made this quicker than expected.
        */
        public override object Task1()
        {
            char[] image = initialState.Clone() as char[];

            for (int turn = 0; turn < 5; turn++)
            {
                image = ENHANCE(image);
            }

            int count = 0;
            foreach (char i in image)
            {
                if (i == '#') count++;
            }
            return count;
        }

        private char[] ENHANCE(char[] image)
        {
            char[][] cells = Divide(image);
            for (int i = 0; i < cells.Length; i++)
            {
                cells[i] = lookupTable[string.Join('/', cells[i])];
            }
            return Join(cells);
        }

        public override object Task2()
        {
            char[] image = initialState.Clone() as char[];

            for (int turn = 0; turn < 18; turn++)
            {
                image = ENHANCE(image);
            }

            long count = 0;
            foreach (char i in image)
            {
                if (i == '#') count++;
            }
            return count;
        }
    }
}
