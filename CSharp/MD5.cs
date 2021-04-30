namespace AdventOfCode
{
    static class MD5
    {
        private static System.Security.Cryptography.MD5 md5 = System.Security.Cryptography.MD5.Create();

        public static string HexDigest(string input, int iters = 1)
        {
            byte[] bytes = System.Text.Encoding.ASCII.GetBytes(input);
            for (int _ = 0; _ < iters; _++)
            {
                bytes = ToLowercaseHexBytes(md5.ComputeHash(bytes));
            }
            return System.Text.Encoding.ASCII.GetString(bytes);
        }

        public static byte[] ToLowercaseHexBytes(byte[] hash)
        {
            byte[] ret = new byte[32];
            for (int i = 0, j = 0; i < hash.Length; i++, j += 2)
            {
                ret[j] = (byte)((hash[i] / 16) + 48);
                // some bit-fiddling to see if the value is over 57 so it can have
                // 39 added to it in order to get a lower-case ascii letter value
                // This only works on unsigned values.
                ret[j] += (byte)((57 + (~ret[j] + 1) >> 31 & 1) * 39);
                ret[j + 1] = (byte)((hash[i] % 16) + 48);
                ret[j + 1] += (byte)((57 + (~ret[j + 1] + 1) >> 31 & 1) * 39);
            }
            return ret;
        }
    }
}