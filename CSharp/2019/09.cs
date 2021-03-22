using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2019
{
    public class Challenge09 : Challenge
    {

        public override object Task1()
        {
            IntCode.Emulator ICE = new IntCode.Emulator();
            ICE.Boot(IntCode.Tools.ParseCode(input[0]));
            ICE.QueueInput(1);
            return ICE.Run().Item2;
        }

        private void Test()
        {
            IntCode.Emulator ICE = new IntCode.Emulator();

            Console.Write("Test 1: ");
            long[] code = new long[] { 109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99 };
            var response = ICE.Boot(code);
            List<long> output = new List<long>();
            while (response.Item1 != IntCode.ExitCode.Complete)
            {
                response = ICE.Run();
                output.Add(response.Item2);
            }
            output.RemoveAt(output.Count - 1);
            Console.WriteLine(string.Join(',', code) == string.Join(',', output));

            Console.Write("Test 2: ");
            code = new long[] { 1102, 34915192, 34915192, 7, 4, 7, 99, 0 };
            response = ICE.Boot(code);
            response = ICE.Run();
            Console.WriteLine(response.Item2.ToString().Length == 16);

            Console.Write("Test 3: ");
            code = new long[] { 104, 1125899906842624, 99 };
            ICE.Boot(code);
            response = ICE.Run();
            Console.WriteLine($"{response.Item2 == code[1]}");
        }

        public override object Task2()
        {
            IntCode.Emulator ICE = new IntCode.Emulator();
            ICE.Boot(IntCode.Tools.ParseCode(input[0]));
            ICE.QueueInput(2);
            return ICE.Run().Item2;
        }
    }
}