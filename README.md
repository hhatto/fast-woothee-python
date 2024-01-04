# fast-woothee-python [![build](https://github.com/hhatto/fast-woothee-python/actions/workflows/python-package.yml/badge.svg?branch=main)](https://github.com/hhatto/fast-woothee-python/actions/workflows/python-package.yml)

Python Bindings to [woothee-rust](https://github.com/woothee/woothee-rust).

## Installation

```
$ pip install --upgrade git+https://github.com/hhatto/fast-woothee-python
```

## Usage

```python
import fast_woothee as woothee

ua = "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)"
print(woothee.parse(ua))
```

## Performance

```
## benchmarker:         release 4.0.1 (for python)
## python version:      3.11.7
## python compiler:     Clang 15.0.0 (clang-1500.1.0.2.5)
## python platform:     macOS-14.1.2-x86_64-i386-64bit
## python executable:   python
## cpu model:           Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz
## parameters:          loop=100000, cycle=1, extra=0

##                                       real    (total    = user    + sys)
uap                                    0.0436    0.0400    0.0400    0.0000
uap(non-cache)                         0.0480    0.0500    0.0500    0.0000
woothee                                0.6790    0.6800    0.6800    0.0000
fast-woothee                           0.3756    0.3700    0.3700    0.0000

## Ranking                               real
uap                                    0.0436  (100.0) ********************
uap(non-cache)                         0.0480  ( 90.8) ******************
fast-woothee                           0.3756  ( 11.6) **
woothee                                0.6790  (  6.4) *

## Matrix                                real    [01]    [02]    [03]    [04]
[01] uap                               0.0436   100.0   110.1   860.8  1556.4
[02] uap(non-cache)                    0.0480    90.8   100.0   781.9  1413.5
[03] fast-woothee                      0.3756    11.6    12.8   100.0   180.8
[04] woothee                           0.6790     6.4     7.1    55.3   100.0
```
