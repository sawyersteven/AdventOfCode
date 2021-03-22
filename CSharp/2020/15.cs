using AdventOfCode;
using System.Collections.Generic;

namespace Advent2020
{
    public class Challenge15 : Challenge
    {

        public override object Task1()
        {
            const int TURNS = 2020;

            List<int> numHistory = new List<int>();

            foreach (string i in input[0].Split(','))
            {
                numHistory.Add(int.Parse(i));
            }

            HashSet<int> spokenNums = new HashSet<int>(numHistory);

            int nextNum = 0;

            while (numHistory.Count < TURNS)
            {
                // if num is new
                if (!spokenNums.Contains(nextNum))
                {
                    numHistory.Add(nextNum);
                    spokenNums.Add(nextNum);
                    nextNum = 0;
                    continue;
                }
                numHistory.Add(nextNum);
                spokenNums.Add(nextNum);

                int ft = LastTurnOf(numHistory, nextNum);
                nextNum = numHistory.Count - ft;

            }
            return numHistory[numHistory.Count - 1];
        }

        private int LastTurnOf(List<int> collection, int member)
        {
            for (int i = collection.Count - 2; i >= 0; i--)
            {
                if (collection[i] == member) return i + 1;
            }
            return -1;
        }

        // This still takes slightly over 2 seconds... might be room for optimization somewhere.
        public override object Task2()
        {
            const uint TURNS = 30000000;

            // <num, lastUse>
            Dictionary<uint, uint> numberHistory = new Dictionary<uint, uint>();

            string[] parts = input[0].Split(',');
            for (uint i = 0; i < parts.Length; i++)
            {
                numberHistory[uint.Parse(parts[i])] = i + 1;
            }

            uint prevNum = 0;
            uint nextNum = 0;
            uint currentTurn = (uint)parts.Length + 1;

            while (currentTurn < TURNS)
            {
                prevNum = nextNum;
                if (!numberHistory.ContainsKey(nextNum))
                {
                    nextNum = 0;
                }
                else
                {
                    nextNum = currentTurn - numberHistory[nextNum];
                }

                numberHistory[prevNum] = currentTurn;
                currentTurn++;
            }

            return nextNum;
        }
    }
}