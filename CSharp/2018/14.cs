using AdventOfCode;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge14 : Challenge
    {
        public override object Task1()
        {
            int turns = int.Parse(input[0]);

            RecipeMaker rp = new RecipeMaker();
            while (rp.recipeScores.Count < turns + 10)
            {
                rp.SimulateRound();
            }
            return string.Join("", rp.recipeScores.GetRange(turns, 10));
        }

        private class RecipeMaker
        {
            int elfA = 0;
            int elfB = 1;
            public readonly List<int> recipeScores = new List<int>() { 3, 7 };
            public void SimulateRound()
            {
                int newRecipe = recipeScores[elfA] + recipeScores[elfB];
                if (newRecipe >= 10)
                {
                    recipeScores.Add(1);
                    recipeScores.Add(newRecipe - 10);
                }
                else
                {
                    recipeScores.Add(newRecipe);
                }
                elfA = (elfA + recipeScores[elfA] + 1) % recipeScores.Count;
                elfB = (elfB + recipeScores[elfB] + 1) % recipeScores.Count;
            }
        }

        public override object Task2()
        {
            List<int> scoreSequence = new List<int>();
            foreach (char c in input[0])
            {
                scoreSequence.Add((int)char.GetNumericValue(c));
            }
            int scoreSeqLen = input[0].Length;

            RecipeMaker rp = new RecipeMaker();

            while (rp.recipeScores.Count - 1 - scoreSeqLen < 0)
            {
                rp.SimulateRound();
            }

            int lastLen = rp.recipeScores.Count;
            while (true)
            {
                rp.SimulateRound();

                // this mess is much quicker than comparing a List<T>.GetRange()
                bool match = true;
                for (int i = 1; i <= scoreSeqLen; i++)
                {
                    match &= (rp.recipeScores[^i] == scoreSequence[^i]);
                    if (!match) break;
                }
                if (match)
                {
                    return rp.recipeScores.Count - scoreSeqLen;
                }

                if (rp.recipeScores.Count - lastLen == 2)
                {
                    match = true;
                    for (int i = 1; i <= scoreSeqLen; i++)
                    {
                        match &= (rp.recipeScores[^(i + 1)] == scoreSequence[^i]);
                        if (!match) break;
                    }
                    if (match)
                    {
                        return rp.recipeScores.Count - scoreSeqLen - 1;
                    }
                }
                lastLen = rp.recipeScores.Count;
            }
        }
    }
}