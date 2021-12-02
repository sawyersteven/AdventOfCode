using System;

namespace ExtensionMethods
{

    public static class ArrayExtensions
    {
        public static string Join<T>(this T[,] arr, char sep)
        {
            int h = arr.GetLength(0);
            int w = arr.GetLength(1);
            T[] chars = new T[h * w];
            for (int y = 0; y < h; y++)
            {
                for (int x = 0; x < w; x++)
                {
                    chars[(y * h) + x] = arr[y, x];
                }
            }
            return string.Join(sep, chars);
        }

        public static int Count<T>(this T[,] arr, T val)
        {
            int count = 0;
            int h = arr.GetLength(0);
            int w = arr.GetLength(1);
            for (int y = 0; y < h; y++)
            {
                for (int x = 0; x < w; x++)
                {
                    if (arr[y, x].Equals(val)) count++;
                }
            }
            return count;
        }

        public static int Count<T>(this T[] arr, T val)
        {
            int count = 0;
            for (int i = 0; i < arr.Length; i++)
            {
                if (arr[i].Equals(val)) count++;
            }
            return count;
        }

        public static void Print<T>(this T[,] grid)
        {
            Console.WriteLine(grid.Render());
        }

        public static string Render<T>(this T[,] grid)
        {
            int w = grid.GetLength(1);
            int h = grid.GetLength(0);

            string[] lines = new string[h];
            for (int i = 0; i < h; i++)
            {
                string[] line = new string[w];
                for (int j = 0; j < w; j++)
                {
                    line[j] = grid[i, j].ToString();
                }
                lines[i] = string.Join(null, line);
            }
            return string.Join('\n', lines);
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