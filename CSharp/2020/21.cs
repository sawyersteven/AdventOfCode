using AdventOfCode;
using System.Collections.Generic;

namespace Advent2020
{
    public class Challenge21 : Challenge
    {

        private void IntersectQueue<T>(Queue<T> q, IEnumerable<T> e)
        {
            int c = q.Count;
            for (int i = 0; i < c; i++)
            {
                bool keep = false;
                T qVal = q.Dequeue();
                foreach (T eVal in e)
                {
                    if (eVal.Equals(qVal))
                    {
                        keep = true;
                        break;
                    }
                }
                if (keep) q.Enqueue(qVal);
            }
        }

        private void RemoveFromQueue<T>(Queue<T> q, T val)
        {
            int c = q.Count;
            for (int i = 0; i < c; i++)
            {
                T qVal = q.Dequeue();
                if (!qVal.Equals(val)) q.Enqueue(qVal);
            }
        }

        private void EnqueueAll<T>(Queue<T> q, IEnumerable<T> additions)
        {
            foreach (T a in additions) q.Enqueue(a);
        }

        private Dictionary<string, Queue<string>> allergenMap;

        public override object Task1()
        {
            allergenMap = new Dictionary<string, Queue<string>>();

            Queue<string> safeIngredients = new Queue<string>();
            HashSet<string> unsafeIngredients = new HashSet<string>();

            Queue<string> ingredientPile = new Queue<string>();

            // make allergen map
            foreach (string line in input)
            {
                string[] parts = line.Split(" (contains ");
                string[] recipe = parts[0].Split(' ');
                string[] allergens = parts[1].Substring(0, parts[1].Length - 1).Split(", ");

                EnqueueAll(ingredientPile, recipe);

                foreach (string a in allergens)
                {
                    if (!allergenMap.ContainsKey(a))
                    {
                        allergenMap[a] = new Queue<string>(recipe);
                    }
                    else
                    {
                        IntersectQueue(allergenMap[a], recipe);
                    }
                }

                foreach (string r in recipe)
                {
                    if (!safeIngredients.Contains(r)) safeIngredients.Enqueue(r);
                }
            }

            // reduce allergen map
            HashSet<string> reduced = new HashSet<string>();
            while (reduced.Count != allergenMap.Count)
            {
                foreach (KeyValuePair<string, Queue<string>> kv in allergenMap)
                {
                    if (!reduced.Contains(kv.Key) && kv.Value.Count == 1)
                    {
                        reduced.Add(kv.Key);
                        string val = kv.Value.Peek();
                        unsafeIngredients.Add(val);
                        foreach (KeyValuePair<string, Queue<string>> kv2 in allergenMap)
                        {
                            if (kv.Key == kv2.Key) continue;
                            RemoveFromQueue(kv2.Value, val);
                        }
                    }
                }
            }

            // remove unsafe ingredients
            int c = safeIngredients.Count;
            for (int i = 0; i < c; i++)
            {
                string ing = safeIngredients.Dequeue();
                if (!unsafeIngredients.Contains(ing)) safeIngredients.Enqueue(ing);
            }

            ulong answer = 0;
            foreach (string ingredient in ingredientPile)
            {
                if (safeIngredients.Contains(ingredient)) answer++;
            }

            return answer;
        }

        public override object Task2()
        {
            List<string> allergenNames = new List<string>(allergenMap.Keys);
            allergenNames.Sort();

            string[] orderedIngredients = new string[allergenNames.Count];
            for (int i = 0; i < orderedIngredients.Length; i++)
            {
                orderedIngredients[i] = allergenMap[allergenNames[i]].Dequeue();
            }

            return string.Join(',', orderedIngredients);
        }
    }
}