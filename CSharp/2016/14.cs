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
            byte[] bytes = System.Text.Encoding.ASCII.GetBytes(input);
            for (int _ = 0; _ < iters; _++)
            {
                bytes = ToLowercaseHexBytes(md5.ComputeHash(bytes));
            }
            return System.Text.Encoding.ASCII.GetString(bytes);
        }

        private byte[] ToLowercaseHexBytes(byte[] hash)
        {
            byte[] ret = new byte[32];
            for (int i = 0, j = 0; i < hash.Length; i++, j += 2)
            {
                ret[j] = (byte)((hash[i] / 16) + 48);
                // some bit-fiddling to see if the value is over 57 so it can have
                // 39 added to it in order to get a lower-case ascii letter value
                // This only works on unsigned values.
                ret[j] += (byte)((57 + (~ret[j] + 1) >> 31 & 1) * 39);
                ret[j + 1] = (byte)((hash[i] % 16) + 48);
                ret[j + 1] += (byte)((57 + (~ret[j + 1] + 1) >> 31 & 1) * 39);
            }
            return ret;
        }

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
