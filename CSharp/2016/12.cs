using AdventOfCode;

namespace Advent2016
{
    public class Challenge12 : Challenge
    {
        public override object Task1()
        {
            int[] registers = new int[4];

            RunCode(ConvertCode(input), registers);

            return registers[0];
        }

        // These things typically get reused, so some early optimization might help
        private void RunCode((char, int, bool, int, bool)[] code, int[] registers)
        {
            for (int i = 0; i < code.Length; i++)
            {
                switch (code[i].Item1)
                {
                    case 'c':
                        registers[code[i].Item4] = code[i].Item3 ? registers[code[i].Item2] : code[i].Item2;
                        break;
                    case 'i':
                        registers[code[i].Item2]++;
                        break;
                    case 'd':
                        registers[code[i].Item2]--;
                        break;
                    case 'j':
                        int nz = code[i].Item3 ? registers[code[i].Item2] : code[i].Item2;
                        if (nz != 0) i += code[i].Item4 - 1;
                        break;
                }
            }
        }

        private (char, int, bool, int, bool)[] ConvertCode(string[] code)
        {
            // cmd, out, isReg, in, isReg
            (char, int, bool, int, bool)[] converted = new (char, int, bool, int, bool)[code.Length];
            for (int i = 0; i < code.Length; i++)
            {
                var inst = ((char)0, 0, true, 0, true);
                string[] parts = code[i].Split(' ');
                inst.Item1 = parts[0][0];
                if (parts[1][0] >= 'a')
                {
                    inst.Item2 = parts[1][0] - 'a';
                    inst.Item3 = true;
                }
                else
                {
                    inst.Item2 = int.Parse(parts[1]);
                    inst.Item3 = false;
                }
                if (parts.Length > 2)
                {
                    if (parts[2][0] >= 'a')
                    {
                        inst.Item4 = parts[2][0] - 'a';
                        inst.Item5 = true;
                    }
                    else
                    {
                        inst.Item4 = int.Parse(parts[2]);
                        inst.Item5 = false;
                    }
                }
                converted[i] = inst;
            }
            return converted;
        }

        public override object Task2()
        {
            int[] registers = new int[4];
            registers[2] = 1;

            RunCode(ConvertCode(input), registers);

            return registers[0];
        }
    }
}
