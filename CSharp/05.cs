namespace Advent2019
{
    public class Challenge05 : Challenge
    {
        IntCodeEmulator ICE = new IntCodeEmulator();
        public override object Task1()
        {
            ICE.Run(input[0], 1);

            for (int i = 0; i < ICE.output.Count - 1; i++)
            {
                if (ICE.output[i] != 0) return "FailedDiag";
            }

            return ICE.output[ICE.output.Count - 1];
        }

        public override object Task2()
        {
            ICE.Run(input[0], 5);

            for (int i = 0; i < ICE.output.Count - 1; i++)
            {
                if (ICE.output[i] != 0) return "FailedDiag";
            }

            return ICE.output[ICE.output.Count - 1];
        }
    }
}