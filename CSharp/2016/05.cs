using System;
using System.Collections.Generic;
using System.Text;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge05 : Challenge
    {
        System.Security.Cryptography.MD5 md5 = System.Security.Cryptography.MD5.Create();

        public override object Task1()
        {
            List<char> passcode = new List<char>();
            for (int number = 0; passcode.Count < 8; number++)
            {
                string hash = CreateMD5(rawInput + number.ToString());
                if (!hash.StartsWith("00000")) continue;

                passcode.Add(hash[5]);

            }

            return string.Join("", passcode);
        }

        private string CreateMD5(string input)
        {
            byte[] inputBytes = System.Text.Encoding.ASCII.GetBytes(input);
            return BitConverter.ToString(md5.ComputeHash(inputBytes)).Replace("-", "");

        }

        public override object Task2()
        {
            int idx = 0;

            char[] passcode = new char[8];
            int foundChars = 0;
            for (; ; idx++)
            {
                string hash = CreateMD5(rawInput + idx.ToString());
                if (!hash.StartsWith("00000")) continue;
                int ind = (int)char.GetNumericValue(hash[5]);
                if (ind > 7 || ind < 0 || passcode[ind] != 0) continue;
                bool isOk = true;
                if (isOk)
                {
                    passcode[ind] = hash[6];
                    foundChars++;
                    if (foundChars == passcode.Length) break;
                }
            }

            return string.Join("", passcode);
        }
    }
}
