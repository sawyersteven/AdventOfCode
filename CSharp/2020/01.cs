using AdventOfCode;

namespace Advent2020
{
    public class Challenge01 : Challenge
    {
        private int[] expenses;

        public override object Task1()
        {
            expenses = new int[input.Length];

            for (int i = 0; i < input.Length; i++) expenses[i] = int.Parse(input[i]);

            for (int i = 0; i < expenses.Length; i++)
            {
                int a = expenses[i];
                for (int j = i + 1; j < expenses.Length; j++)
                {
                    if (a + expenses[j] == 2020)
                    {
                        return (a * expenses[j]);
                    }
                }
            }
            return null;
        }

        public override object Task2()
        {
            for (int i = 0; i < expenses.Length; i++)
            {
                int a = expenses[i];
                for (int j = i + 1; j < expenses.Length; j++)
                {
                    int b = expenses[j];
                    for (int k = j + 1; k < expenses.Length; k++)
                    {
                        if (a + b + expenses[k] == 2020)
                        {
                            return (a * b * expenses[k]);
                        }
                    }
                }
            }
            return null;
        }
    }
}