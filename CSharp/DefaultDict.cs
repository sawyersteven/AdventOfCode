using System;
using System.Collections.Generic;

namespace AdventOfCode
{
    public class DefaultDict<Tk, Tv> : Dictionary<Tk, Tv> where Tv : new()
    {
        private Tv dv;
        private bool useDv = false;

        public DefaultDict() { }

        public DefaultDict(Tv defaultValue)
        {
            dv = defaultValue;
            useDv = true;
        }

        public new Tv this[Tk key]
        {
            get
            {
                if (base.ContainsKey(key)) return base[key];
                Tv v = useDv ? dv : new Tv();
                this[key] = v;
                return v;
            }
            set
            {
                base[key] = value;
            }
        }

    }
}