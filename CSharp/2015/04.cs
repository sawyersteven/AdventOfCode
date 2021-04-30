using AdventOfCode;

namespace Advent2015
{
    public class Challenge04 : Challenge
    {
        public override object Task1()
        {
            for (int i = 0; ; i++)
            {
                string d = MD5.HexDigest(rawInput + i);
                if (d.StartsWith("00000")) return i;
            }
        }

        public override object Task2()
        {
            for (int i = 0; ; i++)
            {
                string d = MD5.HexDigest(rawInput + i);
                if (d.StartsWith("000000")) return i;
            }
        }
    }
}
