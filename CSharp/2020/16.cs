using AdventOfCode;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Advent2020
{
    public class Challenge16 : Challenge
    {
        private int[][] Tickets;
        private List<int[]> ValidTickets = new List<int[]>();
        private List<TicketRule> Rules = new List<TicketRule>();

        public override object Task1()
        {
            ParseRules();
            ParseTickets();

            int answer = 0;
            foreach (int[] ticket in Tickets)
            {
                int badVal = FindInvalidValue(ticket);
                if (badVal == -1)
                {
                    ValidTickets.Add(ticket);
                }
                else
                {
                    answer += badVal;
                }
            }

            return answer;
        }

        private int FindInvalidValue(int[] ticket)
        {
            foreach (int fieldVal in ticket)
            {
                bool hasValidRule = false;
                foreach (TicketRule rule in Rules)
                {
                    if (rule.InRange(fieldVal))
                    {
                        hasValidRule = true;
                        break;
                    }
                }
                if (!hasValidRule) return fieldVal;
            }
            return -1;
        }

        private void ParseTickets()
        {
            int firstTicket = Array.IndexOf(input, "nearby tickets:") + 1;

            Tickets = new int[input.Length - firstTicket][];

            for (int i = 0; i < Tickets.Length; i++)
            {
                string[] p = input[firstTicket + i].Split(',');
                int[] ticket = new int[p.Length];
                for (int j = 0; j < ticket.Length; j++)
                {
                    ticket[j] = int.Parse(p[j]);
                }
                Tickets[i] = ticket;
            }
        }

        private void ParseRules()
        {

            for (int i = 0; i < input.Length; i++)
            {
                if (input[i] == string.Empty) break;

                TicketRule r = new TicketRule();

                string[] parts = input[i].Split(": ", 2);
                r.Name = parts[0];

                string[] ab = parts[1].Split(" or ");
                r.RangeA.Start = int.Parse(ab[0].Split('-')[0]);
                r.RangeA.End = int.Parse(ab[0].Split('-')[1]);
                r.RangeB.Start = int.Parse(ab[1].Split('-')[0]);
                r.RangeB.End = int.Parse(ab[1].Split('-')[1]);
                Rules.Add(r);
            }
        }

        // This is yet another case where the code works for all given
        // testing data, but doesn't work on the actual challenge data
        // Setting this aside until I feel like coming back to it and
        // effectively brute forcing my way to a solution since the 
        // test data is insufficient as-is
        public override object Task2()
        {
            int[] myTicket = ParseMyTicket();

            foreach (TicketRule rule in Rules)
            {
                for (int fieldNum = 0; fieldNum < ValidTickets[0].Length; fieldNum++)
                {
                    bool isValid = true;
                    foreach (int[] ticket in ValidTickets)
                    {
                        if (!rule.InRange(ticket[fieldNum]))
                        {
                            isValid = false;
                            break;
                        }
                    }
                    if (isValid) rule.PossibleFields.Add(fieldNum);
                }
            }

            bool keepGoing = true;
            while (keepGoing)
            {
                keepGoing = false;
                foreach (TicketRule rule in Rules)
                {
                    if (rule.PossibleFields.Count == 1)
                    {
                        foreach (TicketRule otherRule in Rules)
                        {
                            if (otherRule == rule) continue;
                            otherRule.PossibleFields.Remove(rule.FieldPosition);
                        }
                    }
                    else
                    {
                        keepGoing = true;
                    }
                }
            }

            long answer = 1;
            foreach (TicketRule rule in Rules)
            {
                if (rule.Name.StartsWith("departure"))
                {
                    Console.WriteLine(myTicket[rule.FieldPosition]);
                    answer *= myTicket[rule.FieldPosition];
                }
            }

            return answer;
        }

        private bool RuleValidForFieldNum(int field, TicketRule rule)
        {
            foreach (int[] ticket in ValidTickets)
            {
                if (!rule.InRange(ticket[field])) return false;
            }
            return true;
        }

        private int[] ParseMyTicket()
        {
            string[] p = input[Array.IndexOf(input, "your ticket:") + 1].Split(',');
            int[] t = new int[p.Length];
            for (int i = 0; i < t.Length; i++)
            {
                t[i] = int.Parse(p[i]);
            }
            return t;
        }

        private class TicketRule
        {
            public string Name;
            public IntRange RangeA = new IntRange();
            public IntRange RangeB = new IntRange();
            public int FieldPosition
            {
                get => (PossibleFields.Count == 0) ? -1 : PossibleFields[0];
            }

            public List<int> PossibleFields = new List<int>();

            public bool InRange(int i)
            {
                return RangeA.InRange(i) || RangeB.InRange(i);
            }
        }

        private struct IntRange
        {
            public int Start;
            public int End;
            // inclusive of both start and end
            public bool InRange(int i)
            {
                return i <= End && i >= Start;
            }
        }
    }
}