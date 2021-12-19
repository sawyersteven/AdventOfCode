using System;
using System.Text.RegularExpressions;
using AdventOfCode;

namespace Advent2021
{
    public class Challenge18 : Challenge
    {
        /*
         The "proper" way to do this is probably to parse all of the input
        lines into a tree and do the required explode/split functions on the
        tree node. But I'm tired and this requires less thought right now.
        I'll probably come back to this later and do it the right way, but
        this seems speedy enough and it works.
        */


        private string Reduce(string snailNum)
        {
            string reduced;
            while (true)
            {
                reduced = Explode(snailNum);
                if (reduced.Length != snailNum.Length)
                {
                    snailNum = reduced;
                    continue;
                }
                reduced = Split(reduced);
                if (reduced.Length == snailNum.Length) break;
                snailNum = reduced;
            }

            return snailNum;
        }

        private bool IsNum(char c)
        {
            return c <= '9' && c >= '0';
        }

        private string Split(string snailNum)
        {
            for (int numStart = 0; numStart < snailNum.Length; numStart++)
            {
                if (IsNum(snailNum[numStart]) && IsNum(snailNum[numStart + 1]))
                {
                    int n = int.Parse(snailNum.Substring(numStart, 2));
                    string outPair = $"[{n / 2},{n - n / 2}]";
                    snailNum = snailNum.Substring(0, numStart) + outPair + snailNum.Substring(numStart + 2);
                    break;
                }
            }
            return snailNum;
        }

        // I'm not proud of this
        private string Explode(string snailNum)
        {
            int depth = 0;
            for (int pairStart = 0; pairStart < snailNum.Length; pairStart++)
            {
                if (snailNum[pairStart] == '[') depth++;
                else if (snailNum[pairStart] == ']') depth--;

                if (depth > 4 && IsNum(snailNum[pairStart]))
                {
                    int pairEnd = pairStart;
                    while (snailNum[pairEnd] != ']') pairEnd++;

                    int[] nums = Array.ConvertAll(snailNum.Substring(pairStart, pairEnd - pairStart).Split(','), int.Parse);

                    // find left num
                    (int, int, string) lRepl = (0, 0, null);
                    int lNumEnd = pairStart - 1;
                    int lNumStart = -1;
                    for (; lNumEnd >= 0; lNumEnd--)
                    {
                        if (IsNum(snailNum[lNumEnd]))
                        {
                            lNumStart = lNumEnd;
                            for (; lNumStart >= 0; lNumStart--)
                            {
                                if (!IsNum(snailNum[lNumStart]))
                                {
                                    lNumStart++;
                                    int ln = int.Parse(snailNum.Substring(lNumStart, lNumEnd - lNumStart + 1));
                                    ln += nums[0];
                                    lRepl.Item1 = lNumStart;
                                    lRepl.Item2 = lNumEnd + 1;
                                    lRepl.Item3 = ln.ToString();
                                    // found num
                                    break;
                                }
                            }
                            break;
                        }
                    };

                    // find right num
                    (int, int, string) rRepl = (0, 0, null);
                    int rNumEnd = -1;
                    int rNumStart = pairEnd;
                    for (; rNumStart < snailNum.Length; rNumStart++)
                    {
                        if (IsNum(snailNum[rNumStart]))
                        {
                            rNumEnd = rNumStart + 1;
                            for (; rNumEnd < snailNum.Length; rNumEnd++)
                            {
                                if (!IsNum(snailNum[rNumEnd]))
                                {
                                    rNumEnd--;
                                    // found num;
                                    int rn = int.Parse(snailNum.Substring(rNumStart, rNumEnd - rNumStart + 1));
                                    rn += nums[1];
                                    rRepl.Item1 = rNumStart;
                                    rRepl.Item2 = rNumEnd + 1;
                                    rRepl.Item3 = rn.ToString();
                                    break;
                                }
                            }
                            break;
                        }
                    }
                    if (rRepl.Item3 != null)
                    {
                        snailNum = snailNum.Substring(0, rRepl.Item1) + rRepl.Item3 + snailNum.Substring(rRepl.Item2);
                    }
                    // replace pair with 0
                    snailNum = snailNum.Substring(0, pairStart - 1) + "0" + snailNum.Substring(pairEnd + 1);
                    if (lRepl.Item3 != null)
                    {
                        snailNum = snailNum.Substring(0, lRepl.Item1) + lRepl.Item3 + snailNum.Substring(lRepl.Item2);
                    }
                    return snailNum;
                }
            }
            return snailNum;
        }



        public override object Task1()
        {
            string snailNum = input[0];

            for (int i = 1; i < input.Length; i++)
            {
                snailNum = $"[{snailNum},{input[i]}]";
                snailNum = Reduce(snailNum);
            }

            return Magnitude(snailNum);
        }

        private Regex reg = new Regex(@"\[\d*,\d*]", RegexOptions.Compiled);
        private long Magnitude(string snailNum)
        {
            while (true)
            {
                Match m = reg.Match(snailNum);
                string s = m.Value;
                if (string.IsNullOrEmpty(s)) break;
                int[] ints = Array.ConvertAll(s.Substring(1, s.Length - 2).Split(','), int.Parse);
                int mag = (3 * ints[0]) + (2 * ints[1]);
                snailNum = snailNum.Replace(s, mag.ToString());
            }
            return long.Parse(snailNum);
        }

        public override object Task2()
        {
            long bestMag = 0;
            string snailNum;
            long mag;
            for (int i = 0; i < input.Length - 1; i++)
            {
                for (int j = i + 1; j < input.Length; j++)
                {
                    snailNum = $"[{input[i]},{input[j]}]";
                    mag = Magnitude(Reduce(snailNum));
                    if (mag > bestMag) bestMag = mag;


                    snailNum = $"[{input[j]},{input[i]}]";
                    mag = Magnitude(Reduce(snailNum));
                    if (mag > bestMag) bestMag = mag;

                }
            }
            return bestMag;
        }
    }
}
