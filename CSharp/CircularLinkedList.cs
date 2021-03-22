using System;
using System.Collections;
using System.Collections.Generic;

namespace AdventOfCode
{
    public class CircularLinkedListNode<T>
    {
        public T Value;
        public CircularLinkedListNode<T> Next;
        public CircularLinkedListNode<T> Previous;

        public CircularLinkedListNode(T value)
        {
            Value = value;
        }
    }

    /// <summary>
    /// Mostly just a copy of LinkedList<T> but the last node's `.Next` points
    /// back to the first node so the list can be rotated as much as needed
    /// in either direction without worrying about reaching the end
    /// </summary>
    public class CircularLinkedList<T> : IEnumerable<CircularLinkedListNode<T>>
    {
        private CircularLinkedListNode<T> head;
        private int _Count;
        public int Count => _Count;

        public CircularLinkedListNode<T> Last => head == null ? null : head.Previous;
        public CircularLinkedListNode<T> First => head;

        public CircularLinkedListNode<T> AddLast(T value)
        {
            CircularLinkedListNode<T> result = new CircularLinkedListNode<T>(value);
            if (head == null)
            {
                InsertFirstNode(result);
            }
            else
            {
                InsertBefore(head, result);
            }
            return result;
        }

        public void Remove(CircularLinkedListNode<T> node)
        {
            if (node.Next == node)
            {
                head = null;
            }
            else
            {
                node.Next.Previous = node.Previous;
                node.Previous.Next = node.Next;
                if (head == node)
                {
                    head = node.Next;
                }
            }
            _Count--;
        }

        public void AddAfter(CircularLinkedListNode<T> origin, CircularLinkedListNode<T> node)
        {
            InsertBefore(origin.Next, node);
        }

        private void InsertFirstNode(CircularLinkedListNode<T> node)
        {
            node.Next = node;
            node.Previous = node;
            head = node;
            _Count++;
        }

        private void InsertBefore(CircularLinkedListNode<T> origin, CircularLinkedListNode<T> node)
        {
            node.Next = origin;
            node.Previous = origin.Previous;
            origin.Previous.Next = node;
            origin.Previous = node;
            _Count++;
        }

        public IEnumerator<CircularLinkedListNode<T>> GetEnumerator()
        {
            CircularLinkedListNode<T> current = head;
            do
            {
                yield return current;
                current = current.Next;
            } while (current != head);
        }

        public IEnumerable<T> GetValues()
        {
            CircularLinkedListNode<T> current = head;
            do
            {
                yield return current.Value;
                current = current.Next;
            } while (current != head);
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            throw new NotImplementedException();
        }
    }
}