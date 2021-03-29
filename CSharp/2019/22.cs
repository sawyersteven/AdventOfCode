using System;
using System.Numerics;
using AdventOfCode;

namespace Advent2019
{
    public class Challenge22 : Challenge
    {
        private const long deckSize = 10_007;
        private const long deckSize2 = 119315717514047;
        private const long shuffles = 101741582076661;

        (Technique t, int)[] parsedInput;
        private long[] deck;
        private long[] scratchDeck;

        private enum Technique
        {
            Deal,
            DealN,
            CutN
        }

        public override object Task1()
        {
            deck = new long[deckSize];
            scratchDeck = new long[deckSize];
            for (int i = 0; i < deckSize; i++)
            {
                deck[i] = i;
            }

            ParseInput();

            foreach ((Technique t, int arg) in parsedInput)
            {
                switch (t)
                {
                    case Technique.Deal:
                        Deal(arg);
                        break;
                    case Technique.DealN:
                        DealN(arg);
                        break;
                    case Technique.CutN:
                        CutN(arg);
                        break;
                }
            }

            return Array.IndexOf(deck, 2019);
        }

        private void ParseInput()
        {
            parsedInput = new (Technique, int)[input.Length];
            for (int i = 0; i < input.Length; i++)
            {
                if (input[i][0] == 'c')
                {
                    parsedInput[i] = (Technique.CutN, int.Parse(input[i].Split(' ')[^1]));
                }
                else
                {
                    if (input[i][^1] == 'k')
                    {
                        parsedInput[i] = (Technique.Deal, 0);
                    }
                    else
                    {
                        parsedInput[i] = (Technique.DealN, int.Parse(input[i].Split(' ')[^1]));
                    }
                }
            }
        }

        private void Deal(int _) => Array.Reverse(deck);

        private void CutN(int n)
        {
            if (n > 0)
            {
                Array.Copy(deck, scratchDeck, n);

                for (int i = 0; i < deck.Length - n; i++)
                {
                    deck[i] = deck[i + n];
                }
                for (int i = 1; i <= n; i++)
                {
                    deck[deck.Length - i] = scratchDeck[n - i];
                }
            }
            else
            {
                n = Math.Abs(n);
                Array.Copy(deck, deck.Length - n, scratchDeck, 0, n);
                for (int i = deck.Length - 1; i > n - 1; i--)
                {
                    deck[i] = deck[i - n];
                }
                for (int i = 0; i < n; i++)
                {
                    deck[i] = scratchDeck[i];
                }
            }
        }

        private void DealN(int n)
        {
            Array.Copy(deck, scratchDeck, deck.Length);

            for (int i = 0; i < deck.Length; i++)
            {
                deck[(i * n) % deck.Length] = scratchDeck[i];
            }
        }

        /* This is more of an advanced mathematics problem than a programming
        problem that can be intuited with moderate knoweldge, so I'll happily
        admit to cribbing most of this from a python solution I found that
        required 3 pages to poorly explain the math
        */
        public override object Task2()
        {
            const long iters = 101741582076661;
            BigInteger cardIndex = 2020;

            BigInteger a = 1;
            BigInteger b = 0;

            // convert the procedures into linear algebra
            foreach ((Technique t, int arg) in parsedInput)
            {
                long la = 0;
                long lb = 0;
                switch (t)
                {
                    case Technique.Deal:
                        la = -1;
                        lb = -1;
                        break;
                    case Technique.DealN:
                        la = arg;
                        lb = 0;
                        break;
                    case Technique.CutN:
                        la = 1;
                        lb = -arg;
                        break;
                }
                a = RealMod(la * a, deckSize2);
                b = RealMod(la * b + lb, deckSize2);
            }

            BigInteger Ma = (BigInteger)System.Numerics.BigInteger.ModPow(a, iters, deckSize2);
            BigInteger Mb = RealMod((b * (Ma - 1)) * InvMod(a - 1, deckSize2), deckSize2);

            return RealMod(((cardIndex - Mb) * InvMod(Ma, deckSize2)), deckSize2);
        }

        // in C#, % is a remainder operation, not actually mod
        private BigInteger RealMod(BigInteger x, BigInteger m)
        {
            return (x % m + m) % m;
        }

        // returns a number X such that (value * X) % mod == 1;
        private BigInteger InvMod(BigInteger value, BigInteger mod)
        {
            return (BigInteger)BigInteger.ModPow(value, mod - 2, mod);
        }
    }
}