using AdventOfCode;

namespace Advent2019
{
    public class Challenge07 : Challenge
    {
        private long[] thrusterCode;
        public override object Task1()
        {
            thrusterCode = IntCode.Tools.ParseCode(input[0]);
            IntCode.Emulator ICE = new IntCode.Emulator(thrusterCode);
            long maxSignal = 0;

            var response = IntCode.Emulator.ResultTemplate;
            foreach (int[] arr in Permutator.Permutate(new int[] { 0, 1, 2, 3, 4 }))
            {
                response.Item2 = 0;
                foreach (int s in arr)
                {
                    ICE.Reboot();
                    ICE.QueueInput(s, response.Item2);
                    response = ICE.Run();
                }
                if (response.Item2 > maxSignal) maxSignal = response.Item2;
            }
            return maxSignal;
        }

        IntCode.Emulator[] thrusters;
        public override object Task2()
        {
            thrusters = new IntCode.Emulator[5];
            for (int i = 0; i < 5; i++)
            {
                thrusters[i] = new IntCode.Emulator(thrusterCode);
            }

            int[] phaseSettings = new int[] { 5, 6, 7, 8, 9 };
            long maxSignal = 0;
            foreach (int[] arr in Permutator.Permutate(phaseSettings))
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
            (IntCode.StatusCode, long) response = (IntCode.StatusCode.Null, 0);
            for (int i = 0; i < thrusters.Length; i++)
            {
                thrusters[i].Reboot();
                thrusters[i].ExpandMem(20_000_000); // yup
                thrusters[i].QueueInput(settings[i], response.Item2);
                response = thrusters[i].Run();
            }

            int currentThruster = 0;
            while (true)
            {
                IntCode.Emulator thruster = thrusters[currentThruster];
                thruster.QueueInput(response.Item2);
                response = thruster.Run();

                if (response.Item1 == IntCode.StatusCode.Complete && currentThruster == thrusters.Length - 1) return response.Item2;

                currentThruster++;
                currentThruster %= thrusters.Length;
            }
        }
    }
}