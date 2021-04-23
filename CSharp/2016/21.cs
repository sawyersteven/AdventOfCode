using System;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge21 : Challenge
    {
        public override object Task1()
        {
            char[] password = "abcdefgh".ToCharArray();
            return string.Join(null, Scramble(password));
        }

        private char[] Scramble(char[] password, bool reverse = false)
        {
            for (int i = 0; i < input.Length; i++)
            {
                string line = input[i];
                string[] parts = line.Split(' ');
                switch (parts[0])
                {
                    case "swap":
                        int x = 0;
                        int y = 0;
                        if (parts[1] == "position")
                        {
                            x = int.Parse(parts[2]);
                            y = int.Parse(parts[5]);
                        }
                        else
                        {
                            x = Array.IndexOf(password, parts[2][0]);
                            y = Array.IndexOf(password, parts[5][0]);
                        }
                        password[x] += password[y];
                        password[y] = (char)(password[x] - password[y]);
                        password[x] -= password[y];
                        break;

                    case "reverse":

                        int start = int.Parse(parts[2]);
                        int end = int.Parse(parts[4]);
                        Array.Reverse(password, start, end - start + 1);
                        break;

                    case "rotate":
                        int by = 0;
                        if (parts[1] == "based")
                        {
                            by = Array.IndexOf(password, parts[6][0]);
                            if (reverse)
                            {
                                if (by % 2 == 1) by -= (by / 2);
                                else by = 5 + (by / 2);
                                if (by == 5) by = 9;
                                by *= -1;
                            }
                            else
                            {
                                if (by >= 4) by++;
                                by++;
                            }
                        }
                        else
                        {
                            by = int.Parse(parts[2]) * (parts[1] == "left" ? -1 : 1);
                            if (reverse) by *= -1;
                        }
                        RotateR(password, by);
                        break;

                    case "move":
                        int from = int.Parse(parts[2]);
                        int to = int.Parse(parts[5]);

                        if (reverse) (to, from) = (from, to);

                        char m = password[from];
                        if (from > to)
                        {
                            ShiftRange(password, to, from - to, 1);
                        }
                        else
                        {
                            ShiftRange(password, from + 1, to - from, -1);
                        }
                        password[to] = m;
                        break;
                }
            }
            return password;
        }

        private void ShiftRange<T>(T[] arr, int start, int length, int distance)
        {
            T[] tmp = new T[length];
            Array.Copy(arr, start, tmp, 0, length);
            Array.Copy(tmp, 0, arr, start + distance, tmp.Length);
        }

        private void RotateR<T>(T[] arr, int by)
        {
            by = (by + arr.Length + arr.Length) % arr.Length;
            T[] buffer = new T[by];
            Array.Copy(arr, arr.Length - by, buffer, 0, by);
            Array.Copy(arr, 0, arr, by, arr.Length - by);
            Array.Copy(buffer, 0, arr, 0, by);
        }

        public override object Task2()
        {
            char[] scrambled = "fbgdceah".ToCharArray();
            Array.Reverse(input);
            return string.Join(null, Scramble(scrambled, true));
        }
    }
}
