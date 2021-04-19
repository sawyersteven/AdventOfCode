using System.Collections.Generic;

namespace AdventOfCode
{
    public class FixedSizeQueue<T> : Queue<T>
    {
        private readonly int size;

        public FixedSizeQueue(int maxSize)
        {
            size = maxSize;
        }

        public new void Enqueue(T item)
        {
            base.Enqueue(item);
            while (Count > size) base.Dequeue();
        }
    }
}