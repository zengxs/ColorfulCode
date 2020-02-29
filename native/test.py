#!/usr/bin/env python3

import gc
import resource
import sys
from ctypes import CDLL, c_void_p, c_char_p, c_int
from timeit import timeit


lib = CDLL('target/debug/libsyntect.dylib')

lib.load_default_theme_set.restype = c_void_p
lib.load_default_syntax_set.restype = c_void_p
lib.find_syntax_by_extension.argtypes = (c_char_p, c_int, c_void_p, c_char_p)  # buf, buf_size, ss_ptr, ext
lib.highlight_to_html.argtypes = (c_char_p, c_int, c_char_p, c_void_p, c_char_p, c_void_p, c_char_p)  # buf, buf_size, src, src, ss_ptr, syntax_name, ts_ptr, theme_name


def memory_usage():
    gc.collect()
    mem = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    if sys.platform == 'darwin':
        print(f'  * Current memory usage: {mem / 1000} KB')
    elif sys.platform == 'linux':
        print(f'  * Current memory usage: {mem} KB')
    else:
        raise NotImplementedError


print('> Trying to load default ThemeSet...')
memory_usage()
ts = lib.load_default_theme_set()
ts = c_void_p(ts)
print('  * Load it OK')
buf = bytes(10000)
lib.find_all_themes(buf, len(buf), ts)
themes = buf.strip(b'\x00').decode().split('\n')
print(f'  * Loaded themes: {themes}')
lib.release_theme_set(ts)
print('  * Release it OK')
memory_usage()
ts = c_void_p(lib.load_default_theme_set())  # load it again for rest tests

loop = 100
t = timeit(lambda: lib.release_theme_set(c_void_p(lib.load_default_theme_set())), number=loop)
print(f'  * {loop} loops, total cost {t} sec, {t / loop * 1000} nsec per loop')
memory_usage()

print('> Trying to load default SyntaxSet...')
memory_usage()
ss = lib.load_default_syntax_set(True)
ss = c_void_p(ss)
print('  * Load it OK')
lib.find_all_syntaxes(buf, len(buf), ss)
syntaxes = buf.strip(b'\00').decode().split('\n')
print(f'  * Loaded syntaxes: {syntaxes}')
lib.release_syntax_set(ss)
print('  * Release it OK')
memory_usage()
ss = c_void_p(lib.load_default_syntax_set())  # load it again for rest tests

loop = 10
t = timeit(lambda: lib.release_syntax_set(c_void_p(lib.load_default_syntax_set())), number=loop)
print(f'  * {loop} loops, total cost {t} sec, {t / loop} sec per loop')
memory_usage()

print('> Trying to find syntax from SyntaxSet...')
buf = bytes(100)
errno = lib.find_syntax_by_extension(buf, len(buf), ss, b'py')
syntax_name = buf.strip(b'\x00').decode()
print(f'  * Found syntax: "{syntax_name}"')

loop = 100000
t = timeit(lambda: lib.find_syntax_by_extension(buf, len(buf), ss, b'rs'), number=loop)
print(f'  * {loop} loops, total cost {t} sec, {t / loop * 1000} nsec per loop')

source = '''import sys, os
print("hello world")
'''.encode()

print('> Trying to highlight source code to html...')
buf = bytes(40960)
errno = lib.highlight_to_html(buf, len(buf), source, ss, b'Python', ts, b'InspiredGitHub')
print('  * Highlighted HTML: {} ...'.format(buf.decode().replace('\n', '')[:50]))
memory_usage()

loop = 10000
t = timeit(lambda: lib.highlight_to_html(buf, len(buf), source, ss, b'Python', ts, b'Solarized (light)'), number=loop)
print(f'  * {loop} loops, total cost {t} sec, {t / loop * 1000} nsec per loop')
memory_usage()
