using System;
using Advent2019.IntCode;
namespace Advent2019
{
    public class Challenge02 : Challenge
    {
        Emulator ICE = new Emulator(null);
        public override object Task1()
        {
            int[] program = Tools.ParseCode(input[0]);
            program[1] = 12;
            program[2] = 2;

            ICE.Reboot(program);
            ICE.Run();

            return ICE.program[0];
        }

        public override object Task2()
        {
            const int reqOutput = 19690720;

            string[] parts = input[0].Split(',');
            int[] intCode = new int[parts.Length];
            for (int i = 0; i < intCode.Length; i++) intCode[i] = int.Parse(parts[i]);

            int[] testCode = new int[intCode.Length];

            for (int verb = 100; verb > -1; verb--)
            {
                for (int noun = 0; noun < 100; noun++)
                {
                    Array.Copy(intCode, testCode, intCode.Length);
                    testCode[1] = noun;
                    testCode[2] = verb;
                    ICE.Reboot(testCode);
                    ICE.Run();
                    if (ICE.program[0] == reqOutput) return (100 * noun) + verb;
                }
            }
            return null;
        }
    }
}