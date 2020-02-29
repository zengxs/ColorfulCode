using System;
using System.Runtime.InteropServices;
using System.Text;

namespace ColorfulCode
{
    static class Bindings
    {
        private const string NativeLibrary = "syntect";

        [DllImport(NativeLibrary, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr load_default_syntax_set(bool newlines);

        [DllImport(NativeLibrary, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void release_syntax_set(IntPtr ssPtr);

        [DllImport(NativeLibrary, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int find_all_syntaxes(StringBuilder buf, int bufSize, IntPtr ssPtr);

        [DllImport(NativeLibrary, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int find_syntax_by_extension(StringBuilder buf, int bufSize, IntPtr ssPtr, string ext);

        [DllImport(NativeLibrary, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int find_syntax_by_token(StringBuilder buf, int bufSize, IntPtr ssPtr, string ext);

        [DllImport(NativeLibrary, CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr load_default_theme_set();

        [DllImport(NativeLibrary, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void release_theme_set(IntPtr tsPtr);

        [DllImport(NativeLibrary, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int find_all_themes(StringBuilder buf, int bufSize, IntPtr tsPtr);

        [DllImport(NativeLibrary, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int highlight_to_html(
            StringBuilder buf,
            int bufSize,
            string src,
            IntPtr ssPtr,
            string syntaxName,
            IntPtr tsPtr,
            string themeName);
    }
}
