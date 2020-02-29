# ColorfulCode

[![Nuget](https://img.shields.io/nuget/v/ColorfulCode?logo=nuget)](https://www.nuget.org/packages/ColorfulCode)
![Repository Size](https://img.shields.io/github/repo-size/zengxs/ColorfulCode?logo=git)
[![GitHub](https://img.shields.io/github/license/zengxs/ColorfulCode)](LICENSE)

ColorfulCode is a syntax highlighter library for C#/.NET Core that uses
[sublime-text syntax definitions](http://www.sublimetext.com/docs/3/syntax.html#include-syntax).
It is a wrapper for native [rust](https://www.rust-lang.org) library [syntect][syntect].

Benefit from [syntect][syntect] syntax highlighting engine, ColorfulCode is very fast.
See [the performance of syntect](https://github.com/trishume/syntect#performance) for more info.

[syntect]: https://github.com/trishume/syntect

## Getting Started

### Installation
ColorfulCode is available as [a NuGet package from nuget.org](https://www.nuget.org/packages/ColorfulCode).
It can be added to a project in a numbers of ways, depending on the project type and tools used:

##### `dotnet` CLI
```sh
$ dotnet add package ColorfulCode --version 1.0.0-preview1 
```

##### Visual Studio
```
PM> Install-Package ColorfulCode -Version 1.0.0-preview1
```

##### `.csproj`
```xml
<PackageReference Include="ColorfulCode" Version="1.0.0-preview1" />
```

### Supported Platforms
ColorfulCode runs on the following platforms and .NET Core versions:

| OS | Version | Architectures | .NET Runtimes |
|:---|:--------|:--------------|:--------------|
| macOS | \> 10.12 | x64 | 3.1 / 3.0 / 2.1 |
| Linux | | x86 / x64 | 3.1 / 3.0 / 2.1 |
| Windows | | x86 / x64 | 3.1 / 3.0 / 2.1 |

### Example Code
Prints highlighted html of C# source code to the terminal:
```c#
using ColorfulCode;

// get a syntax
SyntaxSet ss = SyntaxSet.LoadDefaults();
Syntax syntax = ss.FindByExtension("cs");  // syntax for C#

// get a theme
ThemeSet ts = ThemeSet.LoadDefaults();
Theme theme = ts["InspiredGitHub"];  // github syntax highlighting theme

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
```

Run the code, you can see the highlighted html on your console:
```html
<pre style="background-color:#ffffff;">
<span style="font-weight:bold;color:#a71d5d;">using </span><span style="color:#323232;">ColorfulCode;
</span><span style="color:#323232;">
</span><span style="font-weight:bold;color:#a71d5d;">namespace </span><span style="color:#323232;">HelloWorld {
</span><span style="color:#323232;">    </span><span style="font-weight:bold;color:#a71d5d;">class </span><span style="color:#0086b3;">Program </span><span style="color:#323232;">{
</span><span style="color:#323232;">        </span><span style="font-weight:bold;color:#a71d5d;">static </span><span style="font-weight:bold;color:#795da3;">Main</span><span style="color:#323232;">(</span><span style="font-weight:bold;color:#a71d5d;">string</span><span style="color:#323232;">[] args) {
</span><span style="color:#323232;">            Console.WriteLine(</span><span style="color:#183691;">&quot;Hello World&quot;</span><span style="color:#323232;">);
</span><span style="color:#323232;">        }
</span><span style="color:#323232;">    }
</span><span style="color:#323232;">}
</span></pre>
```

### Supported Languages
Default SyntaxSet:

`Plain Text`, `ASP`, `HTML (ASP)`, `ActionScript`, `AppleScript`, `Batch File`, `NAnt Build File`, `C#`, `C++`, `C`, `CSS`, `Clojure`, `D`, `Diff`, `Erlang`, `HTML (Erlang)`, `Go`, `Graphviz (DOT)`, `Groovy`, `HTML`, `Haskell`, `Literate Haskell`, `Java Server Page (JSP)`, `Java`, `JavaDoc`, `Java Properties`, `JSON`, `JavaScript`, `Regular Expressions (Javascript)`, `BibTeX`, `LaTeX Log`, `LaTeX`, `TeX`, `Lisp`, `Lua`, `Make Output`, `Makefile`, `Markdown`, `MultiMarkdown`, `MATLAB`, `OCaml`, `OCamllex`, `OCamlyacc`, `camlp4`, `Objective-C++`, `Objective-C`, `PHP Source`, `PHP`, `Pascal`, `Perl`, `Python`, `Regular Expressions (Python)`, `R Console`, `R`, `Rd (R Documentation)`, `HTML (Rails)`, `JavaScript (Rails)`, `Ruby Haml`, `Ruby on Rails`, `SQL (Rails)`, `Regular Expression`, `reStructuredText`, `Ruby`, `Cargo Build Results`, `Rust`, `SQL`, `Scala`, `Bourne Again Shell (bash)`, `Shell-Unix-Generic`, `commands-builtin-shell-bash`, `HTML (Tcl)`, `Tcl`, `Textile`, `XML`, `YAML`

### Supported Themes
Default ThemeSet:

* `InspiredGitHub`
* `Solarized (dark)`
* `Solarized (light)`
* `base16-eighties.dark`
* `base16-mocha.dark`
* `base16-ocean.dark`
* `base16-ocean.light`

## Features / Goals
- [x] Support highlight code to HTML
- [ ] **Syntax**: Include more syntax definitions of commonly used language into default syntax set
- [ ] **Syntax**: Support custom syntax definitions
- [ ] **Theme**: Support custom theme definitions
- [ ] Support highlight code for terminal

## License
ColorfulCode is licensed under the [Apache-2.0 license](LICENSE).
