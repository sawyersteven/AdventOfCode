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
            InputRequest = 74,
            EOF = 38,
            OutputDelivery = 4,
            Complete = 99,
            InvalidCommand = 1,
            Null = 0
        }

        public static class Tools
        {
            public static long[] ParseCode(string code) => Array.ConvertAll(code.Split(','), long.Parse);
        }

        /// <summary>
        /// How this works:
        /// Emulator is initialized with an array of ints (intcode) that gets
        ///  copied as `program`. Any changes to the code will be made in this
        ///  property while the original code array remains untouched.
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
            public static Result ResultTemplate = (ExitCode.Null, 0);

            public bool Verbose { get; set; } = true;

            private const int memExpandLen = 200;
            private long position = 0;
            private long relativeBase = 0;

            private long[] OrigProgram;

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

            private Queue<long> inpQueue = new Queue<long>();

            public void QueueInput(params long[] input)
            {
                foreach (long i in input) inpQueue.Enqueue(i);
            }

            public void EmptyInputQueue()
            {
                inpQueue.Clear();
            }

            public void ExpandMem(int by)
            {
                long[] m = new long[_Memory.Length + by];
                Array.Copy(_Memory, m, _Memory.Length);
                _Memory = m;
            }

            public Emulator(long[] program)
            {
                Memory = program;
                OrigProgram = Memory.Clone() as long[];
            }

            public Emulator(string program)
            {
                Memory = Tools.ParseCode(program);
                OrigProgram = Memory.Clone() as long[];
            }

            public Result Reboot()
            {
                position = 0;
                inpQueue.Clear();
                relativeBase = 0;
                Memory = OrigProgram.Clone() as long[];
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
                if (Verbose)
                {
                    Console.ForegroundColor = ConsoleColor.Yellow;
                    Console.WriteLine("* Expanding Memory");
                    Console.ResetColor();
                }
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
            public Result Run()
            {
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
                            if (inpQueue.Count == 0)
                            {
                                return (ExitCode.InputRequest, 0);
                            }
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