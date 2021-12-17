using System.Collections;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2021
{
    public class Challenge16 : Challenge
    {
        private enum packetID
        {
            sum = 0,
            prod = 1,
            min = 2,
            max = 3,
            literal = 4,
            gt = 5,
            lt = 6,
            eq = 7,
        }

        BitArrayReader br;
        public override void ParseInput()
        {
            long[] longs = new long[rawInput.Length];
            int i = 0;
            for (i = 0; i < rawInput.Length; i++)
            {
                longs[i] = long.Parse(rawInput[i].ToString(), System.Globalization.NumberStyles.HexNumber);
            }

            BitArray bits = new BitArray(longs.Length * 4);
            i = 0;
            foreach (long j in longs)
            {
                long mask = 0b1000;
                for (long k = 0; k < 4; k++, i++)
                {
                    bits[i] = (j & mask) == mask;
                    mask >>= 1;
                }
            }
            br = new BitArrayReader(bits);
        }

        private long outVal = -1;
        public override object Task1()
        {
            outVal = br.ReadPacket();
            return br.VersionSum;
        }

        public override object Task2()
        {
            return outVal;
        }

        private class BitArrayReader
        {
            private BitArray arr;
            public readonly long Length;
            public int Position = 0;
            public long VersionSum = 0;

            public BitArrayReader(BitArray array)
            {
                arr = array;
                Length = array.Length;
            }

            public bool Peek() => arr[Position];

            public bool ReadBool() => arr[Position++];

            public long ReadPacket()
            {
                VersionSum += ReadLong(3);
                long id = ReadLong(3);

                if (id == 4) return ReadLiteral();

                List<long> subPacketVals = ReadSubpacketValues();

                long val = 0;
                switch (id)
                {
                    case 0: // sum
                        foreach (long i in subPacketVals) val += i;
                        break;
                    case 1: // prod
                        val = 1;
                        foreach (long i in subPacketVals) val *= i;
                        break;
                    case 2: // min
                        val = long.MaxValue;
                        foreach (long i in subPacketVals) if (i < val) val = i;
                        break;
                    case 3: // max
                        foreach (long i in subPacketVals) if (i > val) val = i;
                        break;
                    case 4: // gt
                        val = subPacketVals[0] > subPacketVals[1] ? 1 : 0;
                        break;
                    case 5: // lt
                        val = subPacketVals[0] < subPacketVals[1] ? 1 : 0;
                        break;
                    case 6: // eq
                        val = subPacketVals[0] == subPacketVals[1] ? 1 : 0;
                        break;
                }

                return val;
            }

            public long ReadLong(long length)
            {
                long o = 0;
                long end = Position + length;
                for (; Position < end; Position++)
                {
                    o <<= 1;
                    o += arr[Position] ? 1 : 0;
                }
                return o;
            }

            public List<long> ReadSubpacketValues()
            {
                List<long> vals = new List<long>();  // long[] ?
                bool lenType = ReadBool();

                if (lenType)
                {
                    long subPacketCount = ReadLong(11);
                    for (long i = 0; i < subPacketCount; i++)
                    {
                        vals.Add(ReadPacket());
                    }
                }
                else
                {
                    long subPacketLen = ReadLong(15);
                    long packetsEnd = Position + subPacketLen;
                    while (Position < packetsEnd)
                    {
                        vals.Add(ReadPacket());
                    }
                }
                return vals;
            }

            public long ReadLiteral()
            {
                long o = 0;
                long flag;
                do
                {
                    flag = ReadLong(1);
                    o <<= 4;
                    o += ReadLong(4);

                } while (flag != 0);
                return o;
            }
        }
    }
}
