using System;

namespace ColorfulCode
{
    internal enum ErrorCode
    {
        General = -1,
        BufferTooSmall = -10,
        BufferNullPointer = -11,
        SyntaxNotFound = -20,
        SyntaxSetNullPointer = -21,
        ThemeNotFound = -30,
        ThemeSetNullPointer = -31,
    }

    public class WrapperException : SystemException
    {
    }

    public class SyntaxNotFound : SystemException
    {
        public SyntaxNotFound(string key, string type = "syntax")
            : base($"The given {type} '{key}' was not found in the syntax set.")
        {
        }
    }

    public class ThemeNotFound : SystemException
    {
        public ThemeNotFound(string key)
            : base($"The given theme '{key}' was not found in the theme set.")
        {
        }
    }
}
