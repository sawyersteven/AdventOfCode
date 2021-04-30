using System.Diagnostics;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge12 : Challenge
    {
        public override object Task1()
        {
            int t = 0;
            string[] parts = rawInput.Split(new char[] { ':', ',', '}', '[', ']' });
            foreach (string s in parts)
            {
                if (int.TryParse(s, out int i)) t += i;
            }
            return t;
        }

        // CSharp is terrible at deserializing unknown JSON even with Newtonsoft, so I cheated and called a python script to get the answer
        public override object Task2()
        {
            ProcessStartInfo start = new ProcessStartInfo();
            start.FileName = "py";
            start.Arguments = "2015/2015.12.py";
            start.UseShellExecute = false;
            start.RedirectStandardInput = true;
            start.RedirectStandardOutput = true;
            Process p = Process.Start(start);
            p.StandardInput.Write(rawInput);
            p.StandardInput.Close();

            return p.StandardOutput.ReadToEnd();
        }
    }
}
