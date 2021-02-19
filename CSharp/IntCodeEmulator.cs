using System;

namespace Advent2019
{
    namespace IntCode
    {
        using Result = ValueTuple<ExitCode, int>;

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
            public static int[] ParseCode(string code)
            {
                string[] parts = code.Split(',');
                int[] intCode = new int[parts.Length];
                for (int i = 0; i < intCode.Length; i++) intCode[i] = int.Parse(parts[i]);
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
            private int[] _program;
            public int[] program
            {
                get => _program;
                private set
                {
                    if (value == null)
                    {
                        _program = null;
                        return;
                    }
                    _program = new int[value.Length];
                    Array.Copy(value, _program, value.Length);
                }
            }
            private int position = 0;

            public Emulator(int[] program)
            {
                this.program = program;
            }

            public void Reboot(int[] program)
            {
                position = 0;
                this.program = program;
            }

            int Param1() => (program[position] / 100 % 10) == 0 ? program[program[position + 1]] : program[position + 1];
            int Param2() => (program[position] / 1000 % 10) == 0 ? program[program[position + 2]] : program[position + 2];
            void Write(int val) => program[program[position + 3]] = val;


            private Result r = (ExitCode.Null, 0);
            public Result Run(params int[] c3Input)
            {
                int inputInd = 0;

                while (position < program.Length)
                {
                    int opCode = program[position] % 100;
                    switch (opCode)
                    {
                        case 1:
                            Write(Param1() + Param2());
                            position += 4;
                            break;
                        case 2:
                            Write(Param1() * Param2());
                            position += 4;
                            break;
                        case 3:
                            program[program[position + 1]] = c3Input[inputInd];
                            inputInd++;
                            position += 2;
                            break;
                        case 4:
                            int ret = Param1();
                            position += 2;
                            r.Item1 = ExitCode.OutputDelivery;
                            r.Item2 = ret;
                            return r;
                        case 5:
                            if (Param1() != 0) position = Param2();
                            else position += 3;
                            break;
                        case 6:
                            if (Param1() == 0) position = Param2();
                            else position += 3;
                            break;
                        case 7:
                            Write((Param1() < Param2()) ? 1 : 0);
                            position += 4;
                            break;
                        case 8:
                            Write((Param1() == Param2()) ? 1 : 0);
                            position += 4;
                            break;
                        case 99:
                            r.Item1 = ExitCode.Complete;
                            return r;
                        default:
                            return (ExitCode.InvalidCommand, program[position] % 100);
                    }
                }
                return (ExitCode.EOF, 0);
            }
        }
    }
}