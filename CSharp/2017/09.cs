using AdventOfCode;

namespace Advent2017
{
    public class Challenge09 : Challenge
    {
        public override object Task1()
        {
            long score = 0;
            int depth = 0;

            bool inGarbage = false;
            for (int i = 0; i < rawInput.Length; i++)
            {
                char c = rawInput[i];
                if (c == '!')
                {
                    i++;
                    continue;
                }

                if (inGarbage)
                {
                    if (c == '>') inGarbage = false;
                    continue;
                }

                if (c == '<') inGarbage = true;

                if (c == '{') depth++;
                if (c == '}')
                {

                    score += depth;
                    depth--;
                }

            }

            return score;
        }

        public override object Task2()
        {
            int garbCount = 0;

            bool inGarbage = false;
            for (int i = 0; i < rawInput.Length; i++)
            {
                char c = rawInput[i];
                if (c == '!')
                {
                    i++;
                    continue;
                }

                if (inGarbage)
                {
                    if (c == '>') inGarbage = false;
                    else garbCount++;
                    continue;
                }
                if (c == '<') inGarbage = true;
            }


            return garbCount;
        }
    }
}
