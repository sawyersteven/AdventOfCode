using System;

namespace Advent2019
{
    public class Challenge02 : Challenge
    {
        public override object Task1()
        {
            string[] parts = input[0].Split(',');
            int[] intCode = new int[parts.Length];
            for (int i = 0; i < intCode.Length; i++) intCode[i] = int.Parse(parts[i]);

            intCode[1] = 12;
            intCode[2] = 2;

            return RunCode(intCode);
        }

        private int RunCode(int[] intCode)
        {
            for (int position = 0; position < intCode.Length; position += 4)
            {
                int opCode = intCode[position];
                switch (opCode)
                {
                    case 1:
                        int a = intCode[intCode[position + 1]];
                        int b = intCode[intCode[position + 2]];
                        intCode[intCode[position + 3]] = a + b;
                        break;
                    case 2:
                        int c = intCode[intCode[position + 1]];
                        int d = intCode[intCode[position + 2]];
                        intCode[intCode[position + 3]] = c * d;
                        break;
                    case 99:
                        return intCode[0];
                    default:
                        return -1;
                }
            }
            return -1;
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
                    if (RunCode(testCode) == reqOutput) return (100 * noun) + verb;
                }
            }


            return null;
        }
    }
}