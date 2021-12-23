using System;

namespace Grids
{
    public struct Rectangle
    {
        public int xMin;
        public int xMax;
        public int yMin;
        public int yMax;

        public int Width { get; private set; } = 0;
        public int Height { get; private set; } = 0;

        public Rectangle(int xMin, int yMin, int xMax, int yMax)
        {
            this.xMin = xMin;
            this.xMax = xMax;
            this.yMin = yMin;
            this.yMax = yMax;

            Recalc();
        }

        public override string ToString()
        {
            return $"[<{xMin}, {yMin}>, <{xMax},{yMax}>]";
        }

        public void Recalc()
        {
            Width = xMax - xMin;
            Height = yMax - yMin;
        }

        public bool Contains(Vector2Int point)
        {
            return point.x >= xMin && point.x <= xMax && point.y >= yMin && point.y <= yMax;
        }
    }
}