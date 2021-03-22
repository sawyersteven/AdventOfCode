using AdventOfCode;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Advent2020
{
    public class Challenge04 : Challenge
    {

        private IEnumerator<Dictionary<string, string>> ParsePassports()
        {
            Dictionary<string, string> fields = new Dictionary<string, string>();
            foreach (string line in input)
            {
                if (line == string.Empty)
                {
                    yield return fields;
                    fields.Clear();
                    continue;
                }

                string[] parts = line.Split(' ');
                foreach (string part in parts)
                {
                    string[] kv = part.Split(':');
                    fields.Add(kv[0], kv[1]);
                }
            }
            yield return fields;
        }

        public override object Task1()
        {
            int goodPassports = 0;
            IEnumerator<Dictionary<string, string>> passports = ParsePassports();

            passports.MoveNext();
            HashSet<string> reqFields = new HashSet<string>(passports.Current.Keys);
            reqFields.Remove("cid");
            goodPassports++;

            while (passports.MoveNext())
            {
                passports.Current.Remove("cid");
                if (reqFields.SetEquals(passports.Current.Keys))
                {
                    goodPassports++;
                }
            }
            return goodPassports;
        }

        public override object Task2()
        {
            // Because I want to without regex
            string[] eyeColors = new string[] { "amb", "blu", "brn", "gry", "grn", "hzl", "oth" };

            int goodPassports = 0;
            IEnumerator<Dictionary<string, string>> passports = ParsePassports();
            passports.MoveNext();
            HashSet<string> reqFields = new HashSet<string>(passports.Current.Keys);
            reqFields.Remove("cid");

            passports.Dispose();
            passports = ParsePassports();

            while (passports.MoveNext())
            {
                passports.Current.Remove("cid");
                if (!reqFields.SetEquals(passports.Current.Keys)) continue;

                if (passports.Current["byr"].Length + passports.Current["iyr"].Length + passports.Current["eyr"].Length != 12) continue;

                int byr = int.Parse(passports.Current["byr"]);
                if (byr < 1920 || byr > 2002) continue;
                int iyr = int.Parse(passports.Current["iyr"]);
                if (iyr < 2010 || iyr > 2020) continue;
                int eyr = int.Parse(passports.Current["eyr"]);
                if (eyr < 2020 || eyr > 2030) continue;

                if (passports.Current["hgt"].Length < 4) continue;
                int hgt = int.Parse(passports.Current["hgt"].Substring(0, (passports.Current["hgt"].Length - 2)));
                if (passports.Current["hgt"].Substring(passports.Current["hgt"].Length - 2) == "in")
                {
                    if (hgt < 59 || hgt > 76) continue;
                }
                else if (hgt < 150 || hgt > 193) continue;

                if (passports.Current["hcl"][0] != '#') continue;
                if (!int.TryParse(passports.Current["hcl"].Substring(1), System.Globalization.NumberStyles.HexNumber, null, out _)) continue;

                if (!eyeColors.Contains(passports.Current["ecl"])) continue;

                if (passports.Current["pid"].Length != 9 || !int.TryParse(passports.Current["pid"], out _)) continue;
                goodPassports++;
            }
            return goodPassports;
        }
    }
}