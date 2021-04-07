using AdventOfCode;

namespace Advent2017
{
    public class Challenge17 : Challenge
    {
        public override object Task1()
        {
            int rotationSteps = int.Parse(rawInput);
            CircularLinkedList<int> circle = new CircularLinkedList<int>();
            circle.AddLast(0);
            for (int i = 1; i < 2018; i++)
            {
                circle.MoveHeadRight(rotationSteps);
                circle.AddAfter(circle.First, i);
                circle.MoveHeadRight(1);
            }

            return circle.First.Next.Value;
        }

        // There is likely a more math-oriented solution but I couldn't
        // find a pattern in any of the values
        public override object Task2()
        {
            int rotationSteps = int.Parse(rawInput);

            int zeroPos = 0;
            int secondVal = -1;
            for (int i = 1; i < 50_000_000; i++)
            {
                zeroPos = (zeroPos + rotationSteps + 1) % i;
                if (zeroPos == 0) secondVal = i;
            }
            return secondVal;
        }
    }
}
