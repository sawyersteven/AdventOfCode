using AdventOfCode;

namespace Advent2019
{
    public class Challenge23 : Challenge
    {
        const int networkSize = 50;
        IntCode.Emulator[] network;

        private void MakeNetwork()
        {
            long[] program = IntCode.Tools.ParseCode(rawInput);
            network = new IntCode.Emulator[networkSize];
            for (int i = 0; i < networkSize; i++)
            {
                network[i] = new IntCode.Emulator(program);
                var resp = network[i].Run();
                network[i].QueueInput(i);
            }
        }

        public override object Task1()
        {
            MakeNetwork();

            while (true)
            {
                foreach (IntCode.Emulator ICE in network)
                {
                    var response = ICE.Run();
                    if (response.Item1 == IntCode.StatusCode.InputRequest)
                    {
                        ICE.QueueInput(-1);
                    }
                    else if (response.Item1 == IntCode.StatusCode.OutputDelivery)
                    {
                        long destination = response.Item2;
                        long x = ICE.Run().Item2;
                        long y = ICE.Run().Item2;
                        if (destination == 255) return y;
                        network[destination].QueueInput(x, y);
                    }
                    else
                    {
                        throw new System.Exception();
                    }
                }
            }
        }

        public override object Task2()
        {
            MakeNetwork();

            (long, long) NATPacket = (0, 0);
            long lastNATPacketY = -1;

            while (true)
            {
                bool networkIdle = true;
                foreach (IntCode.Emulator ICE in network)
                {
                    var response = ICE.Run();
                    if (response.Item1 == IntCode.StatusCode.InputRequest)
                    {
                        ICE.QueueInput(-1);
                    }
                    else if (response.Item1 == IntCode.StatusCode.OutputDelivery)
                    {
                        networkIdle = false;
                        long destination = response.Item2;
                        if (destination == 255)
                        {
                            NATPacket = (ICE.Run().Item2, ICE.Run().Item2);
                        }
                        else
                        {
                            network[destination].QueueInput(ICE.Run().Item2, ICE.Run().Item2);
                        }
                    }
                    else
                    {
                        throw new System.Exception();
                    }
                }
                if (networkIdle)
                {
                    network[0].QueueInput(NATPacket.Item1, NATPacket.Item2);
                    if (lastNATPacketY == NATPacket.Item2)
                    {
                        return lastNATPacketY;
                    }
                    lastNATPacketY = NATPacket.Item2;
                }
            }
        }
    }
}