using System;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge01 : Challenge
    {
        int[] nums;
        public override object Task1()
        {
            nums = new int[rawInput.Length];
            for (int i = 0; i < rawInput.Length; i++)
            {
                nums[i] = (int)char.GetNumericValue(rawInput[i]);
            }

            int sum = 0;
            for (int i = 0; i < nums.Length - 1; i++)
            {
                if (nums[i] == nums[i + 1]) sum += nums[i];
            }

            if (nums[0] == nums[^1]) sum += nums[0];

            return sum;
        }

        public override object Task2()
        {
            int sum = 0;
            int half = nums.Length / 2;

            for (int i = 0; i < nums.Length; i++)
            {
                if (nums[i] == nums[(i + half) % nums.Length]) sum += nums[i];
            }

            return sum;
        }
    }
}