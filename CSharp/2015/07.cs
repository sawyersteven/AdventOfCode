using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge07 : Challenge
    {
        Dictionary<string, string> WireMap;
        public override void ParseInput()
        {
            Dictionary<string, string> map = new Dictionary<string, string>();
            foreach (string line in input)
            {
                string[] parts = line.Split(" -> ");
                map.Add(parts[1], parts[0]);
            }
            WireMap = map;
        }

        private int T1;
        public override object Task1()
        {
            return T1 = Find("a");
        }

        Dictionary<string, int> Found = new Dictionary<string, int>();
        private int Find(string outChannel)
        {
            if (Found.TryGetValue(outChannel, out int v)) return v;

            int val;

            string command = WireMap[outChannel];
            if (int.TryParse(command, out val))
            {
                Found[outChannel] = val;
                return val;
            };

            string[] parts = command.Split(' ');
            if (parts.Length == 1)
            {
                val = Find(parts[0]);
            }
            else if (parts.Length == 2)
            {
                val = ~Find(parts[1]);
            }
            else
            {

                int a = int.TryParse(parts[0], out int aa) ? aa : Find(parts[0]);
                int b = int.TryParse(parts[2], out int bb) ? bb : Find(parts[2]);
                switch (parts[1])
                {
                    case "OR": val = a | b; break;
                    case "AND": val = a & b; break;
                    case "RSHIFT": val = a >> b; break;
                    case "LSHIFT": val = a << b; break;
                    default:
                        throw new System.Exception(parts[1]);
                }
            }
            Found[outChannel] = val;
            return val;
        }

        public override object Task2()
        {
            Found.Clear();
            Found["b"] = T1;
            return Find("a");
        }
    }
}
