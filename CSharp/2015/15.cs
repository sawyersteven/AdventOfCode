using System;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge15 : Challenge
    {
        int[][] ingredients;
        public override void ParseInput()
        {
            char[] splt = new char[] { ' ', ',' };
            ingredients = new int[input.Length][];
            for (int i = 0; i < input.Length; i++)
            {
                string[] parts = input[i].Split(splt, StringSplitOptions.RemoveEmptyEntries);
                ingredients[i] = new int[] { int.Parse(parts[2]), int.Parse(parts[4]), int.Parse(parts[6]), int.Parse(parts[8]), int.Parse(parts[10]) };
            }
        }

        // I wanted to do this with a generator, but it seemed like a giant pain
        public override object Task1()
        {
            const int total = 100;

            int ingredientCount = ingredients[0].Length - 1;

            int best = 0;
            int[] tsps = new int[] { 1, 1, 1, 1 };
            for (tsps[0] = 1; tsps[0] < total; tsps[0]++)
            {
                for (tsps[1] = 1; tsps[1] < total - tsps[0]; tsps[1]++)
                {
                    for (tsps[2] = 1; tsps[2] < total - tsps[1] - tsps[0]; tsps[2]++)
                    {
                        tsps[3] = total - tsps[1] - tsps[2] - tsps[0];

                        int score = ScoreRecipe(tsps);

                        if (score > best) best = score;
                    }
                }
            }
            return best;
        }

        private int ScoreRecipe(int[] tsps)
        {
            int score = 1;

            for (int attr = 0; attr < ingredients[0].Length - 1; attr++)
            {
                int attrScore = 0;
                for (int ingredient = 0; ingredient < ingredients.Length; ingredient++)
                {
                    int s = tsps[ingredient] * ingredients[ingredient][attr];
                    attrScore += s;
                }
                if (attrScore <= 0) return 0;
                score *= attrScore;
            }
            return score;
        }

        private int ScoreRecipe500Cal(int[] tsps)
        {
            int c = Calories(tsps);
            if (c != 500) return 0;
            int score = 1;

            for (int attr = 0; attr < ingredients[0].Length - 1; attr++)
            {
                int attrScore = 0;
                for (int ingredient = 0; ingredient < ingredients.Length; ingredient++)
                {
                    int s = tsps[ingredient] * ingredients[ingredient][attr];
                    attrScore += s;
                }
                if (attrScore <= 0) return 0;
                score *= attrScore;
            }
            return score;
        }

        private int Calories(int[] tsps)
        {
            int cal = 0;
            for (int tsp = 0; tsp < tsps.Length; tsp++)
            {
                cal += ingredients[tsp][^1] * tsps[tsp];
            }
            return cal;
        }

        public override object Task2()
        {
            const int total = 100;

            int ingredientCount = ingredients[0].Length - 1;

            int best = 0;
            int[] tsps = new int[] { 1, 1, 1, 1 };
            for (tsps[0] = 1; tsps[0] < total; tsps[0]++)
            {
                for (tsps[1] = 1; tsps[1] < total - tsps[0]; tsps[1]++)
                {
                    for (tsps[2] = 1; tsps[2] < total - tsps[1] - tsps[0]; tsps[2]++)
                    {
                        tsps[3] = total - tsps[1] - tsps[2] - tsps[0];

                        int score = ScoreRecipe500Cal(tsps);

                        if (score > best) best = score;
                    }
                }
            }
            return best;
        }
    }
}
