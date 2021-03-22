using AdventOfCode;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge24 : Challenge
    {
        private const bool Immune = true;
        private const bool Infection = false;

        private class Group
        {
            public bool System;
            public int Units;
            public int HP;
            public HashSet<string> Weaknesses = new HashSet<string>();
            public HashSet<string> Immunities = new HashSet<string>();
            public int AttackDamage;
            public string AttackType;
            public int Initiative;
            public int EffectivePower => Units * AttackDamage;

            public Group(bool system, string info)
            {
                System = system;
                string[] words = info.Split(' ');
                Units = int.Parse(words[0]);
                HP = int.Parse(words[4]);

                if (info.Contains('('))
                {
                    foreach (string p in info.Split('(')[1].Split(')')[0].Split("; "))
                    {
                        HashSet<string> hs = p[0] == 'w' ? Weaknesses : Immunities;
                        foreach (string s in p.Split("to ")[1].Split(", "))
                        {
                            hs.Add(s);
                        };
                    }
                }

                AttackDamage = int.Parse(words[^6]);
                AttackType = words[^5];
                Initiative = int.Parse(words[^1]);
            }

            public int CalcDamage(Group target)
            {
                if (target.Immunities.Contains(AttackType)) return 0;
                else if (target.Weaknesses.Contains(AttackType)) return EffectivePower * 2;
                return EffectivePower;
            }
        }

        private List<Group> ParseInput()
        {
            List<Group> groups = new List<Group>();
            bool system = Immune;
            for (int line = 1; line < input.Length; line++)
            {
                if (input[line] == string.Empty)
                {
                    line++;
                    system = Infection;
                    continue;
                }
                groups.Add(new Group(system, input[line]));
            }
            return groups;
        }

        private void RemoveDead(List<Group> armies)
        {
            for (int i = 0; i < armies.Count; i++)
            {
                if (armies[i].Units <= 0)
                {
                    armies.RemoveAt(i);
                    i--;
                }
            }
        }

        private int TargetSelectionSort(Group a, Group b)
        {
            if (a.EffectivePower == b.EffectivePower)
            {
                return (a.Initiative > b.Initiative) ? -1 : 1;
            }
            return (a.EffectivePower > b.EffectivePower) ? -1 : 1;
        }

        private int AttackOrderSort(Group a, Group b)
        {
            return (a.Initiative > b.Initiative) ? -1 : 1;
        }

        private Dictionary<Group, Group> SelectTargets(List<Group> armies)
        {
            armies.Sort(TargetSelectionSort);

            Dictionary<Group, Group> attackMap = new Dictionary<Group, Group>();

            foreach (Group attacker in armies)
            {
                int bestDmg = 0;
                Group bestTarget = null;
                foreach (Group target in armies)
                {
                    if (attacker.System == target.System || attackMap.ContainsValue(target)) continue;
                    int damage = attacker.CalcDamage(target);
                    if (damage == 0) continue;
                    if (damage > bestDmg)
                    {
                        bestDmg = damage;
                        bestTarget = target;
                    }
                    else if (damage == bestDmg)
                    {
                        if (bestTarget.EffectivePower == target.EffectivePower)
                        {
                            bestTarget = bestTarget.Initiative > target.Initiative ? bestTarget : target;
                        }
                        else
                        {
                            bestTarget = bestTarget.EffectivePower > target.EffectivePower ? bestTarget : target;
                        }
                    }
                }
                attackMap[attacker] = bestTarget;
            }
            return attackMap;
        }

        private bool BattleOver(List<Group> groups)
        {
            bool first = groups[0].System;
            for (int i = 1; i < groups.Count; i++)
            {
                if (groups[i].System != first) return false;
            }
            return true;
        }

        private List<Group> SimulateBattle(int immuneBoost = 0)
        {
            List<Group> groups = ParseInput();

            foreach (Group g in groups)
            {
                if (g.System == Immune)
                {
                    g.AttackDamage += immuneBoost;
                }
            }

            bool stalemate = false;
            while (!BattleOver(groups))
            {
                stalemate = true;
                Dictionary<Group, Group> attackMap = SelectTargets(groups);
                groups.Sort(AttackOrderSort);
                foreach (Group attacker in groups)
                {
                    if (attacker.Units <= 0) continue;

                    Group target = attackMap[attacker];
                    if (target == null) continue;
                    int lostUnits = attacker.CalcDamage(target) / target.HP;
                    if (lostUnits > 0) stalemate = false;
                    target.Units -= lostUnits;
                }
                RemoveDead(groups);
                if (stalemate) return null;
            };
            return groups;
        }

        private int TotalUnits(List<Group> victors)
        {
            int total = 0;
            foreach (Group g in victors)
            {
                if (g.Units > 0) total += g.Units;
            }
            return total;
        }

        public override object Task1()
        {
            List<Group> victors = SimulateBattle();

            return TotalUnits(victors);
        }

        public override object Task2()
        {
            // chunk it up in case its a high number. it isn't for my input.
            int boost = 1;
            for (; ; boost += 10)
            {
                List<Group> victors = SimulateBattle(boost);
                if (victors != null && victors[0].System == Immune)
                {
                    boost -= 10;
                    break;
                }
            }

            for (; ; boost++)
            {
                List<Group> victors = SimulateBattle(boost);
                if (victors != null && victors[0].System == Immune)
                {
                    return TotalUnits(victors);
                }
            }
        }
    }
}