using System;
using System.Collections.Generic;

namespace Advent2019
{
    using Console = System.Console;
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine($":: Running challenge {args[0]} ::");

            int challengeId = int.Parse(args[0]);
            string clsName = $"Advent2019.Challenge{challengeId.ToString("00")}";

            Challenge c = (Challenge)Activator.CreateInstance(Type.GetType(clsName));
            List<string> input = new List<string>(System.IO.File.ReadAllLines($"../Input/{challengeId.ToString("00")}.txt"));
            c.Go(input);

            Console.WriteLine("Task1 -");
            Console.WriteLine($"    Time: {c.result1.Time}ms");
            Console.WriteLine($"    Result: {c.result1.Answer}");
            Console.WriteLine("Task2 -");
            Console.WriteLine($"    Time: {c.result2.Time}ms");
            Console.WriteLine($"    Result: {c.result2.Answer}");
        }
    }
}
