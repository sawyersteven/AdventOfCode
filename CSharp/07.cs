using System;
using System.Collections.Generic;

namespace Advent2019
{
    public class Challenge07 : Challenge
    {
        private long[] thrusterCode;
        public override object Task1()
        {
            thrusterCode = IntCode.Tools.ParseCode(input[0]);
            IntCode.Emulator ICE = new IntCode.Emulator();
            long maxSignal = 0;
            (IntCode.ExitCode, long) response = (IntCode.ExitCode.Null, 0);
            foreach (int[] arr in Permutate(new int[] { 0, 1, 2, 3, 4 }))
            {
                response.Item2 = 0;
                foreach (int s in arr)
                {
                    ICE.Boot(thrusterCode);
                    response = ICE.Run(s, response.Item2);

                }
                if (response.Item2 > maxSignal) maxSignal = response.Item2;
            }
            return maxSignal;
        }

        private static IEnumerable<T[]> Permutate<T>(T[] sequence)
        {
            foreach (T[] i in _Permutate(sequence, sequence.Length))
            {
                yield return i;
            }

            IEnumerable<T[]> _Permutate(T[] sequence, int count)
            {
                if (count == 1) yield return sequence;

                for (int i = 0; i < count; i++)
                {
                    foreach (T[] perm in _Permutate(sequence, count - 1))
                    {
                        yield return perm;
                    }
                    T tmp = sequence[count - 1];
                    for (int j = count - 1; j > 0; j--)
                    {
                        sequence[j] = sequence[j - 1];
                    }
                    sequence[0] = tmp;
                }
            }
        }

        IntCode.Emulator[] thrusters;
        public override object Task2()
        {
            thrusters = new IntCode.Emulator[5];
            for (int i = 0; i < 5; i++)
            {
                thrusters[i] = new IntCode.Emulator();
            }

            int[] phaseSettings = new int[] { 5, 6, 7, 8, 9 };
            long maxSignal = 0;
            foreach (int[] arr in Permutate(phaseSettings))
            {
                long sig = RunFeedbackLoop(arr);
                if (sig > maxSignal)
                {
                    maxSignal = sig;
                }
            }
            return maxSignal;
        }

        private long RunFeedbackLoop(int[] settings)
        {
            (IntCode.ExitCode, long) response = (IntCode.ExitCode.Null, 0);
            for (int i = 0; i < thrusters.Length; i++)
            {
                thrusters[i].Boot(thrusterCode);
                response = thrusters[i].Run(settings[i], response.Item2);
            }

            int currentThruster = 0;
            while (true)
            {
                IntCode.Emulator thruster = thrusters[currentThruster];

                response = thruster.Run(response.Item2);

                if (response.Item1 == IntCode.ExitCode.Complete && currentThruster == thrusters.Length - 1) return response.Item2;

                currentThruster++;
                currentThruster %= thrusters.Length;
            }
        }
    }
}