using System.Collections.Generic;
using AdventOfCode;

namespace Advent2019
{
    public class Challenge21 : Challenge
    {

        /*   /-\
            @   a
        #####ABCD########
        #########EFGHI###
        */
        public override object Task1()
        {
            string[] springScript = new string[]{
                "NOT B J",
                "NOT C T",
                "OR T J",
                "AND D J",
                "NOT A T",
                "OR T J",
                "WALK",
            };

            List<long> output = RunSpringScript(springScript);

            if (output[^1] > 128) return output[^1];

            char[] errReport = new char[output.Count];
            for (int i = 0; i < errReport.Length; i++)
            {
                errReport[i] = (char)output[i];
            }
            System.Console.WriteLine(string.Join("", errReport));

            return null;
        }

        private List<long> RunSpringScript(string[] script)
        {
            IntCode.Emulator ICE = new IntCode.Emulator(rawInput);

            List<long> outBuffer = new List<long>();

            while (true)
            {
                var resp = ICE.Run();
                if (resp.Item1 == IntCode.ExitCode.OutputDelivery)
                {
                    outBuffer.Add((char)resp.Item2);
                }
                else break;
            }

            foreach (string line in script)
            {
                SendString(ICE, line);
            }

            outBuffer.Clear();
            while (true)
            {
                var resp = ICE.Run();
                if (resp.Item1 == IntCode.ExitCode.OutputDelivery)
                {
                    outBuffer.Add(resp.Item2);
                }
                else break;
            }

            return outBuffer;
        }

        private void SendString(IntCode.Emulator ICE, string instructions)
        {
            long[] inst = new long[instructions.Length + 1];
            for (int i = 0; i < instructions.Length; i++)
            {
                inst[i] = instructions[i];
            }
            inst[^1] = (char)10;
            ICE.QueueInput(inst);
        }

        public override object Task2()
        {
            // Same as T1 script but with added line "AND H J"
            string[] springScript = new string[]{
                "NOT B J",
                "NOT C T",
                "OR T J",
                "AND D J",
                "AND H J",
                "NOT A T",
                "OR T J",
                "RUN",
            };

            List<long> output = RunSpringScript(springScript);

            if (output[^1] > 128) return output[^1];

            char[] errReport = new char[output.Count];
            for (int i = 0; i < errReport.Length; i++)
            {
                errReport[i] = (char)output[i];
            }
            System.Console.WriteLine(string.Join("", errReport));

            return null;
        }
    }
}