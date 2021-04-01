using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2019
{
    public class Challenge25 : Challenge
    {
        // There is probably a programmatic way to populate this. But I don't want to.
        private string[] forbiddenItems = new string[] { "molten lava", "photons", "infinite loop", "giant electromagnet", "escape pod" };

        public override object Task1()
        {
            IntCode.Emulator ICE = new IntCode.Emulator(rawInput);

            string dir = ExploreMap(ICE);

            return TrySecDoor(ICE, dir);
        }

        private string TrySecDoor(IntCode.Emulator ICE, string doorDirection)
        {
            SendString(ICE, "inv");
            string output = RunUntilDone(ICE).Item2;
            HashSet<string> inventory = new HashSet<string>();
            foreach (string line in output.Split((char)10))
            {
                if (line.StartsWith('-')) inventory.Add(line.Substring(2));
            }

            foreach (HashSet<string> i in PermutateInventory(inventory))
            {
                HashSet<string> remove = new HashSet<string>(inventory);
                remove.ExceptWith(i);
                foreach (string r in remove)
                {
                    DropItem(ICE, r);
                }

                HashSet<string> add = new HashSet<string>(i);
                add.ExceptWith(inventory);
                foreach (string a in add)
                {
                    PickUpItem(ICE, a);
                }
                inventory = new HashSet<string>(i);

                SendString(ICE, doorDirection);
                var resp = RunUntilDone(ICE);
                if (resp.Item1 != IntCode.StatusCode.InputRequest)
                {
                    return resp.Item2.Split("typing ")[1].Split(' ')[0];
                }
            }
            return string.Empty;
        }

        private void DropItem(IntCode.Emulator ICE, string item)
        {
            SendString(ICE, "drop " + item);
            RunUntilDone(ICE);
        }

        private void PickUpItem(IntCode.Emulator ICE, string item)
        {
            SendString(ICE, "take " + item);
            RunUntilDone(ICE);
        }

        private IEnumerable<HashSet<string>> PermutateInventory(HashSet<string> inventory)
        {
            int mask = (int)Math.Pow(2, inventory.Count) - 1;
            List<string> inv = new List<string>(inventory);
            HashSet<string> permutation = new HashSet<string>();
            while (mask > 0)
            {
                int m = 1;
                for (int i = 0; i < inventory.Count; i++)
                {

                    if ((mask & m) == m)
                    {
                        permutation.Add(inv[i]);
                    }
                    m <<= 1;
                }
                yield return permutation;
                permutation.Clear();
                mask--;
            }
        }

        private (IntCode.StatusCode, string) RunUntilDone(IntCode.Emulator ICE)
        {
            List<char> outputBuffer = new List<char>();
            while (true)
            {
                var r = ICE.Run();
                if (r.Item1 != IntCode.StatusCode.OutputDelivery)
                {
                    return (r.Item1, string.Join("", outputBuffer));
                }
                outputBuffer.Add((char)r.Item2);
            }
        }

        // Makes a loop around all the rooms by always trying to take the left-
        // hand door when possible. If no door is to the drone's left it turns
        // right until thre is.
        // Returns once the security checkpoint is reached for the 2nd time, 
        // ensuring every other room has been visisted at least one time.
        // Returns a string for the direction of the pressure-plate room.
        private string ExploreMap(IntCode.Emulator ICE)
        {
            Queue<char> outBuffer = new Queue<char>();

            int currentDir = 0;
            bool secCheck = false;

            while (true)
            {
                var resp = ICE.Run();
                if (resp.Item1 == IntCode.StatusCode.InputRequest)
                {
                    string report = string.Join("", outBuffer);
                    outBuffer.Clear();
                    string[] doors = report.Split("lead:")[1].Split("\n\n")[0].Substring(3).Split("\n- ");

                    if (report.Contains("here:"))
                    {
                        string[] items = report.Split("here:")[1].Split("\n\n")[0].Substring(3).Split("\n- ");
                        foreach (string item in items)
                        {
                            if (Array.IndexOf(forbiddenItems, item) != -1) continue;
                            SendString(ICE, "take " + item);
                        }
                    }

                    if (report.Contains("Security Checkpoint"))
                    {
                        if (secCheck == false)
                        {
                            secCheck = true;
                            currentDir = (currentDir + 2) % 4;
                            SendString(ICE, directions[currentDir]);
                            continue;
                        }
                        else
                        {
                            return GetNextDirection(currentDir, doors).Item2;
                        }
                    }

                    string nextRoom;
                    (currentDir, nextRoom) = GetNextDirection(currentDir, doors);

                    SendString(ICE, nextRoom);
                    ICE.Run();
                }

                outBuffer.Enqueue((char)resp.Item2);
            }
        }

        string[] directions = new string[] { "north", "east", "south", "west" };
        private (int, string) GetNextDirection(int currentDir, string[] doors)
        {
            string next = string.Empty;
            while (next == string.Empty)
            {
                string leftHandDoor = directions[(currentDir + 3) % 4];
                if (Array.IndexOf(doors, leftHandDoor) == -1)
                {
                    currentDir = (currentDir + 1) % 4;
                    continue;
                }
                currentDir = (currentDir + 3) % 4;
                next = leftHandDoor;
            }
            return (currentDir, next);
        }
        private long SendString(IntCode.Emulator ICE, string instructions)
        {
            long[] inst = new long[instructions.Length + 1];
            for (int i = 0; i < instructions.Length; i++)
            {
                inst[i] = instructions[i];
            }
            inst[inst.Length - 1] = (char)10;
            ICE.QueueInput(inst);
            return ICE.Run().Item2;
        }

        public override object Task2()
        {

            return null;
        }
    }
}