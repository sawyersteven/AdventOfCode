using AdventOfCode;
using System;
using System.Collections.Generic;
using Grids;

namespace Advent2018
{
    public class Challenge13 : Challenge
    {
        private class Cart
        {
            private static int[] turns = new int[] { -1, 0, 1 };
            private static Vector2Int[] directions = new Vector2Int[] { Vector2Int.Down, Vector2Int.Right, Vector2Int.Up, Vector2Int.Left };

            public Vector2Int Position;
            public int direction;
            private int nextTurn = 0;
            public bool HasCrashed = false;

            public Cart(Vector2Int position, int direction)
            {
                Position = position;
                this.direction = direction;
            }

            public void Move(char[,] grid)
            {
                Position += directions[direction];
                char track = grid[Position.y, Position.x];

                switch (track)
                {
                    case '/':
                        Rotate(direction % 2 == 0 ? 1 : 3);
                        break;
                    case '\\':
                        Rotate(direction % 2 == 0 ? 3 : 1);
                        break;
                    case '+':
                        direction += turns[nextTurn];
                        nextTurn += 1;
                        nextTurn %= turns.Length;
                        break;
                }
                direction = (direction + 4) % 4;
            }

            private void Rotate(int amount)
            {
                direction += amount;
                direction %= 4;
            }
        }

        private char[] cartChars = new char[] { '^', '>', 'v', '<' };
        private char[,] grid;
        private List<Cart> carts;
        private HashSet<Vector2Int> cartPositions;
        private void ParseInput()
        {
            cartPositions = new HashSet<Vector2Int>();
            carts = new List<Cart>();
            grid = new char[input.Length, input[0].Length];
            for (int y = 0; y < input.Length; y++)
            {
                for (int x = 0; x < input[0].Length; x++)
                {
                    grid[y, x] = input[y][x];
                    int cartType = Array.IndexOf(cartChars, grid[y, x]);
                    if (cartType != -1)
                    {
                        grid[y, x] = cartType % 2 == 1 ? '-' : '|';
                        Cart c = new Cart(new Vector2Int(x, y), cartType);
                        c.Position = new Vector2Int(x, y);
                        carts.Add(c);
                        cartPositions.Add(c.Position);
                    }
                }
            }
        }

        public override object Task1()
        {
            ParseInput();
            for (int i = 0; ; i++)
            {
                carts.Sort(CartSort);
                foreach (Cart c in carts)
                {
                    cartPositions.Remove(c.Position);
                    c.Move(grid);
                    if (cartPositions.Contains(c.Position))
                    {
                        Console.WriteLine(i);
                        return c.Position;
                    }
                    cartPositions.Add(c.Position);
                }
            }
        }

        private int CartSort(Cart a, Cart b)
        {
            if (a.Position.y > b.Position.y) return 1;
            if (a.Position.y < b.Position.y) return -1;
            return (a.Position.x > b.Position.x) ? 1 : -1;
        }

        public override object Task2()
        {
            ParseInput();
            while (true)
            {
                carts = carts.FindAll((x) => x.HasCrashed == false);
                if (carts.Count == 1)
                {
                    break;
                }
                carts.Sort(CartSort);

                foreach (Cart c in carts)
                {
                    if (c.HasCrashed) continue;

                    cartPositions.Remove(c.Position);
                    c.Move(grid);
                    if (cartPositions.Contains(c.Position))
                    {
                        c.HasCrashed = true;
                        foreach (Cart other in carts)
                        {
                            if (other.Position == c.Position)
                            {
                                other.HasCrashed = true;
                                cartPositions.Remove(other.Position);
                            }
                        }
                    }
                    if (!c.HasCrashed) cartPositions.Add(c.Position);
                }
            }
            return carts[0].Position;
        }
    }
}