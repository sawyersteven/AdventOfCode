using System.Collections.Generic;

namespace Advent2019
{
    public class Challenge04 : Challenge
    {

        private bool TestDoubles(string pw)
        {
            char a = pw[0];
            for (int i = 1; i < pw.Length; i++)
            {
                char b = pw[i];
                if (a == b) return true;
                a = b;
            }
            return false;
        }

        private bool TestAscending(string pw)
        {
            double a = char.GetNumericValue(pw[0]);
            for (int i = 1; i < pw.Length; i++)
            {
                double b = char.GetNumericValue(pw[i]);
                if (b < a) return false;
                a = b;
            }
            return true;
        }

        List<string> origMatches;
        public override object Task1()
        {
            origMatches = new List<string>();
            string[] parts = input[0].Split('-');

            int count = 0;
            for (int pw = int.Parse(parts[0]); pw <= int.Parse(parts[1]); pw++)
            {
                string pwString = pw.ToString();

                if (TestDoubles(pwString) && TestAscending(pwString))
                {
                    origMatches.Add(pwString);
                    count++;
                }
            }

            return count;
        }

        private bool TestDoublesT2(string pw)
        {
            int groupStart = 0;
            int groupLen = 1;

            while (groupStart + groupLen < pw.Length)
            {
                char a = pw[groupStart];
                char b = pw[groupStart + groupLen];
                if (pw[groupStart] == pw[groupStart + groupLen])
                {
                    groupLen++;
                    continue;
                }
                if (groupLen == 2)
                    return true;
                groupStart += groupLen;
                groupLen = 1;
            }
            return groupLen == 2;
        }

        public override object Task2()
        {
            string[] parts = input[0].Split('-');

            int count = 0;
            foreach (string pw in origMatches)
            {
                if (TestDoublesT2(pw) && TestAscending(pw)) count++;
            }
            return count;
        }
    }
}