using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge21 : Challenge
    {
        /* I made this much more difficult than it needed to be, so I'm leaving these notes

        Since IP=5 the program can exit only when mem[5] is set higher than 31, the code len.

        Looking at the code for places where [5] is set, some are set to a hardcoded value
            using 'seti', and some have the result of a 'eqxx' or 'gtxx' added to them. These
            operations will either continue as usual [5] += 0 or skip one line [5] += 1. The
            only place this is interesting is line 30, since adding one will take us past the
            31st line of the code and halt. What's being added is the result of a 'eqrr 3 0 4'
            which is also the only place where mem[0] is read. If mem[3] == mem[0], mem[4] is
            set to 1, then mem[5] += mem[4] at line 30 and we arrive at line 32.

        So the solution is deceptively simple -- watch for line 30 then grab the value
            of mem[3]. This is what mem[0] should be set at so that this equality is
            satisfied on the first possible check.
        */

        int ip;
        (string, int[])[] code;
        public override object Task1()
        {
            ip = int.Parse(input[0].Split(' ')[1]);

            code = new (string, int[])[input.Length - 1];
            for (int i = 0; i < code.Length; i++)
            {
                (string, int[]) inst = (input[i + 1].Substring(0, 4), null);

                inst.Item2 = Array.ConvertAll(input[i + 1].Substring(5).Split(' '), int.Parse);

                code[i] = inst;
            }

            int[] mem = new int[6];
            ref int instPtr = ref mem[ip];

            while (instPtr != 30)
            {
                Challenge16.Computer.OperationsByName[code[instPtr].Item1](mem, code[instPtr].Item2);
                instPtr++;
            }
            return mem[3];
        }

        public override object Task2()
        {
            HashSet<int> results = new HashSet<int>();
            foreach (int i in ProgramPort())
            {
                if (results.Contains(i))
                {
                    break;
                }
                results.Add(i);
            }

            int[] r = new int[results.Count];
            results.CopyTo(r);
            return r[^1];
        }

        public IEnumerable<int> ProgramPort(int reg0 = 0) // appx 18,000 X faster
        {
            List<int> outs = new List<int>();
            int[] mem = new int[] { reg0, 0, 0, 0, 0, 0 };

            mem[1] = mem[3] | 65536;
            mem[3] = 9450265;

            while (true)
            {
                mem[3] += mem[1] & 255;
                mem[3] &= 16777215;
                mem[3] *= 65899;
                mem[3] &= 16777215;

                if (256 <= mem[1])
                {
                    mem[1] /= 256;
                    continue;
                }
                yield return mem[3];
                if (mem[3] == mem[0]) break;
                mem[1] = mem[3] | 65536;
                mem[3] = 9450265;
            }
        }
    }

    /*
         #ip 5                   C#                             Flow ctrl                                    
    0    seti 123 0 3            mem[3] = 123                 

    1    bani 3 456 3            mem[3] &= 456                  
    2    eqri 3 72 3             mem[3] = mem[3] == 72 ? 1: 0    
    3    addr 3 5 5              mem[5] = mem[3] + mem[5]       If mem[3] == 72 JMP to 5

    4    seti 0 _ 5              mem[5] = 0                     JMP to 1

    5    seti 0 _ 3              mem[3] = 0                     
    6    bori 3 65536 1          mem[1] = mem[3] | 65536     
    7    seti 9450265 _ 3        mem[3] = 9450265             
    8    bani 1 255 4            mem[4] = mem[1] & 255       
    9    addr 3 4 3              mem[3] = mem[3] + mem[4]
   10    bani 3 16777215 3       mem[3] &= 16777215
    1    muli 3 65899 3          mem[3] *= 65899
    2    bani 3 16777215 3       mem[3] &= 16777215

    3    gtir 256 1 4            mem[4] = 256 > mem[1] ? 1 : 0
    4    addr 4 5 5              mem[5] += mem[4]                   If mem[1] > 256, JMP to 16

    5    addi 5 1 5              mem[5] += 1                        JMP to 17
    6    seti 27 _ 5             mem[5] = 27                        JMP to 28
    7    seti 0 _ 4              mem[4] = 0

    8    addi 4 1 2             mem[2] = mem[4] + 1
    9    muli 2 256 2           mem[2] *= 256
   20    gtrr 2 1 2             mem[2] = mem[2] > mem[1] ? 1:0
    1    addr 2 5 5             mem[5] += mem[2]                    If mem[2] == 1 JMP to 23

    2    addi 5 1 5             mem[5]++                            JMP to 24
    3    seti 25 _ 5            mem[5] = 25                         JMP to 26
    4    addi 4 1 4             mem[4] += 1  
    5    seti 17 _ 5            mem[5] = 17                         JMP to 18

    6    setr 4 _ 1             mem[1] = mem[4]
    7    seti 7 _ 5             mem[5] = 7                          JMP to 8

    8    eqrr 3 0 4             mem[4] = mem[3] == mem[0] ? 1 : 0
    9    addr 4 5 5             mem[5] += mem[4]                    JMP to 31 if mem[4] == 1
   30    seti 5 _ 5             mem[5] = 5                          JMP to 6
    */
}
