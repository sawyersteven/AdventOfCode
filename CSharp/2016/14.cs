using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge14 : Challenge
    {
        System.Security.Cryptography.MD5 md5 = System.Security.Cryptography.MD5.Create();

        public override object Task1()
        {
            Queue<string> hashQ = new Queue<string>(1000);
            string[] join = new string[6];

            int i = 0;
            for (; i < 1000; i++)
            {
                hashQ.Enqueue(CreateMD5(rawInput + i));
            }

            int validCount = 0;
            for (; ; i++)
            {
                string current = hashQ.Dequeue();
                hashQ.Enqueue(CreateMD5(rawInput + i));

                char t = FindTriple(current);
                if (t == ' ') continue;

                string quintuple = string.Join(t, join);
                foreach (string h in hashQ)
                {
                    if (h.Contains(quintuple))
                    {
                        validCount++;
                        break;
                    }
                }
                if (validCount == 64) break;
            }

            return i - 1000;
        }

        private char FindTriple(string str)
        {
            char c = ' ';
            int count = 0;
            for (int i = 0; i < str.Length; i++)
            {
                if (str[i] != c)
                {
                    c = str[i];
                    count = 1;
                }
                else
                {
                    count++;
                    if (count == 3)
                    {
                        return c;
                    }
                }
            }
            return ' ';
        }

        private string CreateMD5(string input, int iters = 1)
        {

            for (int _ = 0; _ < iters; _++)
            {
                byte[] bytes = System.Text.Encoding.ASCII.GetBytes(input);
                bytes = md5.ComputeHash(bytes);
                input = BitConverter.ToString(bytes).Replace("-", "").ToLower();
            }
            return input;
        }

        // This is painfully slow, but I blame C#'s slow MD5 implementation
        public override object Task2()
        {
            Queue<string> hashQ = new Queue<string>(1000);
            string[] join = new string[6];

            int i = 0;
            for (; i < 1000; i++)
            {
                hashQ.Enqueue(CreateMD5(rawInput + i, 2017));
            }

            int validCount = 0;
            for (; ; i++)
            {
                string current = hashQ.Dequeue();
                hashQ.Enqueue(CreateMD5(rawInput + i, 2017));

                char t = FindTriple(current);
                if (t == ' ') continue;

                string quintuple = string.Join(t, join);
                foreach (string h in hashQ)
                {
                    if (h.Contains(quintuple))
                    {
                        validCount++;
                        break;
                    }
                }
                if (validCount == 64) break;
            }

            return i - 1000;
        }
    }
}
