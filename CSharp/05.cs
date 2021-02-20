using Advent2019.IntCode;
namespace Advent2019
{
    public class Challenge05 : Challenge
    {
        public override object Task1()
        {
            long[] program = IntCode.Tools.ParseCode(input[0]);

            Emulator ICE = new Emulator(program);

            (ExitCode, long) result = (0, 0);
            while (true)
            {
                (ExitCode, long) r = ICE.Run(1);
                if (r.Item1 == ExitCode.Complete) break;
                result = r;
            }

            return result.Item2;
        }

        public override object Task2()
        {
            long[] program = IntCode.Tools.ParseCode(input[0]);
            Emulator ICE = new Emulator(program);

            (ExitCode, long) result = (0, 0);
            while (true)
            {
                result = ICE.Run(5);
                if (result.Item1 == ExitCode.Complete) break;
            }

            return result.Item2;
        }
    }
}