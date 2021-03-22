using AdventOfCode;

namespace Advent2020
{
    public class Challenge25 : Challenge
    {
        public override object Task1()
        {
            int cardPubKey = int.Parse(input[0]);
            int doorPubKey = int.Parse(input[1]);

            return Transform(doorPubKey, LoopSize(cardPubKey));
        }

        private int LoopSize(int pubKey)
        {
            int val = 1;
            for (int loop = 1; ; loop++)
            {
                val = (val * 7) % 20201227;
                if (val == pubKey) return loop;
            }
        }

        private long Transform(int subject, int loopSize)
        {
            long val = 1;
            for (int _ = 0; _ < loopSize; _++)
            {
                val = (val * subject) % 20201227;
            }
            return val;
        }

        public override object Task2()
        {
            return "(*)";
        }
    }
}