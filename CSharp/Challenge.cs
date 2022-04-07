using System.Collections.Generic;
using System.Diagnostics;

namespace AdventOfCode
{
    public abstract class Challenge
    {
        protected string[] input;
        protected string rawInput;

        public virtual void ParseInput()
        {
            return;
        }

        public IEnumerable<object> Go(string rawInput)
        {
            this.rawInput = rawInput;
            this.input = rawInput.Split(new char[] { '\r', '\n' });

            Stopwatch sw = new Stopwatch();
            sw.Start();
            ParseInput();
            sw.Stop();
            yield return new ChallengeResult("Parse Input", null, sw.Elapsed.Milliseconds);

            sw.Restart();
            object r = Task1();
            sw.Stop();
            yield return new ChallengeResult("Task 1", r, sw.Elapsed.TotalMilliseconds);

            sw.Restart();
            r = Task2();
            sw.Stop();
            yield return new ChallengeResult("Task 2", r, sw.Elapsed.TotalMilliseconds);

        }

        public abstract object Task1();
        public abstract object Task2();
    }

    public class ChallengeResult
    {
        public readonly double Time;
        public readonly string Answer;
        public readonly string Name;

        public ChallengeResult(string name, object answer, double time)
        {
            Name = name;
            Time = time;
            Answer = answer?.ToString();
        }
    }
}