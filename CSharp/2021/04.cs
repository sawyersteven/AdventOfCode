using System;
using System.Collections.Generic;
using System.Text.RegularExpressions;
using AdventOfCode;
using ExtensionMethods;

namespace Advent2021
{
    public class Challenge04 : Challenge
    {
        private const int boardSize = 5;

        private int[] callNums;
        private List<int[,]> boards;
        public override void ParseInput()
        {
            callNums = Array.ConvertAll<string, int>(input[0].Split(','), int.Parse);

            boards = new List<int[,]>();
            int[,] board = new int[boardSize, boardSize];
            for (int i = 1; i < input.Length;)
            {
                if (string.IsNullOrEmpty(input[i]))
                {
                    board = new int[boardSize, boardSize];
                }
                i++;
                for (int r = 0; r < 5; r++, i++)
                {
                    int[] boardRow = Array.ConvertAll<string, int>(Regex.Split(input[i].Trim(), @"\D+"), int.Parse);
                    for (int c = 0; c < boardRow.Length; c++)
                    {
                        board[r, c] = boardRow[c];
                    }
                }
                boards.Add(board);
            }
        }

        public override object Task1()
        {
            List<int[,]> t1Boards = new List<int[,]>(boards);
            foreach (int n in callNums)
            {
                foreach (int[,] board in t1Boards)
                {
                    if (MarkBoard(board, n) && CheckBoardWin(board))
                    {
                        return SumBoard(board) * n;
                    }
                }
            }

            return null;
        }

        private bool MarkBoard(int[,] board, int callNum)
        {
            bool modified = false;
            for (int i = 0; i < board.Length; i++)
            {
                int r = i / boardSize;
                int c = i % boardSize;
                if (board[r, c] == callNum)
                {
                    board[r, c] = -1;
                    modified = true;
                }
            }
            return modified;
        }

        private bool CheckBoardWin(int[,] board)
        {
            for (int i = 0; i < boardSize; i++)
            {
                int rowSum = 0;
                int colSum = 0;
                foreach (int cell in board.GetRow(i))
                {
                    rowSum += cell;
                }
                foreach (int cell in board.GetColumn(i))
                {
                    colSum += cell;
                }
                if (rowSum == -5 || colSum == -5) return true;
            }

            return false;
        }

        private int SumBoard(int[,] board)
        {
            int sum = 0;
            foreach (int i in board)
            {
                if (i != -1) sum += i;
            }
            return sum;
        }


        public override object Task2()
        {
            List<int[,]> t2Boards = new List<int[,]>(boards);
            foreach (int n in callNums)
            {
                for (int boardIndex = 0; boardIndex < t2Boards.Count; boardIndex++)
                {
                    int[,] board = t2Boards[boardIndex];
                    if (MarkBoard(board, n) && CheckBoardWin(board))
                    {
                        if (t2Boards.Count == 1)
                        {
                            return SumBoard(board) * n;
                        }
                        t2Boards.RemoveAt(boardIndex);
                        boardIndex--;
                    }
                }
            }
            return null;
        }
    }
}
