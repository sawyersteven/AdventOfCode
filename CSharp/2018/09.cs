using System;
using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;

namespace Advent2018
{
    public class Challenge09 : Challenge
    {
        private (int, int) gameData;
        public override void ParseInput()
        {
            string[] parts = input[0].Split(' ');
            gameData = (int.Parse(parts[0]), int.Parse(parts[parts.Length - 2]));
        }

        public override object Task1()
        {
            (int playerCount, int lastScore) = gameData;

            return PlayGame(playerCount, lastScore).MaxVal();
        }

        private long[] PlayGame(int playerCount, int lastMarble)
        {
            long[] players = new long[playerCount];

            LinkedList<int> circle = new LinkedList<int>();
            circle.AddFirst(0);
            LinkedListNode<int> currentMarble = circle.First;

            for (int marble = 1; marble < lastMarble; marble++)
            {
                LinkedListNode<int> m = new LinkedListNode<int>(marble);
                if (marble % 23 == 0)
                {
                    int player = (marble - 1) % playerCount;
                    players[player] += marble;
                    for (int i = 0; i < 7; i++)
                    {
                        currentMarble = currentMarble.Previous ?? circle.Last;
                    }
                    players[player] += currentMarble.Value;
                    currentMarble = currentMarble.Next;
                    circle.Remove(currentMarble.Previous);
                }
                else
                {
                    circle.AddAfter(currentMarble.Next ?? circle.First, m);
                    currentMarble = m;
                }
            }
            return players;
        }

        public override object Task2()
        {
            (int playerCount, int lastScore) = gameData;

            return PlayGame(playerCount, lastScore * 100).MaxVal();
        }
    }
}