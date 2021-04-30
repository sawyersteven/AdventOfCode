using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge24 : Challenge
    {
        public override object Task1()
        {
            ulong[] nums = Array.ConvertAll(input, ulong.Parse);
            ulong targetWeight = Sum(nums) / 3;

            ulong lowest = ulong.MaxValue;
            foreach (IList<ulong> wc in GenWeightCombos(targetWeight, nums))
            {
                ulong l = Mul(wc);
                if (l < lowest) lowest = l;

            }
            return lowest;
        }

        private List<IList<ulong>> GenWeightCombos(ulong targetWeight, ulong[] nums)
        {
            List<IList<ulong>> weightCombos = new List<IList<ulong>>();
            for (int i = 2; i < input.Length; i++)
            {
                foreach (var wc in Permutator.Combinations(i, nums))
                {
                    if (Sum(wc) == targetWeight)
                    {
                        weightCombos.Add(wc);
                    }
                }
                if (weightCombos.Count != 0) break;
            }
            return weightCombos;
        }

        private ulong Mul(IEnumerable<ulong> arr)
        {
            ulong t = 1;
            foreach (ulong i in arr) t *= i;
            return t;
        }

        private ulong Sum(IEnumerable<ulong> arr)
        {
            ulong t = 0;
            foreach (ulong i in arr) t += i;
            return t;
        }

        public override object Task2()
        {
            ulong[] nums = Array.ConvertAll(input, ulong.Parse);
            ulong targetWeight = Sum(nums) / 4;

            ulong lowest = ulong.MaxValue;
            foreach (IList<ulong> wc in GenWeightCombos(targetWeight, nums))
            {
                ulong l = Mul(wc);
                if (l < lowest) lowest = l;

            }
            return lowest;
        }
    }
}
