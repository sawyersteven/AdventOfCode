using System;
using System.Collections.Generic;

namespace AdventOfCode
{
    public class DefaultDict<Tk, Tv> : Dictionary<Tk, Tv> where Tv : new()
    {
        public new Tv this[Tk key]
        {
            get
            {
                if (base.ContainsKey(key)) return base[key];
                this[key] = new Tv();
                return this[key];
            }
            set
            {
                base[key] = value;
            }
        }

    }
}