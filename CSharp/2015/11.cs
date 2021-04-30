using System;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge11 : Challenge
    {
        string T1;
        public override object Task1()
        {
            char[] pw = rawInput.ToCharArray();
            Increase(pw);

            while (true)
            {
                if (IsValid(pw)) return T1 = string.Join(null, pw);
                Increase(pw);
            }
        }

        private void Increase(char[] pw)
        {
            int i = pw.Length - 1;
            pw[i]++;
            while (pw[i] > 'z')
            {
                pw[i] = 'a';
                i--;
                pw[i]++;
            }
        }


        private char[] forbidden = new char[] { 'i', 'o', 'l' };
        private bool IsValid(char[] pw)
        {
            for (int i = 0; i < pw.Length; i++)
            {
                if (Array.IndexOf(forbidden, pw[i]) != -1) return false;
            }

            bool hasSeq = false;
            for (int i = 0; i < pw.Length - 3; i++)
            {
                if (pw[i + 1] == pw[i] + 1 && pw[i + 2] == pw[i] + 2)
                {
                    hasSeq = true;
                    break;
                }
            }
            if (!hasSeq) return false;

            int pairs = 0;
            for (int i = 0; i < pw.Length - 1; i++)
            {
                if (pw[i] == pw[i + 1])
                {
                    pairs++;
                    i++;
                }
            }
            if (pairs < 2) return false;
            return true;
        }

        public override object Task2()
        {
            rawInput = T1;
            return Task1();
        }
    }
}
