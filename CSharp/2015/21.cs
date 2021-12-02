using AdventOfCode;

namespace Advent2015
{
    public class Challenge21 : Challenge
    {
        (int, int, int)[] Weapons = new (int, int, int)[]{
            (8,4,0),
            (10,5,0),
            (25,6,0),
            (40,7,0),
            (74,8,0)};

        (int, int, int)[] Armor = new (int, int, int)[]{
            (0,0,0),
            (13,0,1),
            (31,0,2),
            (53,0,3),
            (75,0,4),
            (102,0,5)};

        (int, int, int)[] Rings = new (int, int, int)[]{
            (0,0,0),
            (0,0,0),
            (25,1,0),
            (50,2,0),
            (100,3,0),
            (20,0,1),
            (40,0,2),
            (80,0,3)};


        private int bossStartHP;
        private int bossArmor;
        private int bossDMG;

        public override void ParseInput()
        {
            bossStartHP = int.Parse(input[0].Split(' ')[^1]);
            bossDMG = int.Parse(input[1].Split(' ')[^1]);
            bossArmor = int.Parse(input[2].Split(' ')[^1]);
        }

        public override object Task1()
        {
            int lowestCost = int.MaxValue;

            foreach ((int wcost, int damage, int _) in Weapons)
            {
                foreach ((int acost, int _, int armor) in Armor)
                {
                    for (int ring1 = 0; ring1 < Rings.Length - 1; ring1++)
                    {
                        (int r1cost, int r1dmg, int r1armor) = Rings[ring1];
                        for (int ring2 = ring1 + 1; ring2 < Rings.Length; ring2++)
                        {
                            (int r2cost, int r2dmg, int r2armor) = Rings[ring2];

                            int totalDMG = damage + r1dmg + r2dmg;
                            int totalArmor = armor + r1armor + r2armor;

                            int totalCost = wcost + acost + r1cost + r2cost;
                            if (WinBattle(totalDMG, totalArmor))
                            {
                                if (totalCost < lowestCost) lowestCost = totalCost;
                            }
                            else
                            {
                                if (totalCost > highestCost) highestCost = totalCost;
                            }
                        }
                    }
                }
            }

            return lowestCost;
        }

        private bool WinBattle(int dmg, int armor)
        {
            int bossHP = bossStartHP;
            int playerHP = 100;

            if (dmg == bossArmor) dmg++;
            if (armor == bossDMG) armor--;

            while (true)
            {
                bossHP -= dmg - bossArmor;
                if (bossHP <= 0) return true;
                playerHP -= bossDMG - armor;
                if (playerHP <= 0) return false;
            }
        }

        private int highestCost = 0;
        public override object Task2()
        {
            return highestCost;
        }
    }
}
