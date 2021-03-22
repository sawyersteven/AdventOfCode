using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge16 : Challenge
    {

        public override object Task1()
        {
            int total = 0;
            for (int i = 0; i < input.Length; i += 4)
            {
                if (input[i] == string.Empty)
                {
                    break;
                }
                int[] before = ParseInts(input[i].Substring(9, input[i].Length - 1 - 9));
                int[] inst = ParseInts(input[i + 1], ' ');
                int[] after = ParseInts(input[i + 2].Substring(9, input[i].Length - 1 - 9));

                int matches = 0;
                foreach (var operation in Computer.Operations)
                {
                    int[] memCpy = before.Clone() as int[];
                    operation(memCpy, inst);

                    if (MemMatch(memCpy, after)) matches++;
                    if (matches == 3)
                    {
                        total++;
                        break;
                    }
                }
            }
            return total;
        }

        private bool MemMatch(int[] a, int[] b)
        {
            for (int i = 0; i < a.Length; i++)
            {
                if (a[i] != b[i]) return false;
            }
            return true;
        }

        private int[] ParseInts(string line, char sep = ',')
        {
            int[] arr = new int[4];
            string[] parts = line.Split(sep);
            for (int i = 0; i < parts.Length; i++)
            {
                arr[i] = int.Parse(parts[i]);
            }
            return arr;
        }

        public override object Task2()
        {
            Action<int[], int[]>[] orderedOpcodes = SortOpCodes();

            int[] mem = new int[4];
            string[] program = rawInput.Split("\r\n\r\n\r\n\r\n")[1].Split("\r\n");

            for (int i = 0; i < program.Length; i++)
            {
                int[] inst = ParseInts(program[i], ' ');
                orderedOpcodes[inst[0]](mem, inst);
            }

            return mem[0];
        }

        private Action<int[], int[]>[] SortOpCodes()
        {
            Dictionary<int, HashSet<int>> possibilities = new Dictionary<int, HashSet<int>>();
            for (int i = 0; i < Computer.Operations.Length; i++)
            {
                possibilities[i] = new HashSet<int>();
            }

            for (int i = 0; i < input.Length; i += 4)
            {
                if (input[i] == string.Empty)
                {
                    break;
                }
                int[] before = ParseInts(input[i].Substring(9, input[i].Length - 1 - 9));
                int[] inst = ParseInts(input[i + 1], ' ');
                int[] after = ParseInts(input[i + 2].Substring(9, input[i].Length - 1 - 9));

                foreach (int operation in possibilities.Keys)
                {
                    int[] memCpy = before.Clone() as int[];

                    Computer.Operations[operation](memCpy, inst);

                    if (MemMatch(memCpy, after))
                    {
                        possibilities[operation].Add(inst[0]);
                    }
                }
            }

            Action<int[], int[]>[] orderedOps = new Action<int[], int[]>[possibilities.Count];
            int[] val = new int[1];
            while (possibilities.Count > 0)
            {
                foreach (var kv in possibilities)
                {
                    if (kv.Value.Count == 1)
                    {
                        kv.Value.CopyTo(val);
                        foreach (var kv2 in possibilities)
                        {
                            if (kv2.Key == kv.Key) continue;
                            kv2.Value.Remove(val[0]);
                        }
                        orderedOps[val[0]] = Computer.Operations[kv.Key];
                        possibilities.Remove(kv.Key);
                    }
                }
            }
            return orderedOps;
        }

        private T First<T>(HashSet<T> source)
        {
            foreach (T i in source) return i;
            return default(T);
        }

        public static class Computer
        {
            private const int op = 0;
            private const int a = 1;
            private const int b = 2;
            private const int c = 3;

            public static Action<int[], int[]>[] Operations = new Action<int[], int[]>[]
            {
                (mem, inst) => mem[inst[c]] = mem[inst[a]] + mem[inst[b]],
                (mem, inst) => mem[inst[c]] = mem[inst[a]] + inst[b],
                (mem, inst) => mem[inst[c]] = mem[inst[a]] * mem[inst[b]],
                (mem, inst) => mem[inst[c]] = mem[inst[a]] * inst[b],
                (mem, inst) => mem[inst[c]] = mem[inst[a]] & mem[inst[b]],
                (mem, inst) => mem[inst[c]] = mem[inst[a]] & inst[b],
                (mem, inst) => mem[inst[c]] = mem[inst[a]] | mem[inst[b]],
                (mem, inst) => mem[inst[c]] = mem[inst[a]] | inst[b],
                (mem, inst) => mem[inst[c]] = mem[inst[a]],
                (mem, inst) => mem[inst[c]] = inst[a],
                (mem, inst) => mem[inst[c]] = inst[a] > mem[inst[b]] ? 1 : 0,
                (mem, inst) => mem[inst[c]] = mem[inst[a]] > inst[b] ? 1 : 0,
                (mem, inst) => mem[inst[c]] = mem[inst[a]] > mem[inst[b]] ? 1 : 0,
                (mem, inst) => mem[inst[c]] = inst[a] == mem[inst[b]] ? 1 : 0,
                (mem, inst) => mem[inst[c]] = mem[inst[a]] == inst[b]? 1 : 0,
                (mem, inst) => mem[inst[c]] = mem[inst[a]] == mem[inst[b]]? 1 : 0,
            };

            public static Dictionary<string, Action<int[], int[]>> OperationsByName = new Dictionary<string, Action<int[], int[]>>
            {
                {"addr", (mem, inst) => mem[inst[b]] = mem[inst[op]] + mem[inst[a]]},
                {"addi", (mem, inst) => mem[inst[b]] = mem[inst[op]] + inst[a]},
                {"mulr", (mem, inst) => mem[inst[b]] = mem[inst[op]] * mem[inst[a]]},
                {"muli", (mem, inst) => mem[inst[b]] = mem[inst[op]] * inst[a]},
                {"banr", (mem, inst) => mem[inst[b]] = mem[inst[op]] & mem[inst[a]]},
                {"bani", (mem, inst) => mem[inst[b]] = mem[inst[op]] & inst[a]},
                {"borr", (mem, inst) => mem[inst[b]] = mem[inst[op]] | mem[inst[a]]},
                {"bori", (mem, inst) => mem[inst[b]] = mem[inst[op]] | inst[a]},
                {"setr", (mem, inst) => mem[inst[b]] = mem[inst[op]]},
                {"seti", (mem, inst) => mem[inst[b]] = inst[op]},
                {"gtir", (mem, inst) => mem[inst[b]] = inst[op] > mem[inst[a]] ? 1 : 0},
                {"gtri", (mem, inst) => mem[inst[b]] = mem[inst[op]] > inst[a] ? 1 : 0},
                {"gtrr", (mem, inst) => mem[inst[b]] = mem[inst[op]] > mem[inst[a]] ? 1 : 0},
                {"eqir", (mem, inst) => mem[inst[b]] = inst[op] == mem[inst[a]] ? 1 : 0},
                {"eqri", (mem, inst) => mem[inst[b]] = mem[inst[op]] == inst[a]? 1 : 0},
                {"eqrr", (mem, inst) => mem[inst[b]] = mem[inst[op]] == mem[inst[a]]? 1 : 0},
            };
        }
    }
}