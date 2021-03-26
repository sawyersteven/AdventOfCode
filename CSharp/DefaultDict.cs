using System;
using System.Collections.Generic;

namespace AdventOfCode
{
    public class DefaultDict<Tk, Tv> : Dictionary<Tk, Tv> where Tv : struct
    {
        public new Tv this[Tk key]
        {
            get
            {
                if (base.ContainsKey(key)) return base[key];
                return Activator.CreateInstance<Tv>();
            }
            set
            {
                base[key] = value;
            }
        }

    }
}