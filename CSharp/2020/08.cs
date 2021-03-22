using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2020
{
    public class Challenge08 : Challenge
    {

        public override object Task1()
        {
            int acc = 0;
            int currentLine = 0;

            HashSet<int> consumedLines = new HashSet<int>();

            while (true)
            {
                if (consumedLines.Contains(currentLine)) break;
                consumedLines.Add(currentLine);
                string[] instruction = input[currentLine].Split(" ");

                switch (instruction[0])
                {
                    case "nop":
                        currentLine++;
                        break;
                    case "acc":
                        acc += int.Parse(instruction[1]);
                        currentLine++;
                        break;
                    case "jmp":
                        currentLine += int.Parse(instruction[1]);
                        break;

                    default:
                        break;
                }

            }

            return acc;
        }

        /// <summary>
        /// This works for my input. The program terminates after running the last
        /// line, and only one instruction has been changed. AoC doesn't accept the
        /// anwer that this code generates even though it meets all of the requirements.
        ///  ¯\_(ツ)_/¯
        /// </summary>
        /// 
        /// <returns></returns>
        public override object Task2()
        {
            string[] codeCopy = new string[input.Length];
            for (int i = 0; i < input.Length; i++)
            {
                string cmd = input[i];
                switch (cmd.Substring(0, 3))
                {
                    case "nop":
                        input[i] = input[i].Replace("nop", "jmp");
                        break;
                    case "jmp":
                        input[i] = input[i].Replace("jmp", "nop");
                        break;
                    default:
                        continue;
                }

                (bool ok, int acc) = ValidateCode(input);

                if (ok) return acc;
                input[i] = cmd;
            }
            return -1;
        }

        private ValueTuple<bool, int> ValidateCode(string[] code)
        {
            int acc = 0;
            int currentLine = 0;
            HashSet<int> consumedLines = new HashSet<int>();
            while (true)
            {
                if (consumedLines.Contains(currentLine)) return new ValueTuple<bool, int>(false, 0);
                if (currentLine > input.Length - 1) break;

                consumedLines.Add(currentLine);
                string[] instruction = input[currentLine].Split(" ");

                switch (instruction[0])
                {
                    case "nop":
                        currentLine++;
                        break;
                    case "acc":
                        acc += int.Parse(instruction[1]);
                        currentLine++;
                        break;
                    case "jmp":
                        currentLine += int.Parse(instruction[1]);
                        break;
                    default:
                        break;
                }
            }
            return new ValueTuple<bool, int>(true, acc);
        }
    }
}