using AdventOfCode;
namespace Advent2018
{
    public class Challenge11 : Challenge
    {
        private const int gridSize = 300;
        private int serialNum;
        private int[,] powerGrid;

        public override object Task1()
        {
            serialNum = int.Parse(input[0]);
            powerGrid = new int[gridSize, gridSize];

            for (int y = 0; y < gridSize; y++)
            {
                for (int x = 0; x < gridSize; x++)
                {
                    powerGrid[y, x] = CellPower(x, y);
                }
            }
            (int, int) bestCell = (0, 0);
            int bestCellValue = 0;

            for (int y = 0; y < 297; y++)
            {
                for (int x = 0; x < 297; x++)
                {
                    int p = ChunkPower(x, y);
                    if (p > bestCellValue)
                    {
                        bestCellValue = p;
                        bestCell = (x, y);
                    }
                }
            }
            return $"{bestCell.Item1},{bestCell.Item2}";
        }

        private int ChunkPower(int x, int y)
        {
            int total = 0;
            for (int yy = y; yy < y + 3; yy++)
            {
                for (int xx = x; xx < x + 3; xx++)
                {
                    total += powerGrid[yy, xx];
                }
            }
            return total;
        }

        private int CellPower(int x, int y)
        {
            int rid = x + 10;
            int power = rid * y;
            power += serialNum;
            power *= rid;
            power = (power / 100) % 10;
            power -= 5;
            return power;
        }

        public override object Task2()
        {
            IntegralImage ii = new IntegralImage(powerGrid);

            (int, int, int) gridBox = (0, 0, 0);
            int bestBoxValue = int.MinValue;

            for (int size = 1; size < gridSize; size++)
            {
                for (int y = 0; y < gridSize - size; y++)
                {
                    for (int x = 0; x < gridSize - size; x++)
                    {
                        int v = ii.AreaSum(x, y, size, size);
                        if (v > bestBoxValue)
                        {
                            bestBoxValue = v;
                            gridBox = (x - size + 1, y - size + 1, size);
                        }
                    }

                }
            }
            System.Console.WriteLine(bestBoxValue);
            return $"{gridBox.Item1},{gridBox.Item2},{gridBox.Item3}";
        }

        private class IntegralImage
        {
            private int[,] table;
            private int width;
            private int height;
            public IntegralImage(int[,] original)
            {
                height = original.GetLength(0);
                width = original.GetLength(1);
                table = new int[height, width];

                for (int y = 0; y < height; y++)
                {
                    for (int x = 0; x < height; x++)
                    {
                        table[y, x] = original[y, x] + Get(x - 1, y) + Get(x, y - 1) - Get(x - 1, y - 1);
                    }
                }
            }

            public int Get(int x, int y)
            {
                if (x < 0 || y < 0 || x >= width || y >= height)
                {
                    return 0;
                }
                return table[y, x];
            }

            public int AreaSum(int x, int y, int w, int h)
            {
                return Get(x, y) - Get(x, y - h) - Get(x - w, y) + Get(x - w, y - h);
            }
        }
    }
}
