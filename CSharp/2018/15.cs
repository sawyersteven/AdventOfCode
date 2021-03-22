using AdventOfCode;
using System;
using System.Collections.Generic;
using Grids;

/*
This was a giant pain to get *JUST* right, so I'm leaving many comments
strewn about so if I go back to this at some later date I won't have to
figure it all out again.

This was... not fun. Looking for solutions other people have posted
online wasn't even helpful as they would often result in an incorrect
answer as well. There must be some kind of weird edge case in my input
that other's inputs don't have to deal with.

Regardless, this works now and I can finally put it behind me.
*/

namespace Advent2018
{
    public class Challenge15 : Challenge
    {
        private const bool Elf = true;
        private const bool Goblin = false;

        private static readonly Vector2Int[] readingOrder = new Vector2Int[] { Vector2Int.Down, Vector2Int.Left, Vector2Int.Right, Vector2Int.Up };

        private class Creature
        {
            public readonly bool Species;
            private Vector2Int _Location;
            public Vector2Int Location
            {
                get => _Location;
                set
                {
                    _Location = value;
                    for (int i = 0; i < readingOrder.Length; i++)
                    {
                        adjacentCells[i] = value + readingOrder[i];
                    }
                }
            }
            private Vector2Int[] adjacentCells = new Vector2Int[4];
            public int HP = 200;
            public int AttackPower = 3;

            public Creature(bool species, Vector2Int location)
            {
                Species = species;
                Location = location;
            }

            private bool CanAttack(HashSet<Creature> enemies)
            {
                foreach (Vector2Int v in adjacentCells)
                {
                    foreach (Creature e in enemies)
                    {
                        if (e.Location == v) return true;
                    }
                }
                return false;
            }

            // If game is over returns false
            public bool TakeTurn()
            {
                HashSet<Creature> enemies = new HashSet<Creature>();
                foreach (Creature c in Creatures)
                {
                    if (c.HP > 0 && c.Species != Species) enemies.Add(c);
                }
                if (enemies.Count == 0) return false;

                if (!CanAttack(enemies))
                {
                    Move(enemies);
                }

                Attack(enemies);
                return true;
            }

            private void Attack(HashSet<Creature> enemies)
            {
                int minHP = int.MaxValue;
                Creature target = null;

                foreach (Vector2Int v in adjacentCells)
                {
                    foreach (Creature c in enemies)
                    {
                        if (c.Location == v && c.HP > 0 && c.HP < minHP)
                        {
                            minHP = c.HP;
                            target = c;
                            break;
                        }
                    }
                }

                if (target != null)
                {
                    target.HP -= AttackPower;
                }
            }

            private void Move(HashSet<Creature> enemies)
            {
                HashSet<Vector2Int> blockedCells = new HashSet<Vector2Int>(wallCells);
                foreach (Creature c in Creatures) if (c.HP > 0) blockedCells.Add(c.Location);

                // Get all cells adjacent to an enemy that aren't a wall or occupied
                HashSet<Vector2Int> targetLocations = new HashSet<Vector2Int>();
                foreach (Creature c in enemies)
                {
                    foreach (Vector2Int adj in c.adjacentCells)
                    {
                        if (!blockedCells.Contains(adj)) targetLocations.Add(adj);
                    }
                }
                if (targetLocations.Count == 0) return;

                // Get location to move toward based on reading order
                List<Vector2Int[]> pathsToTargets = ShortestPaths(targetLocations, blockedCells);
                if (pathsToTargets.Count == 0) return;

                Vector2Int target = pathsToTargets[0][^1];
                for (int i = 1; i < pathsToTargets.Count; i++)
                {
                    if (Vector2Sort(target, pathsToTargets[i][^1]) > 0) target = pathsToTargets[i][^1];
                }

                if (target == Location) return;

                // Get first step toward that location, again based on reading order
                List<Vector2Int[]> paths = ShortestPaths(new HashSet<Vector2Int>() { target }, blockedCells);
                if (paths.Count == 0) return;

                Vector2Int step = paths[0][1];
                for (int i = 1; i < paths.Count; i++)
                {
                    if (Vector2Sort(step, paths[i][1]) > 0) step = paths[i][1];
                }
                Location = step;
            }

            /* BFS that searches until any of targets are found, then stops
            searching when the queue'd paths exceed the first path's length so
            that any path to any target within N steps is returned.
            This branches out in readingOrder, so there is some predictability
            to the order of returned paths. But I wouldn't rely on it.
            */
            private List<Vector2Int[]> ShortestPaths(HashSet<Vector2Int> targets, HashSet<Vector2Int> blockedCells)
            {
                var result = new List<Vector2Int[]>();
                int best = int.MaxValue;
                Queue<Vector2Int[]> queue = new Queue<Vector2Int[]>();
                HashSet<Vector2Int> visited = new HashSet<Vector2Int>(blockedCells);
                visited.Remove(Location);

                queue.Enqueue(new Vector2Int[] { Location });

                while (queue.Count > 0)
                {
                    Vector2Int[] path = queue.Dequeue();
                    if (path.Length > best)
                    {
                        return result;
                    }

                    Vector2Int endNode = path[^1];
                    if (targets.Contains(endNode))
                    {
                        result.Add(path);
                        best = path.Length;
                        continue;
                    }

                    if (visited.Contains(endNode)) continue;
                    visited.Add(endNode);

                    foreach (Vector2Int v in readingOrder)
                    {
                        Vector2Int neighbor = endNode + v;
                        if (visited.Contains(neighbor)) continue;

                        Vector2Int[] np = new Vector2Int[path.Length + 1];
                        Array.Copy(path, np, path.Length);
                        np[np.Length - 1] = neighbor;
                        queue.Enqueue(np);
                    }
                }
                return result;
            }
        }


        private static HashSet<Vector2Int> wallCells;
        private static List<Creature> Creatures;
        private List<(bool, Vector2Int)> inititalLocations;
        public override object Task1()
        {
            wallCells = new HashSet<Vector2Int>();
            inititalLocations = new List<(bool, Vector2Int)>();

            for (int y = 0; y < input.Length; y++)
            {
                for (int x = 0; x < input[0].Length; x++)
                {
                    switch (input[y][x])
                    {
                        case 'E':
                            inititalLocations.Add((Elf, new Vector2Int(x, y)));
                            break;
                        case 'G':
                            inititalLocations.Add((Goblin, new Vector2Int(x, y)));
                            break;
                        case '#':
                            wallCells.Add(new Vector2Int(x, y));
                            break;
                    }
                }
            }

            return RunSimluation();
        }

        private int RunSimluation(bool noElfDeaths = false, int elfAttackPower = 0)
        {
            Creatures = new List<Creature>();
            foreach ((bool species, Vector2Int loc) in inititalLocations)
            {
                if (noElfDeaths && species == Elf)
                {
                    Creatures.Add(new Creature(species, loc) { AttackPower = elfAttackPower });
                    continue;
                }
                Creatures.Add(new Creature(species, loc));
            }

            int turn = 0;
            for (; ; turn++)
            {
                for (int i = 0; i < Creatures.Count; i++)
                {
                    if (Creatures[i].HP <= 0)
                    {
                        if (noElfDeaths && Creatures[i].Species == Elf)
                        {
                            return -1;
                        }
                        Creatures.RemoveAt(i);
                        i--;
                    }
                }

                Creatures.Sort(CreatureSort);

                foreach (Creature c in Creatures)
                {
                    if (c.HP <= 0) continue;
                    bool keepPlaying = c.TakeTurn();
                    if (!keepPlaying)
                    {
                        int total = 0;
                        foreach (Creature c2 in Creatures)
                        {
                            if (c2.HP > 0) total += c2.HP;
                        }
                        return total * turn;
                    }
                }
            }
        }

        public override object Task2()
        {
            int atkPower = 4;
            while (true)
            {
                int res = RunSimluation(true, atkPower);
                if (res != -1) return res;
                atkPower++;
            }
        }

        // sorts in reading-order eg row-by-row left->right
        private int CreatureSort(Creature a, Creature b)
        {
            if (a.Location.y == b.Location.y)
            {
                return a.Location.x - b.Location.x;
            }
            return a.Location.y - b.Location.y;
        }

        private static int Vector2Sort(Vector2Int a, Vector2Int b)
        {
            if (a.y == b.y)
            {
                return a.x - b.x;
            }
            return a.y - b.y;
        }
    }
}
