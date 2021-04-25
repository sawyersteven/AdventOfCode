using AdventOfCode;

namespace Advent2016
{
    public class Challenge23 : Challenge
    {
        public override object Task1()
        {
            var code = Challenge12.ConvertCode(input);
            int[] registers = new int[4];
            registers[0] = 7;
            Challenge12.RunCode(code, registers);
            return registers[0];
        }

        public override object Task2()
        {
            var code = Challenge12.ConvertCode(input);

            int answer = 12;
            for (int i = 1; i < 12; i++)
            {
                answer *= i;
            }
            answer += (code[19].Item2 * code[20].Item2);
            return answer;
        }
    }
}
