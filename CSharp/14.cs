using System.Collections.Generic;

namespace Advent2019
{
    public class Challenge14 : Challenge
    {
        private class Ingredient
        {
            public string Name;
            public long Amount;

            public Ingredient(string raw)
            {
                string[] parts = raw.Split(' ');
                Name = parts[1];
                Amount = int.Parse(parts[0]);
            }
            public Ingredient(string name, long amount)
            {
                Name = name;
                Amount = amount;
            }

            public override string ToString()
            {
                return $"{Amount} {Name}";
            }
        }

        // name, (amount made, requirements)
        Dictionary<string, (long, List<Ingredient>)> recipes = new Dictionary<string, (long, List<Ingredient>)>();
        public override object Task1()
        {
            ParseInput();
            return MakeFuel();
        }

        private long MakeFuel(long amt = 1)
        {
            DefaultDict<string, long> leftovers = new DefaultDict<string, long>();

            long requiredOre = 0;
            Queue<Ingredient> orders = new Queue<Ingredient>();

            orders.Enqueue(new Ingredient("FUEL", amt));

            while (orders.Count > 0)
            {
                Ingredient order = orders.Dequeue();
                if (order.Name == "ORE")
                {
                    requiredOre += order.Amount;
                }
                else if (order.Amount <= leftovers[order.Name])
                {
                    leftovers[order.Name] -= order.Amount;
                }
                else
                {
                    long reqAmt = order.Amount - leftovers[order.Name];
                    var recipe = recipes[order.Name];
                    long iters = DivideRoundUp(reqAmt, recipe.Item1);
                    foreach (Ingredient ing in recipe.Item2)
                    {
                        orders.Enqueue(new Ingredient(ing.Name, ing.Amount * iters));
                    }
                    leftovers[order.Name] = iters * recipe.Item1 - reqAmt;
                }
            }
            return requiredOre;
        }

        private void ParseInput()
        {
            foreach (string line in input)
            {
                string[] io = line.Split(" => ");
                string[] outParts = io[1].Split(" ");
                List<Ingredient> l = new List<Ingredient>();
                foreach (string ing in io[0].Split(", "))
                {
                    l.Add(new Ingredient(ing));
                }
                recipes[outParts[1]] = (int.Parse(outParts[0]), l);
            }
        }

        private long DivideRoundUp(long a, long b) => (a - 1) / b + 1;

        public override object Task2()
        {
            long oreReserves = 1000000000000;

            int lower = 1;
            int upper = -1;
            int amount = 0;
            while (lower + 1 != upper)
            {
                if (upper == -1)
                {
                    amount = lower * 2;
                }
                else
                {
                    amount = (upper + lower) / 2;
                }

                long reqOre = MakeFuel(amount);

                if (reqOre > oreReserves)
                {
                    upper = amount;
                }
                else
                {
                    lower = amount;
                }
            }
            return lower;
        }
    }
}