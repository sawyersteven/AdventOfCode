using AdventOfCode;

namespace Advent2016
{
    public class Challenge25 : Challenge
    {
        private const int forever = 1000;

        public override object Task1()
        {
            var code = Challenge12.ConvertCode(input);

            int a = 0;
            for (; a < forever; a++)
            {
                int[] registers = new int[] { a, 0, 0, 0 };
                int i = 0;
                int toggle = 0;
                var c = code.Clone() as (char, int, bool, int, bool)[];
                foreach (int o in Challenge12.RunCode(code, registers))
                {
                    if (o != toggle) break;
                    toggle = toggle == 0 ? 1 : 0;

                    if (i == forever) return a;
                    i++;
                }
            }
            return null;
        }

        public override object Task2()
        {
            return '*';
        }
    }
}
