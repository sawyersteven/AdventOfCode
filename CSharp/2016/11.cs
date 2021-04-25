using AdventOfCode;

namespace Advent2016
{
    public class Challenge11 : Challenge
    {
        /* 
        This probably doesn't work with all of the inputs.
        */
        public override object Task1()
        {
            int[] itemsPerFloor = new int[5];
            itemsPerFloor[1] = 4;
            itemsPerFloor[2] = 5;
            itemsPerFloor[3] = 1;

            return Solve(itemsPerFloor);
        }

        private int Solve(int[] itemsPerFloor)
        {
            int itemCount = 0;
            for (int i = 0; i < itemsPerFloor.Length; i++)
            {
                itemCount += itemsPerFloor[i];
            }

            int moves = 0;
            int floor = 1;
            // assuming we can clear one whole floor at a time without running into a roadblock
            while (itemsPerFloor[4] != itemCount)
            {
                moves += 2 * itemsPerFloor[floor] - 3;
                itemsPerFloor[floor + 1] += itemsPerFloor[floor];
                floor += 1;
            }

            return moves;
        }

        public override object Task2()
        {
            int[] itemsPerFloor = new int[5];
            itemsPerFloor[1] = 8;
            itemsPerFloor[2] = 5;
            itemsPerFloor[3] = 1;

            return Solve(itemsPerFloor);
        }
    }
}

/*

F4 
F3                            TM
F2             TG RG RM CG CM
F1 SG SM PG PM                  [EG EM DG DM]
                                   Part 2  
*/

