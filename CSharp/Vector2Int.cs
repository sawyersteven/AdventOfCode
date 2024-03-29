using System;

namespace Grids
{
    // Mostly based on Unity's Vector2Int
    public struct Vector2Int
    {
        public int x;
        public int y;

        public Vector2Int(int x, int y)
        {
            this.x = x;
            this.y = y;
        }

        public static Vector2Int operator -(Vector2Int v)
        {
            return new Vector2Int(-v.x, -v.y);
        }

        public static Vector2Int operator +(Vector2Int a, Vector2Int b)
        {
            return new Vector2Int(a.x + b.x, a.y + b.y);
        }

        public static Vector2Int operator -(Vector2Int a, Vector2Int b)
        {
            return new Vector2Int(a.x - b.x, a.y - b.y);
        }

        public static Vector2Int operator *(Vector2Int a, Vector2Int b)
        {
            return new Vector2Int(a.x * b.x, a.y * b.y);
        }

        public static Vector2Int operator *(Vector2Int a, int b)
        {
            return new Vector2Int(a.x * b, a.y * b);
        }

        public static Vector2Int operator /(Vector2Int a, int b)
        {
            return new Vector2Int(a.x / b, a.y / b);
        }

        public static bool operator ==(Vector2Int lhs, Vector2Int rhs)
        {
            return lhs.x == rhs.x && lhs.y == rhs.y;
        }

        public static bool operator !=(Vector2Int lhs, Vector2Int rhs)
        {
            return !(lhs == rhs);
        }

        public override string ToString()
        {
            return $"<{x},{y}>";
        }

        public bool Equals(Vector2Int other)
        {
            return x == other.x && y == other.y;
        }

        public override int GetHashCode()
        {
            uint rol5 = ((uint)x << 5) | ((uint)x >> 27);
            return ((int)rol5 + x) ^ y;
        }

        public override bool Equals(object other)
        {
            if (!(other is Vector2Int)) return false;

            return Equals((Vector2Int)other);
        }

        public double AngleTo(Vector2Int other)
        {
            double ang = -((180 / Math.PI) * Math.Atan2(((x * other.x) + (y * other.y)), ((x * other.y) + (y * other.x)))) + 90;
            return ang < 0 ? ang + 360 : ang;
        }

        public int ManhattanDistance(Vector2Int other)
        {
            return Math.Abs(x - other.x) + Math.Abs(y - other.y);
        }

        public static readonly Vector2Int Zero = new Vector2Int(0, 0);
        public static readonly Vector2Int Up = new Vector2Int(0, 1);
        public static readonly Vector2Int Down = new Vector2Int(0, -1);
        public static readonly Vector2Int Left = new Vector2Int(-1, 0);
        public static readonly Vector2Int Right = new Vector2Int(1, 0);
        public static readonly Vector2Int UR = new Vector2Int(1, 1);
        public static readonly Vector2Int DR = new Vector2Int(1, -1);
        public static readonly Vector2Int DL = new Vector2Int(-1, -1);
        public static readonly Vector2Int UL = new Vector2Int(-1, 1);

        public static readonly Vector2Int[] CardinalDirections = new Vector2Int[] { Up, Right, Down, Left };
        public static readonly Vector2Int[] AllDirections = new Vector2Int[] { Up, UR, Right, DR, Down, DL, Left, UL };

    }
}