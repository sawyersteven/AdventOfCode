using System;
using System.Collections.Generic;

namespace Advent2019
{
    public class Challenge16 : Challenge
    {
        int[] pattern = new int[] { 0, 1, 0, -1 };

        public override object Task1()
        {
            int[] signal = new int[input[0].Length];
            for (int i = 0; i < signal.Length; i++)
            {
                signal[i] = (int)char.GetNumericValue(input[0][i]);
            }

            for (int _ = 0; _ < 100; _++)
            {
                int[] result = DecodeMessage(signal);
                Array.Copy(result, signal, result.Length);
            }
            return string.Join("", new ArraySegment<int>(signal, 0, 8));
        }

        private int[] DecodeMessage(int[] signal)
        {
            int[] result = new int[signal.Length];

            for (int row = 0; row < signal.Length; row++)
            {
                int output = 0;

                int patternOffset = 0;
                for (int col = 0; col < signal.Length; col++)
                {
                    if ((col % (row + 1)) - row == 0)
                    {
                        patternOffset = (patternOffset + 1) % pattern.Length;
                    }
                    output += signal[col] * pattern[patternOffset];

                }
                result[row] = Math.Abs(output % 10);
            }
            return result;
        }

        public override object Task2()
        {
            /* Ok so this is a doozy. I don't usually write out the thought process
            for these since the code makes it obvious, but this was a real chore to
            completely understand and I want to document it.
            
            After the halfway point in the input signal, the matching pattern is
            N zeros and M ones where N is the row index and M is signal.Length - N.

            eg with a 10-digit signal length

            0    1 0-1 0 1 0-1 0 1 0
            1    0 1 1 0 0-1-1 0 0 1
            2    0 0 1 1 1 0 0 0-1-1
            3    0 0 0 1 1 1 1 0 0 0
            4    0 0 0 0 1 1 1 1 1 0
            * Halfway *
            5    0 0 0 0 0 1 1 1 1 1 (5 zeros)
            6    0 0 0 0 0 0 1 1 1 1 (6 zeros)
            7    0 0 0 0 0 0 0 1 1 1 (etc...)
            8    0 0 0 0 0 0 0 0 1 1
            9    0 0 0 0 0 0 0 0 0 1

            This means several things:

            The first half of the signal has no effect on the second half of the 
            result. The second half numbers can be processed without even looking
            at the first half, assuming the offset for finding the answer is
            greater than signal.Length / 2;

            Since the only numbers contributing to the answer are multiple of 1,
            there is no need to multiply anything. We can simply add the signal
            numbers starting from the row index and take the ones digit.

            To make this easier we can pretend the first half of the signal
            doesn't even exist. Additionally, we can ignore all of the signal *before*
            the offset position. The patterns remains the same, starting with all
            ones, then [0, 1...] and so on.

            Working backward, we keep a running total of the sigal's values.
            Where i == signal.Length - 1, we can add signal[i] to the total, then
            replace signal[i] with the ones digit of the total. Continue backward
            until the signal is consumed to complete the phase.
            */

            const int multiplier = 10_000;
            int offset = int.Parse(input[0].Substring(0, 7));

            int[] baseSignal = new int[input[0].Length];
            for (int i = 0; i < baseSignal.Length; i++)
            {
                baseSignal[i] = (int)char.GetNumericValue(input[0][i]);
            }

            int totalLen = input[0].Length * multiplier;
            int[] signal = new int[totalLen - offset];
            for ((int i, int j) = (signal.Length - 1, baseSignal.Length - 1); i > -1; i--, j--)
            {
                if (j == -1) j = baseSignal.Length - 1;
                signal[i] = baseSignal[j];
            }

            int signalTotal = 0;
            for (int _ = 0; _ < 100; _++)
            {
                for (int i = signal.Length - 1; i > -1; i--)
                {
                    signalTotal += signal[i];
                    signal[i] = signalTotal % 10;
                }
                signalTotal = 0;
            }

            return string.Join("", new ArraySegment<int>(signal, 0, 8));
        }
    }
}