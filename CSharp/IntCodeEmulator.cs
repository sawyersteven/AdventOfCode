using System;
using System.Collections.Generic;
using System.Linq;

namespace Advent2019
{
    namespace IntCode
    {
        using Result = ValueTuple<ExitCode, long>;

        public enum ExitCode
        {
            EOF = 38,
            OutputDelivery = 4,
            Complete = 99,
            InvalidCommand = 1,
            Null = 0
        }

        public static class Tools
        {
            public static long[] ParseCode(string code)
            {
                string[] parts = code.Split(',');
                long[] intCode = new long[parts.Length];
                for (long i = 0; i < intCode.Length; i++) intCode[i] = long.Parse(parts[i]);
                return intCode;
            }
        }

        /// <summary>
        /// How this works:
        /// Emulator is initialized with an array of ints (intcode) that gets
        ///  copied as `program`. Any changes to the code will be made in this
        ///  property while the original code array remains untouched.
        /// 
        /// To load a new intcode program pass it through Reboot(). This will
        ///  start the new program at the beginning and clear all input and output.
        /// 
        /// Start execution of code with Run()
        /// 
        /// Input can be passed as params to Run(). As input is required it will
        ///  used sequentially from the params. If more input is passed than required
        ///  it will be ignored. If more input is required than passed an exception
        ///  will be thrown.
        /// 
        /// Execution will halt for several reasons:
        ///  1) The code has completely successfully
        ///  2) The code has terminated at the end of the intcode array
        ///  3) An invalid opcode has been reached
        ///  4) To relay output back to the calling function
        /// 
        ///  To resume execution call Run() again. New input values can be passed
        ///    at this time.
        /// 
        /// Upon a halt in execution at ValueTuple<ExitCode, int> will be returned.
        ///   The value attached to the ExitCode follows this pattern:
        ///     OutputDelivery: output from the current execution of code
        ///     Complete: the last OutputDelivery value
        ///     EOF: zero
        ///     InvalidCommand: the opcode that caused the error
        /// </summary>
        public class Emulator
        {
            private long[] _Memory;
            public long[] Memory
            {
                get => _Memory;
                private set
                {
                    if (value == null)
                    {
                        _Memory = null;
                        return;
                    }
                    _Memory = new long[value.Length * 100];
                    Array.Copy(value, _Memory, value.Length);
                }
            }
            private long position = 0;
            private long relativeBase = 0;

            public Emulator(long[] program)
            {
                this.Memory = program;
            }

            public Result Reboot(long[] program)
            {
                position = 0;
                this.Memory = program;
                return (ExitCode.Null, 0);
            }

            private long[] divisors = new long[] { 0, 100, 1000, 10000 };
            private long Addr(long pos)
            {
                long mode = (Memory[position] / divisors[pos]) % 10;
                return mode switch
                {
                    0 => Memory[position + pos],
                    1 => position + pos,
                    2 => relativeBase + Memory[position + pos],
                    _ => throw new ArgumentException()
                };
            }

            private static class OP
            {
                public const long ADD = 1;
                public const long MUL = 2;
                public const long INP = 3;
                public const long OUT = 4;
                public const long TRU = 5;
                public const long FAL = 6;
                public const long LTN = 7;
                public const long EQL = 8;
                public const long SRB = 9;
            }

            private Result r = (ExitCode.Null, 0);
            private static int[] modeMask = new int[] { 0, 100, 1000, 10000 };
            public Result Run(params long[] input)
            {
                Queue<long> inpQueue = new Queue<long>(input);
                long inputInd = 0;

                while (position < Memory.Length)
                {
                    long opCode = Memory[position] % 100;
                    switch (opCode)
                    {
                        case OP.ADD:
                            Memory[Addr(3)] = Memory[Addr(1)] + Memory[Addr(2)];
                            position += 4;
                            break;
                        case OP.MUL:
                            Memory[Addr(3)] = Memory[Addr(1)] * Memory[Addr(2)];
                            position += 4;
                            break;
                        case OP.INP:
                            Memory[Addr(1)] = inpQueue.Dequeue();
                            inputInd++;
                            position += 2;
                            break;
                        case OP.OUT:
                            long ret = Memory[Addr(1)];
                            position += 2;
                            r.Item1 = ExitCode.OutputDelivery;
                            r.Item2 = ret;
                            return r;
                        case OP.TRU:
                            if (Memory[Addr(1)] != 0) position = Memory[Addr(2)];
                            else position += 3;
                            break;
                        case OP.FAL:
                            if (Memory[Addr(1)] == 0) position = Memory[Addr(2)];
                            else position += 3;
                            break;
                        case OP.LTN:
                            Memory[Addr(3)] = (Memory[Addr(1)] < Memory[Addr(2)]) ? 1 : 0;
                            position += 4;
                            break;
                        case OP.EQL:
                            Memory[Addr(3)] = (Memory[Addr(1)] == Memory[Addr(2)]) ? 1 : 0;
                            position += 4;
                            break;
                        case OP.SRB:
                            relativeBase += Memory[Addr(1)];
                            position += 2;
                            break;
                        case 99:
                            r.Item1 = ExitCode.Complete;
                            return r;
                        default:
                            return (ExitCode.InvalidCommand, Memory[position] % 100);
                    }
                }
                return (ExitCode.EOF, 0);
            }
        }
    }
}