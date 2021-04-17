using AdventOfCode;
using Grids;

namespace Advent2016
{
    public class Challenge02 : Challenge
    {
        private char[,] keypad2 = new char[,] { {' ',' ',' ',' ',' ',' ',' '},
                                                {' ',' ',' ','1',' ',' ',' '},
                                                {' ',' ','2','3','4',' ',' '},
                                                {' ','5','6','7','8','9',' '},
                                                {' ',' ','A','B','C',' ',' '},
                                                {' ',' ',' ','D',' ',' ',' '},
                                                {' ',' ',' ',' ',' ',' ',' '}};

        public override object Task1()
        {
            int[] code = new int[input.Length];
            int key = 5;
            for (int i = 0; i < input.Length; i++)
            {

                foreach (char c in input[i])
                {
                    switch (c)
                    {
                        case 'L':
                            if (key % 3 == 1) continue;
                            key--;
                            break;
                        case 'R':
                            if (key % 3 == 0) continue;
                            key++;
                            break;
                        case 'U':
                            if (key <= 3) continue;
                            key -= 3;
                            break;
                        case 'D':
                            if (key >= 7) continue;
                            key += 3;
                            break;
                    }
                }
                code[i] = key;
            }

            return string.Join("", code);
        }

        public override object Task2()
        {
            char[] code = new char[input.Length];
            Vector2Int location = new Vector2Int(1, 3);
            for (int i = 0; i < input.Length; i++)
            {

                foreach (char c in input[i])
                {
                    switch (c)
                    {
                        case 'L':
                            if (keypad2[location.y, location.x - 1] != ' ') location.x--;
                            break;
                        case 'R':
                            if (keypad2[location.y, location.x + 1] != ' ') location.x++;
                            break;
                        case 'U':
                            if (keypad2[location.y - 1, location.x] != ' ') location.y--;
                            break;
                        case 'D':
                            if (keypad2[location.y + 1, location.x] != ' ') location.y++;
                            break;
                    }
                }
                code[i] = keypad2[location.y, location.x];
            }


            return string.Join("", code);
        }
    }
}
