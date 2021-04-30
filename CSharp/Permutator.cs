using System;
using System.Collections.Generic;

namespace AdventOfCode
{
    public static class Permutator
    {
        public static IEnumerable<IList<T>> Permutate<T>(IList<T> sequence)
        {
            foreach (IList<T> i in _Permutate(sequence, sequence.Count))
            {
                yield return i;
            }
        }

        private static IEnumerable<IList<T>> _Permutate<T>(IList<T> sequence, int count)
        {
            if (count == 1) yield return sequence;

            for (int i = 0; i < count; i++)
            {
                foreach (IList<T> perm in _Permutate(sequence, count - 1))
                {
                    yield return perm;
                }
                T tmp = sequence[count - 1];
                for (int j = count - 1; j > 0; j--)
                {
                    sequence[j] = sequence[j - 1];
                }
                sequence[0] = tmp;
            }
        }

        public static IEnumerable<T[]> Combinations<T>(int len, T[] seq)
        {
            int n = seq.Length;
            T[] result = new T[len];
            Stack<int> stack = new Stack<int>();
            stack.Push(0);

            while (stack.Count > 0)
            {
                int index = stack.Count - 1;
                int value = stack.Pop();

                while (value < n)
                {
                    result[index++] = seq[value];
                    value++;
                    stack.Push(value);

                    if (index == len)
                    {
                        T[] cpy = new T[len];
                        result.CopyTo(cpy, 0);
                        yield return cpy;
                        break;
                    }
                }
            }
        }
    }
}