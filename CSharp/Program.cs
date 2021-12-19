using System;
using System.IO;
using System.Net;
using ExtensionMethods;

namespace AdventOfCode
{
    using Console = System.Console;
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length < 2)
            {
                Console.WriteLine("Usage:");
                Console.WriteLine("\tdotnet run [year] [day] [--getInput]");
            }

            string year = args[0];
            string day = int.Parse(args[1]).ToString("00");
            string inputFile = $"../Input/{year}/{day}.txt";

            if (!File.Exists(inputFile))
            {
                GetInput(year, day);
            }

            Console.WriteLine($":: Running challenge {args[0]}.{args[1]} ::");
            string clsName = $"Advent{args[0]}.Challenge{day}";

            Challenge c = (Challenge)Activator.CreateInstance(Type.GetType(clsName));
            string input = System.IO.File.ReadAllText(inputFile);

            int i = 1;
            foreach (ChallengeResult cr in c.Go(input))
            {
                Console.WriteLine($"Task{i} -");
                Console.WriteLine($"    Time: {cr.Time}ms");
                Console.WriteLine($"    Result: {cr.Answer}");
                i++;
            }
        }

        static private void GetInput(string year, string day)
        {
            string src = $"https://adventofcode.com/{year}/day/{int.Parse(day)}/input";
            string dst = $"../Input/{year}/{day}.txt";
            Console.WriteLine($"Getting input for {year}.{day} from {src}");

            string sessionCookie = File.ReadAllText("../session-cookie.txt");

            using (var client = new WebClientC())
            {
                Uri u = new Uri(src);
                client.Cookies.Add(new Cookie("session", sessionCookie, null, u.Host));
                string txt = client.DownloadString(u);
                File.WriteAllText(dst, txt.TrimEnd());
            }
        }
    }

#pragma warning disable SYSLIB0014
    public class WebClientC : WebClient
    {
        CookieContainer cookies = new CookieContainer();
        public CookieContainer Cookies { get { return cookies; } }

        protected override WebRequest GetWebRequest(Uri address)
        {
            WebRequest request = base.GetWebRequest(address);
            if (request.GetType() == typeof(HttpWebRequest))
                ((HttpWebRequest)request).CookieContainer = cookies;

            return request;
        }
    }

    public class Utils
    {
        public static char[,] InputToCharArray(string[] input, bool addBorder = false, char defaultChar = (char)0)
        {
            char[,] grid = new char[input.Length + (addBorder ? 2 : 0), input[0].Length + (addBorder ? 2 : 0)];
            if (defaultChar != 0)
            {
                grid.Fill(defaultChar);
            }
            int y = (addBorder ? 1 : 0);
            int yEnd = input.Length + (addBorder ? 1 : 0);
            int x = (addBorder ? 1 : 0);
            int xEnd = input[0].Length + (addBorder ? 1 : 0);

            for (int gy = 0; y < yEnd; y++, gy++)
            {
                int gx = 0;
                for (gx = 0, x = (addBorder ? 1 : 0); x < xEnd; x++, gx++)
                {
                    grid[y, x] = input[gy][gx];
                }
            }
            return grid;
        }

        public static int[,] InputToIntArray(string[] input, bool addBorder = false, int defaultVal = 0)
        {
            int[,] grid = new int[input.Length + (addBorder ? 2 : 0), input[0].Length + (addBorder ? 2 : 0)];
            if (defaultVal != 0)
            {
                grid.Fill(defaultVal);
            }
            int y = (addBorder ? 1 : 0);
            int yEnd = input.Length + (addBorder ? 1 : 0);
            int x = (addBorder ? 1 : 0);
            int xEnd = input[0].Length + (addBorder ? 1 : 0);

            for (int gy = 0; y < yEnd; y++, gy++)
            {
                int gx = 0;
                for (gx = 0, x = (addBorder ? 1 : 0); x < xEnd; x++, gx++)
                {
                    grid[y, x] = (int)char.GetNumericValue(input[gy][gx]);
                }
            }
            return grid;
        }
    }
}
