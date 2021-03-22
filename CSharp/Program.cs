using System;
using System.Collections.Generic;

namespace AdventOfCode
{
    using Console = System.Console;
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length != 2)
            {
                Console.WriteLine("Usage:");
                Console.WriteLine("\tdotnet run [year] [day]");
            }

            Console.WriteLine($":: Running challenge {args[0]}.{args[1]} ::");

            int challengeId = int.Parse(args[1]);
            string clsName = $"Advent{args[0]}.Challenge{challengeId.ToString("00")}";

            Challenge c = (Challenge)Activator.CreateInstance(Type.GetType(clsName));
            c.Go(System.IO.File.ReadAllText($"../Input/{args[0]}/{challengeId.ToString("00")}.txt"));

            Console.WriteLine("Task1 -");
            Console.WriteLine($"    Time: {c.result1.Time}ms");
            Console.WriteLine($"    Result: {c.result1.Answer}");
            Console.WriteLine("Task2 -");
            Console.WriteLine($"    Time: {c.result2.Time}ms");
            Console.WriteLine($"    Result: {c.result2.Answer}");
        }
    }

    public class Utils
    {
        public static void Print2DArray<T>(T[,] grid)
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

        public static T MaxVal<T>(T[] arr) where T : IComparable
        {
            T max = arr[0];
            foreach (T t in arr)
            {
                if (t.CompareTo(max) == 1) max = t;
            }
            return max;
        }

        public static T MinVal<T>(T[] arr) where T : IComparable
        {
            T min = arr[0];
            foreach (T t in arr)
            {
                if (t.CompareTo(min) == -1) min = t;
            }
            return min;
        }

        public static bool ContentsEqual<T>(IList<T> a, IList<T> b)
        {
            if (a.Count != b.Count) return false;
            for (int i = 0; i < a.Count; i++)
            {
                if (!a[i].Equals(b[i])) return false;
            }
            return true;
        }

        public static char[,] InputToCharArray(string[] input, bool addBorder)
        {
            char[,] grid = new char[input.Length + (addBorder ? 2 : 0), input[0].Length + (addBorder ? 2 : 0)];
            int y = (addBorder ? 1 : 0);
            int yEnd = input.Length + (addBorder ? 1 : 0);
            int x = (addBorder ? 1 : 0);
            int xEnd = input[0].Length + (addBorder ? 1 : 0);

            for (int gy = 0; y < yEnd; y++, gy++)
            {
                int gx = 0;
                for (gx = 0, x = (addBorder ? 1 : 0); x < xEnd; x++, gx++)
                {
                    grid[y, x] = input[gy][gx];
                }
            }
            return grid;
        }

        public static T[] GetRow<T>(T[,] source, int rowIndex)
        {
            T[] row = new T[source.GetLength(1)];

            for (int i = 0; i < row.Length; i++)
            {
                row[i] = source[rowIndex, i];
            }
            return row;
        }
    }
}
