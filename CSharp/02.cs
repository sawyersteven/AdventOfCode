using System;
using Advent2019.IntCode;
namespace Advent2019
{
    public class Challenge02 : Challenge
    {
        Emulator ICE = new Emulator(null);
        public override object Task1()
        {
            long[] program = Tools.ParseCode(input[0]);
            program[1] = 12;
            program[2] = 2;

            ICE.Reboot(program);
            ICE.Run();

            return ICE.Memory[0];
        }

        public override object Task2()
        {
            const int reqOutput = 19690720;

            long[] intCode = IntCode.Tools.ParseCode(input[0]);

            for (int verb = 100; verb > -1; verb--)
            {
                for (int noun = 0; noun < 100; noun++)
                {
                    // long[] testCode = new long[intCode.Length];
                    // Array.Copy(intCode,)

                    ICE.Reboot(intCode);
                    ICE.Memory[1] = noun;
                    ICE.Memory[2] = verb;
                    ICE.Run();
                    if (ICE.Memory[0] == reqOutput) return (100 * noun) + verb;
                }
            }
            return null;
        }
    }
}