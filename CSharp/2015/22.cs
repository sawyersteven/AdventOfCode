using System.Collections.Generic;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge22 : Challenge
    {
        private const int BossStartHP = 55;
        private const int BossDMG = 8;

        private enum SpellName
        {
            Missile,
            Drain,
            Shield,
            Poison,
            Recharge
        }

        private struct Spell
        {
            public SpellName ID;
            public int Cost;
            public int Damage;
            public int Heal;
            public int Armor;
            public int Recharge;
            public int Duration;

            public Spell(SpellName id, int cost, int damage, int heal, int armor, int recharge, int duration)
            {
                ID = id;
                Cost = cost;
                Damage = damage;
                Heal = heal;
                Armor = armor;
                Recharge = recharge;
                Duration = duration;
            }
        }

        private Spell[] Spells = new Spell[]
        {
            new Spell(SpellName.Missile,53, 4, 0, 0, 0, 0),
            new Spell(SpellName.Drain,73, 2, 2, 0, 0, 0),
            new Spell(SpellName.Shield,113, 0, 0, 7, 0, 6),
            new Spell(SpellName.Poison,173, 3, 0, 0, 0, 6),
            new Spell(SpellName.Recharge,229, 0, 0, 0, 101, 5)
        };

        public override object Task1()
        {
            return Sim();
        }

        public override object Task2()
        {
            return Sim(true);
        }

        public int Sim(bool hardMode = false)
        {
            int leastMana = int.MaxValue;
            State start = new State(50, 500, 0, BossStartHP, 0, 0, 0);
            Queue<State> q = new Queue<State>();
            q.Enqueue(start);

            while (q.Count > 0)
            {
                State state = q.Dequeue();
                if (hardMode)
                {
                    state.PlayerHP--;
                    if (state.PlayerHP < 1) continue;
                }
                foreach (Spell nextSpell in Spells)
                {
                    if (state.PlayerMana < nextSpell.Cost) continue;

                    State next = state.Copy();

                    if (nextSpell.Duration == 0)
                    {
                        next.BossHP -= nextSpell.Damage;
                        next.PlayerHP += nextSpell.Heal;
                    }
                    else
                    {
                        if (next[nextSpell.ID] > 0) continue;
                        next[nextSpell.ID] = nextSpell.Duration;
                    }

                    next.PlayerMana -= nextSpell.Cost;
                    next.TotalConsumedMana += nextSpell.Cost;
                    next.ApplyEffects();

                    // boss turn
                    if (next.BossHP > 0)
                    {
                        int dmg = BossDMG - (next.ShieldTimer > 0 ? 7 : 0);
                        if (dmg < 1) dmg = 1;
                        next.PlayerHP -= dmg;
                        next.ApplyEffects();
                    }

                    if (next.TotalConsumedMana >= leastMana || next.PlayerHP < 1) continue;

                    if (next.BossHP < 1)
                    {
                        leastMana = next.TotalConsumedMana;
                        continue;
                    }

                    q.Enqueue(next);
                }
            }
            return leastMana;
        }

        private class State
        {
            public int PlayerHP;
            public int PlayerMana;
            public int TotalConsumedMana;
            public int BossHP;
            public int ShieldTimer;
            public int PoisonTimer;
            public int RechargeTimer;

            public State(int playerHP, int playerMana, int totalConsumedMana, int bossHP, int shieldTimer, int poisonTimer, int rechargeTimer)
            {
                PlayerHP = playerHP;
                PlayerMana = playerMana;
                TotalConsumedMana = totalConsumedMana;
                BossHP = bossHP;
                ShieldTimer = shieldTimer;
                PoisonTimer = poisonTimer;
                RechargeTimer = rechargeTimer;
            }

            public ref int this[SpellName sn]
            {
                get
                {
                    if (sn == SpellName.Poison)
                    {
                        return ref PoisonTimer;
                    }
                    else if (sn == SpellName.Shield)
                    {
                        return ref ShieldTimer;
                    }
                    else if (sn == SpellName.Recharge)
                    {
                        return ref RechargeTimer;
                    }
                    throw new System.Exception();
                }
            }

            public State Copy()
            {
                return new State(PlayerHP, PlayerMana, TotalConsumedMana, BossHP, ShieldTimer, PoisonTimer, RechargeTimer);
            }

            public void ApplyEffects()
            {
                if (ShieldTimer > 0) ShieldTimer--;
                if (PoisonTimer > 0)
                {
                    BossHP -= 3;
                    PoisonTimer--;
                }
                if (RechargeTimer > 0)
                {
                    PlayerMana += 101;
                    RechargeTimer--;
                }
            }
        }
    }
}