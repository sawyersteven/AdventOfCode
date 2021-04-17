using AdventOfCode;
using System;

namespace Advent2016
{
    public class Challenge03 : Challenge
    {
        public override object Task1()
        {
            int count = 0;
            foreach (string line in input)
            {
                int[] nums = Array.ConvertAll(line.Split(' ', StringSplitOptions.RemoveEmptyEntries), int.Parse);
                Array.Sort(nums);
                if (nums[0] + nums[1] > nums[2]) count++;
            }

            return count;
        }

        public override object Task2()
        {
            int count = 0;
            int[] tri = new int[3];
            for (int i = 0; i < input.Length; i += 3)
            {
                int[] numsA = Array.ConvertAll(input[i].Split(' ', StringSplitOptions.RemoveEmptyEntries), int.Parse);
                int[] numsB = Array.ConvertAll(input[i + 1].Split(' ', StringSplitOptions.RemoveEmptyEntries), int.Parse);
                int[] numsC = Array.ConvertAll(input[i + 2].Split(' ', StringSplitOptions.RemoveEmptyEntries), int.Parse);

                for (int j = 0; j < 3; j++)
                {
                    tri[0] = numsA[j];
                    tri[1] = numsB[j];
                    tri[2] = numsC[j];
                    Array.Sort(tri);
                    if (tri[0] + tri[1] > tri[2]) count++;
                }
            }
            return count;
        }
    }
}
