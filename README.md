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

`uv run python benchmarks/bench.py`

```
## benchmarker:         release 4.0.1 (for python)
## python version:      3.15.2
## python compiler:     Clang 23.1.3
## python platform:     macOS-27.0.1-arm64-arm-64bit-Mach-O
## python executable:   /Users/hideohattori/work/fast-woothee-python/.venv/bin/python
## cpu model:           Apple M3
## parameters:          loop=100000, cycle=1, extra=0

##                                       real    (total    = user    + sys)
uap                                    0.0075    0.0100    0.0100    0.0000
uap(non-cache)                         0.1201    0.1200    0.1200    0.0000
woothee                                0.3324    0.3300    0.3300    0.0000
fast-woothee                           0.1705    0.1800    0.1700    0.0100

## Ranking                               real
uap                                    0.0075  (100.0) ********************
uap(non-cache)                         0.1201  (  6.2) *
fast-woothee                           0.1705  (  4.4) *
woothee                                0.3324  (  2.3)

## Matrix                                real    [01]    [02]    [03]    [04]
[01] uap                               0.0075   100.0  1601.0  2272.9  4432.4
[02] uap(non-cache)                    0.1201     6.2   100.0   142.0   276.9
[03] fast-woothee                      0.1705     4.4    70.4   100.0   195.0
[04] woothee                           0.3324     2.3    36.1    51.3   100.0
```

## For Developer

install depends:
```
$ uv sync
```

build with maturin:
```
$ maturin build
```
