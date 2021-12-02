using AdventOfCode;
using ExtensionMethods;

namespace Advent2021
{
    public class Challenge01 : Challenge
    {
        int[] inputNums;

        public override void ParseInput()
        {
            inputNums = input.ToInts();
        }

        public override object Task1()
        {
            int count = 0;
            for (int i = 1; i < inputNums.Length; i++)
            {
                if (inputNums[i] > inputNums[i - 1]) count++;
            }
            return count;
        }

        public override object Task2()
        {
            int count = 0;
            for (int i = 0; i < inputNums.Length - 3; i++)
            {
                // This is probably the right way
                // int a = inputNums[i] + inputNums[i + 1] + inputNums[i + 2];
                // int b = inputNums[i + 1] + inputNums[i + 2] + inputNums[i + 3];
                // if (b > a) count++;

                // This way is ~0.02ms faster and I love it
                int k = inputNums[i + 1] + inputNums[i + 2];
                if (k + inputNums[i + 3] > k + inputNums[i]) count++;
            }
            return count;
        }
    }
}
