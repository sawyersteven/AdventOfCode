using System;
using Advent2019.IntCode;
using AdventOfCode;

namespace Advent2019
{
    public class Challenge02 : Challenge
    {

        public override object Task1()
        {
            long[] program = IntCode.Tools.ParseCode(input[0]);
            program[1] = 12;
            program[2] = 2;
            Emulator ICE = new Emulator(program);
            ICE.Run();

            return ICE.Memory[0];
        }

        public override object Task2()
        {
            const int reqOutput = 19690720;

            long[] intCode = IntCode.Tools.ParseCode(input[0]);
            Emulator ICE = new Emulator(intCode);

            for (int verb = 100; verb > -1; verb--)
            {
                for (int noun = 0; noun < 100; noun++)
                {
                    ICE.Memory[1] = noun;
                    ICE.Memory[2] = verb;
                    ICE.Run();
                    if (ICE.Memory[0] == reqOutput) return (100 * noun) + verb;
                    ICE.Reboot();
                }
            }
            return null;
        }
    }
}