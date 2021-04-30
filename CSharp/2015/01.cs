using AdventOfCode;

namespace Advent2015
{
    public class Challenge01 : Challenge
    {
        public override object Task1()
        {
            int t = 0;
            foreach (char c in rawInput)
            {
                t += c == '(' ? 1 : -1;
            }
            return t;
        }

        public override object Task2()
        {
            int t = 0;
            for (int i = 0; i < rawInput.Length; i++)
            {
                t += rawInput[i] == '(' ? 1 : -1;
                if (t == -1) return i + 1;
            }
            return null;
        }
    }
}
