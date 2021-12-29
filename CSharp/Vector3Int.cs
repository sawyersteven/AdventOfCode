using System;

namespace Grids
{
    // Mostly based on Unity's Vector3Int
    public struct Vector3Int
    {
        public int x;
        public int y;
        public int z;

        public Vector3Int(int x, int y, int z)
        {
            this.x = x;
            this.y = y;
            this.z = z;
        }

        public static implicit operator Vector2Int(Vector3Int v) => new Vector2Int(v.x, v.y);

        public static Vector3Int operator -(Vector3Int v)
        {
            return new Vector3Int(-v.x, -v.y, -v.z);
        }

        public static Vector3Int operator +(Vector3Int a, Vector3Int b)
        {
            return new Vector3Int(a.x + b.x, a.y + b.y, a.z + b.z);
        }

        public static Vector3Int operator +(Vector3Int a, Vector2Int b)
        {
            return new Vector3Int(a.x + b.x, a.y + b.y, a.z);
        }

        public static Vector3Int operator -(Vector3Int a, Vector3Int b)
        {
            return new Vector3Int(a.x - b.x, a.y - b.y, a.z - b.z);
        }

        public static Vector3Int operator *(Vector3Int a, Vector3Int b)
        {
            return new Vector3Int(a.x * b.x, a.y * b.y, a.z * b.z);
        }

        public static Vector3Int operator *(Vector3Int a, int b)
        {
            return new Vector3Int(a.x * b, a.y * b, a.z * b);
        }

        public static Vector3Int operator /(Vector3Int a, int b)
        {
            return new Vector3Int(a.x / b, a.y / b, a.z / b);
        }

        public static bool operator ==(Vector3Int lhs, Vector3Int rhs)
        {
            return lhs.x == rhs.x && lhs.y == rhs.y && lhs.z == rhs.z;
        }

        public static bool operator !=(Vector3Int lhs, Vector3Int rhs)
        {
            return !(lhs == rhs);
        }

        public override string ToString()
        {
            return $"<{x},{y},{z}>";
        }

        public bool Equals(Vector3Int other)
        {
            return x == other.x && y == other.y && z == other.z;
        }

        private int ComboHash(int a, int b)
        {
            uint rol5 = ((uint)a << 5) | ((uint)a >> 27);
            return ((int)rol5 + a) ^ b;
        }

        public override int GetHashCode()
        {
            return ComboHash(ComboHash(x, y), z);
        }

        public override bool Equals(object other)
        {
            if (!(other is Vector3Int)) return false;

            return Equals((Vector3Int)other);
        }
        public long ManhattanDistance(Vector3Int other)
        {
            return Math.Abs(x - other.x) + Math.Abs(y - other.y) + Math.Abs(z - other.z);
        }

        public static readonly Vector3Int Zero = new Vector3Int(0, 0, 0);
        public static readonly Vector3Int Up = new Vector3Int(0, 1, 0);
        public static readonly Vector3Int Down = new Vector3Int(0, -1, 0);
        public static readonly Vector3Int Left = new Vector3Int(-1, 0, 0);
        public static readonly Vector3Int Right = new Vector3Int(1, 0, 0);
        public static readonly Vector3Int Forward = new Vector3Int(-1, 0, 1);
        public static readonly Vector3Int Backward = new Vector3Int(1, 0, -1);

        public static readonly Vector3Int[] CardinalDirections = new Vector3Int[] { Up, Right, Down, Left };

    }
}