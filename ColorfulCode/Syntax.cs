using System;
using System.Collections.Generic;
using System.Text;

namespace ColorfulCode
{
    public sealed class SyntaxSet : IDisposable
    {
        internal readonly IntPtr _syntaxSetPointer;
        private readonly SortedList<string, Syntax> _syntaxes;

        private SyntaxSet(IntPtr ptr)
        {
            _syntaxSetPointer = ptr;
            var buf = new StringBuilder(10240);
            // TODO: handle errors
            var errno = Bindings.find_all_syntaxes(buf, buf.Capacity, _syntaxSetPointer);
            var syntaxNameList = buf.ToString().Split('\n');
            var syntaxList = new SortedList<string, Syntax>();
            foreach (var syntaxName in syntaxNameList)
            {
                syntaxList.Add(syntaxName, new Syntax(this, syntaxName));
            }

            _syntaxes = syntaxList;
        }

        public Syntax this[string syntaxName]
        {
            get
            {
                try
                {
                    return _syntaxes[syntaxName];
                }
                catch (KeyNotFoundException e)
                {
                    throw new SyntaxNotFound(syntaxName);
                }
            }
        }

        public Syntax FindByExtension(string ext)
        {
            var buf = new StringBuilder(1024);
            var errno = Bindings.find_syntax_by_extension(buf, buf.Capacity, _syntaxSetPointer, ext);
            if (errno == (int) ErrorCode.SyntaxNotFound)
            {
                throw new SyntaxNotFound(ext, "extension");
            }

            var syntaxName = buf.ToString();
            return this[syntaxName];
        }

        public Syntax FindByToken(string token)
        {
            var buf = new StringBuilder(1024);
            var errno = Bindings.find_syntax_by_token(buf, buf.Capacity, _syntaxSetPointer, token);
            if (errno == (int) ErrorCode.SyntaxNotFound)
            {
                throw new SyntaxNotFound(token, "token");
            }

            var syntaxName = buf.ToString();
            return this[syntaxName];
        }

        public static SyntaxSet LoadDefaults(bool withNewLines = true)
        {
            return new SyntaxSet(Bindings.load_default_syntax_set(withNewLines));
        }

        public void Dispose()
        {
            Bindings.release_syntax_set(_syntaxSetPointer);
        }
    }

    public sealed class Syntax
    {
        public readonly SyntaxSet SyntaxSet;
        public readonly string Name;

        internal Syntax(SyntaxSet syntaxSet, string name)
        {
            SyntaxSet = syntaxSet;
            Name = name;
        }

        public string HighlightToHtml(string sourceCode, Theme theme)
        {
            var buf = new StringBuilder(40960);
            var errno = Bindings.highlight_to_html(
                buf, buf.Capacity,
                sourceCode,
                SyntaxSet._syntaxSetPointer, this.Name,
                theme._themeSet._themeSetPtr, theme.Name);
            return buf.ToString();
        }
    }
}
