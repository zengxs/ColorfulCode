using System;
using System.Collections.Generic;
using System.Text;

namespace ColorfulCode
{
    public sealed class ThemeSet : IDisposable
    {
        internal readonly IntPtr _themeSetPtr;
        private readonly SortedDictionary<string, Theme> _themes;

        private ThemeSet(IntPtr ptr)
        {
            _themeSetPtr = ptr;

            var buf = new StringBuilder(10240);
            Bindings.find_all_themes(buf, buf.Capacity, _themeSetPtr);
            var themeNameList = buf.ToString().Split('\n');
            var list = new SortedDictionary<string, Theme>();
            foreach (var themeName in themeNameList)
            {
                list.Add(themeName, new Theme(this, themeName));
            }

            _themes = list;
        }

        public Theme this[string themeName]
        {
            get
            {
                try
                {
                    return _themes[themeName];
                }
                catch (KeyNotFoundException _)
                {
                    throw new ThemeNotFound(themeName);
                }
            }
        }

        public static ThemeSet LoadDefaults()
        {
            return new ThemeSet(Bindings.load_default_theme_set());
        }

        public void Dispose()
        {
            Bindings.release_theme_set(_themeSetPtr);
        }
    }

    public sealed class Theme
    {
        internal readonly ThemeSet _themeSet;
        public readonly string Name;

        internal Theme(ThemeSet themeSet, string themeName)
        {
            _themeSet = themeSet;
            Name = themeName;
        }
    }
}
