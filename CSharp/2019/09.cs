using AdventOfCode;

namespace Advent2019
{
    public class Challenge09 : Challenge
    {

        public override object Task1()
        {
            IntCode.Emulator ICE = new IntCode.Emulator(input[0]);
            ICE.QueueInput(1);
            return ICE.Run().Item2;
        }

        public override object Task2()
        {
            IntCode.Emulator ICE = new IntCode.Emulator(input[0]);
            ICE.QueueInput(2);
            return ICE.Run().Item2;
        }
    }
}