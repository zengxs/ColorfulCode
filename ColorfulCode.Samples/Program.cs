using System;

namespace ColorfulCode.Samples
{
    class Program
    {
        static void Main(string[] args)
        {
            // get a syntax
            SyntaxSet ss = SyntaxSet.LoadDefaults();
            Syntax syntax = ss.FindByExtension("cs"); // syntax for C#

            // get a theme
            ThemeSet ts = ThemeSet.LoadDefaults();
            Theme theme = ts["InspiredGitHub"]; // github syntax highlighting theme

            // highlight source code to html
            string sourceCode = @"using ColorfulCode;

namespace HelloWorld {
    class Program {
        static Main(string[] args) {
            Console.WriteLine(""Hello World"");
        }
    }
}
";
            string html = syntax.HighlightToHtml(sourceCode, theme);
            Console.WriteLine(html);
        }
    }
}
