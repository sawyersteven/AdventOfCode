using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2020
{
    public class Challenge22 : Challenge
    {

        private (Queue<int>, Queue<int>) ParseInput()
        {
            Queue<int> P1 = new Queue<int>();
            Queue<int> P2 = new Queue<int>();

            for (int line = 1; line < Array.IndexOf(input, string.Empty); line++)
            {
                P1.Enqueue(int.Parse(input[line]));
            }
            for (int line = Array.IndexOf(input, string.Empty) + 2; line < input.Length; line++)
            {
                P2.Enqueue(int.Parse(input[line]));
            }
            return (P1, P2);
        }

        private int TallyScore(Queue<int> hand)
        {
            int answer = 0;
            int multiplier = hand.Count;
            foreach (int card in hand)
            {
                answer += card * multiplier;
                multiplier--;
            }
            return answer;
        }


        const bool Player1 = true;
        const bool Player2 = false;

        public override object Task1()
        {
            (Queue<int> P1, Queue<int> P2) = ParseInput();

            while (P1.Count != 0 && P2.Count != 0)
            {
                int p1card = P1.Dequeue();
                int p2card = P2.Dequeue();

                if (p1card > p2card)
                {
                    P1.Enqueue(p1card);
                    P1.Enqueue(p2card);
                }
                else
                {
                    P2.Enqueue(p2card);
                    P2.Enqueue(p1card);
                }
            }

            return TallyScore(P1.Count == 0 ? P2 : P1);
        }

        public override object Task2()
        {
            (Queue<int> P1, Queue<int> P2) = ParseInput();
            T2PlayGame(P1, P2);
            return TallyScore(P1.Count == 0 ? P2 : P1);
            // 33469
        }

        private Queue<T> PartialCopyQueue<T>(Queue<T> donor, int count)
        {
            Queue<T> n = new Queue<T>();
            int ind = 0;
            foreach (T d in donor)
            {
                n.Enqueue(d);
                ind++;
                if (ind == count) break;
            }
            return n;
        }

        private bool T2PlayGame(Queue<int> P1, Queue<int> P2)
        {
            HashSet<string> turnHistory = new HashSet<string>();
            while (true)
            {
                string uid = string.Join(' ', P1) + '/' + string.Join(' ', P2);
                if (turnHistory.Contains(uid)) return Player1;
                turnHistory.Add(uid);

                int p1card = P1.Dequeue();
                int p2card = P2.Dequeue();

                bool roundWinner;

                if (p1card <= P1.Count && p2card <= P2.Count)
                {
                    roundWinner = T2PlayGame(PartialCopyQueue(P1, p1card), PartialCopyQueue(P2, p2card));
                }
                else
                {
                    roundWinner = p1card > p2card;
                }

                if (roundWinner)
                {
                    P1.Enqueue(p1card);
                    P1.Enqueue(p2card);
                    if (P2.Count == 0) return Player1;
                }
                else
                {
                    P2.Enqueue(p2card);
                    P2.Enqueue(p1card);
                    if (P1.Count == 0) return Player2;
                }
            }
        }
    }
}
