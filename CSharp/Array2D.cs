using System;
using ExtensionMethods;

namespace Grids
{
    public class Array2D<T>
    {
        private T[,] backingArray;
        public readonly int Height;
        public readonly int Width;

        public Array2D(int height, int width)
        {
            backingArray = new T[height, width];
            Height = height;
            Width = width;
        }

        public Array2D(T[,] source)
        {
            Height = source.GetLength(0);
            Width = source.GetLength(1);
            backingArray = source.Duplicate();
        }

        public T this[int y, int x]
        {
            get => backingArray[y, x];
            set
            {
                backingArray[y, x] = value;
            }
        }

        public T this[Vector2Int v]
        {
            get => backingArray[v.y, v.x];
            set
            {
                backingArray[v.y, v.x] = value;
            }
        }




    }
}