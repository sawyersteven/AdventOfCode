using Advent2019.IntCode;
namespace Advent2019
{
    public class Challenge05 : Challenge
    {
        public override object Task1()
        {
            int[] program = IntCode.Tools.ParseCode(input[0]);

            Emulator ICE = new Emulator(program);

            (ExitCode, int) result = (0, 0);
            while (true)
            {
                (ExitCode, int) r = ICE.Run(1);
                if (r.Item1 == ExitCode.Complete) break;
                result = r;
            }

            return result.Item2;
        }

        public override object Task2()
        {
            int[] program = IntCode.Tools.ParseCode(input[0]);
            Emulator ICE = new Emulator(program);

            (ExitCode, int) result = (0, 0);
            while (true)
            {
                (ExitCode, int) r = ICE.Run(5);
                if (r.Item1 == ExitCode.Complete) break;
                result = r;
            }

            return result.Item2;
        }
    }
}