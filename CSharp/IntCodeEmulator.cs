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
            private const int memExpandLen = 200;
            private long position = 0;
            private long relativeBase = 0;

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
                    _Memory = new long[value.Length + memExpandLen];
                    Array.Copy(value, _Memory, value.Length);
                }
            }

            public void ExpandMem(int by)
            {
                long[] m = new long[_Memory.Length + by];
                Array.Copy(_Memory, m, _Memory.Length);
                _Memory = m;
            }

            public Result Boot(long[] program)
            {
                position = 0;
                this.Memory = program;
                return (ExitCode.Null, 0);
            }

            private static readonly long[] divisors = new long[] { 0, 100, 1000, 10000 };
            private long Addr(long pos)
            {
                long mode = (_Memory[position] / divisors[pos]) % 10;
                return mode switch
                {
                    0 => _Memory[position + pos],
                    1 => position + pos,
                    2 => relativeBase + _Memory[position + pos],
                    _ => throw new ArgumentException()
                };
            }

            private void ExpandMemory(long reqAddress)
            {
                long[] m = new long[reqAddress + memExpandLen];
                Array.Copy(_Memory, m, _Memory.Length);
                _Memory = m;
                Console.ForegroundColor = ConsoleColor.Yellow;
                Console.WriteLine("* Expanding Memory");
                Console.ResetColor();
            }

            private void Write(long val, long address)
            {
                if (address > _Memory.Length - 1)
                {
                    ExpandMemory(address);
                }
                _Memory[address] = val;
            }

            private long Read(long address)
            {
                if (address > _Memory.Length - 1)
                {
                    ExpandMemory(address);
                }
                return _Memory[address];
            }

            private Result r = (ExitCode.Null, 0);
            public Result Run(params long[] input)
            {
                Queue<long> inpQueue = new Queue<long>(input);

                while (position < _Memory.Length)
                {
                    long opCode = Memory[position] % 100;
                    switch (opCode)
                    {
                        case OP.ADD:
                            Write(Read(Addr(1)) + Read(Addr(2)), Addr(3));
                            position += 4;
                            break;
                        case OP.MUL:
                            Write(Read(Addr(1)) * Read(Addr(2)), Addr(3));
                            position += 4;
                            break;
                        case OP.INP:
                            Write(inpQueue.Dequeue(), Addr(1));
                            position += 2;
                            break;
                        case OP.OUT:
                            long ret = Read(Addr(1));
                            position += 2;
                            r.Item1 = ExitCode.OutputDelivery;
                            r.Item2 = ret;
                            return r;
                        case OP.TRU:
                            if (Read(Addr(1)) != 0) position = Read(Addr(2));
                            else position += 3;
                            break;
                        case OP.FAL:
                            if (Read(Addr(1)) == 0) position = Read(Addr(2));
                            else position += 3;
                            break;
                        case OP.LTN:
                            Write(Read(Addr(1)) < Read(Addr(2)) ? 1 : 0, Addr(3)); // todo:
                            position += 4;
                            break;
                        case OP.EQL:
                            Write((Read(Addr(1)) == Read(Addr(2))) ? 1 : 0, Addr(3));
                            position += 4;
                            break;
                        case OP.SRB:
                            relativeBase += Read(Addr(1));
                            position += 2;
                            break;
                        case 99:
                            r.Item1 = ExitCode.Complete;
                            return r;
                        default:
                            return (ExitCode.InvalidCommand, Read(position) % 100);
                    }
                }
                return (ExitCode.EOF, 0);
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
        }
    }
}