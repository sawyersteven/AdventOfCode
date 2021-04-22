using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge19 : Challenge
    {
        // Jospehus algo
        // https://en.wikipedia.org/wiki/Josephus_problem#The_general_case
        public override object Task1()
        {
            int numElves = int.Parse(rawInput);

            int last = 0;
            for (int i = 1; i <= numElves; i++)
            {
                last = (last + 2) % (i + 1);
            }
            return last - 1;
        }

        /* Rather than keep the head on the elf who's turn it is, it is faster
        and easier to keep track of which elf is being eliminated        
        */
        public override object Task2()
        {
            int numElves = int.Parse(rawInput);
            CircularLinkedList<int> elves = new CircularLinkedList<int>();
            for (int i = 1; i <= numElves; i++)
            {
                elves.AddLast(i);
            }

            elves.MoveHeadRight(numElves / 2);

            while (elves.Count > 1)
            {
                elves.Remove(elves.First);
                if (elves.Count % 2 == 0)
                {
                    elves.MoveHeadRight(1);
                }
            }
            return elves.First.Value;
        }
    }
}

