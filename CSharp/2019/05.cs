using AdventOfCode;
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
                ICE.QueueInput(1);
                (ExitCode, long) r = ICE.Run();
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
                ICE.QueueInput(5);
                result = ICE.Run();
                if (result.Item1 == ExitCode.Complete) break;
            }

            return result.Item2;
        }
    }
}