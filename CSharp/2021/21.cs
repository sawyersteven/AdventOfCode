using System.Collections.Generic;
using AdventOfCode;

namespace Advent2021
{
    public class Challenge21 : Challenge
    {
        private const int BOARDSIZE = 10;
        private const int MINROLL = 3;
        private const int MAXROLL = 9;
        private const int WINSCORE1 = 1000;
        private const int WINSCORE2 = 21;

        private int p1Start;
        private int p2Start;
        public override void ParseInput()
        {
            p1Start = int.Parse(input[0].Split(' ')[^1]);
            p2Start = int.Parse(input[1].Split(' ')[^1]);
        }

        /* Part 1 is fairly trivial but part 2 gave me some trouble when I wrote
        a solution that only split the universes after each turn rather than
        after each roll. After realizing that mistake the solution was a bit
        easier to discover.

        rollMultiples indicates how many permutations of the 3-sided die result
        in any given total.

        I start by checking every possible roll combination for each player until
        that player reaches the winning score. Meanwhile the number of universes
        is tallied in the State.Splits field by adding the number of permutations
        tha lead to the current roll. Eg rolling 3 can only happen in one way and
        only creates one new universe. Rolling a 5 can happen in 6 different ways
        so six universes can be created and converge at this same state.

        If any given combination of dice rolls leads to the player winning the
        total number of universes that converge to this particular state are
        added to the winning tally for that game length. So a game that finds
        the player winning in 4 turns with 42 universes converging on that
        state will add 42 to that 4's win column. If a state doesn't result
        in the player winning, the universes converging on that state will be
        added to the loss column.

        Player 1 can win before player 2 is able to roll for that turn. This
        means every combination of wins for player one after N turns will
        beat every combination of losses for player two after N-1 turns. So
        player1's score is a total of (p1[N][wins] * p2[N-1][losses]) for
        each game length (aka N) that player one reaches.

        Player 2 can only win if player1 does not win on that same turn. So
        their score will be the total of (p2[N][wins] * p1[N][losses]) for
        every value of N.
        */

        public override object Task1()
        {
            (int, int) p1 = (p1Start, 0);
            (int, int) p2 = (p2Start, 0);

            int rolls = 0;
            while (true)
            {
                int roll = Roll3(rolls);
                rolls += 3;

                p1.Item1 = ((p1.Item1 + roll - 1) % BOARDSIZE) + 1;
                p1.Item2 += p1.Item1;

                if (p1.Item2 >= WINSCORE1) return rolls * p2.Item2;

                roll = Roll3(rolls);
                rolls += 3;
                p2.Item1 = ((p2.Item1 + roll - 1) % BOARDSIZE) + 1;
                p2.Item2 += p2.Item1;

                if (p2.Item2 >= WINSCORE1) return rolls * p1.Item2;
            }
        }

        private int Roll3(int turn) => (turn + 2) * 3;

        private static readonly int[] rollMultiples = new int[] { 0, 0, 0, 1, 3, 6, 7, 6, 3, 1 };

        public override object Task2()
        {
            long[][] p1Record = RecordWinLossCounts(p1Start);
            long[][] p2Record = RecordWinLossCounts(p2Start);

            long p1Wins = 0;
            long p2Wins = 0;
            for (int i = 1; i < p1Record.Length; i++)
            {
                p1Wins += p1Record[i][0] * p2Record[i - 1][1];
                p2Wins += p2Record[i][0] * p1Record[i][1];
            }

            return p1Wins > p2Wins ? p1Wins : p2Wins;
        }

        private class State
        {
            public int Roll = 0;
            public int Score = 0;
            public int Location = 0;
            public int Splits = 1;

            public void Rewind()
            {
                Score -= Location;
                Location -= Roll;
                Splits /= rollMultiples[Roll];
            }

            public void AddOne()
            {
                Score -= Location;
                Location = (Location % BOARDSIZE) + 1;
                Score += Location;
                Splits = Splits / rollMultiples[Roll] * rollMultiples[Roll + 1];
                Roll++;
            }

            public State Next(int roll)
            {
                State n = new State();
                n.Roll = roll;
                n.Location = ((Location + n.Roll - 1) % BOARDSIZE) + 1;
                n.Score += Score + n.Location;
                n.Splits = Splits * rollMultiples[roll];
                return n;
            }
        }

        private long[][] RecordWinLossCounts(int startLocation)
        {
            long[][] winloss = new long[WINSCORE2 / 2 + 1][];
            for (int i = 0; i < winloss.Length; i++) winloss[i] = new long[2];

            Stack<State> s = new Stack<State>();
            s.Push(new State() { Location = startLocation }.Next(MINROLL));

            while (s.Count > 0)
            {
                State current = s.Peek();
                if (current.Score >= WINSCORE2)
                {
                    winloss[s.Count][0] += current.Splits;

                    do current = s.Pop();
                    while (current.Roll == MAXROLL && s.Count > 0);

                    if (current.Roll == MAXROLL && s.Count == 0) break;
                    current.AddOne();
                    s.Push(current);
                    continue;
                }
                winloss[s.Count][1] += current.Splits;
                s.Push(current.Next(MINROLL));
            }
            return winloss;
        }
    }
}
