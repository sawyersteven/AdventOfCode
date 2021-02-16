using System;

namespace Advent2019
{
    public class Challenge02 : Challenge
    {
        IntCodeEmulator ICE = new IntCodeEmulator();
        public override object Task1()
        {
            string[] parts = input[0].Split(',');
            int[] intCode = new int[parts.Length];
            for (int i = 0; i < intCode.Length; i++) intCode[i] = int.Parse(parts[i]);

            intCode[1] = 12;
            intCode[2] = 2;

            ICE.Run(intCode);
            return intCode[0];
        }

        public override object Task2()
        {

            const int reqOutput = 19690720;

            string[] parts = input[0].Split(',');
            int[] intCode = new int[parts.Length];
            for (int i = 0; i < intCode.Length; i++) intCode[i] = int.Parse(parts[i]);

            int[] testCode = new int[intCode.Length];

            for (int noun = 0; noun < 100; noun++)
            {
                for (int verb = 0; verb < 100; verb++)
                {
                    Array.Copy(intCode, testCode, intCode.Length);
                    testCode[1] = noun;
                    testCode[2] = verb;
                    ICE.Run(testCode);
                    if (testCode[0] == reqOutput) return (100 * noun) + verb;
                }
            }
            return null;
        }
    }
}