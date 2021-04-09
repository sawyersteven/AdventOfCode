using System;

namespace ExtensionMethods
{

    public static class ArrayExtensions
    {
        public static string Join(this char[,] arr, char sep)
        {
            int h = arr.GetLength(0);
            int w = arr.GetLength(1);
            char[] chars = new char[h * w];
            for (int y = 0; y < h; y++)
            {
                for (int x = 0; x < w; x++)
                {
                    chars[(y * h) + x] = arr[y, x];
                }
            }
            return string.Join(sep, chars);
        }

        public static void Print<T>(this T[,] grid)
        {
            for (int i = 0; i < grid.GetLength(0); i++)
            {
                T[] line = new T[grid.GetLength(1)];
                for (int j = 0; j < grid.GetLength(1); j++)
                {
                    line[j] = grid[i, j];
                }
                Console.WriteLine(string.Join("", line));
            }
        }

        public static T[] GetRow<T>(this T[,] source, int rowIndex)
        {
            T[] row = new T[source.GetLength(1)];

            for (int i = 0; i < row.Length; i++)
            {
                row[i] = source[rowIndex, i];
            }
            return row;
        }

        public static void Fill<T>(this T[,] arr, T fill)
        {
            int h = arr.GetLength(0);
            int w = arr.GetLength(1);
            for (int y = 0; y < h; y++)
            {
                for (int x = 0; x < w; x++)
                {
                    arr[y, x] = fill;
                }
            }
        }

        public static T[,] RotatedCW<T>(this T[,] grid)
        {
            int sz = grid.GetLength(0);
            if (sz != grid.GetLength(1)) throw new System.Exception("Only square arrays can be rotated");
            T[,] tmpMap = new T[sz, sz];
            int col, nCol;
            for (int row = 0; row < sz; row++)
            {
                for (col = sz - 1, nCol = 0; col >= 0; col--, nCol++)
                {
                    tmpMap[row, nCol] = grid[col, row];
                }
            }
            return tmpMap;
        }

        public static void FlipHorizontal<T>(this T[,] grid)
        {
            int h = grid.GetLength(0);
            int w = grid.GetLength(1);

            T tmp;

            for (int y = 0; y < h; y++)
            {
                for (int x = 0; x < w / 2; x++)
                {
                    tmp = grid[y, x];
                    grid[y, x] = grid[y, w - x - 1];
                    grid[y, w - x - 1] = tmp;
                }
            }
        }

        public static T MaxVal<T>(this T[] arr) where T : IComparable
        {
            T max = arr[0];
            foreach (T t in arr)
            {
                if (t.CompareTo(max) == 1) max = t;
            }
            return max;
        }

        public static T MinVal<T>(this T[] arr) where T : IComparable
        {
            T min = arr[0];
            foreach (T t in arr)
            {
                if (t.CompareTo(min) == -1) min = t;
            }
            return min;
        }
    }
}