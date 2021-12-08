using System;
using System.Collections.Generic;

// Easy wrapper for making a memoized function call
public class CachedFunction<TSource, TReturn>
{
    private readonly Func<TSource, TReturn> fn;
    private Dictionary<TSource, TReturn> cache;

    public CachedFunction(Func<TSource, TReturn> function)
    {
        fn = function;
        cache = new Dictionary<TSource, TReturn>();
    }

    public TReturn Call(TSource arg)
    {
        if (!cache.ContainsKey(arg))
        {
            cache[arg] = fn(arg);
        }
        return cache[arg];
    }
}