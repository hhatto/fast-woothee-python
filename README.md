# fast-woothee-python [![](https://travis-ci.org/hhatto/fast-woothee-python.svg?branch=master)](https://travis-ci.org/hhatto/fast-woothee-python)

Python Bindings to [woothee-rust](https://github.com/woothee/woothee-rust).

## Requirements

* [setuptools-rust](https://github.com/PyO3/setuptools-rust)
* Nightly Rust

```
$ pip install setuptools_rust
$ rustup default nightly
```

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
## python version:      3.6.3
## python compiler:     GCC 4.2.1 Compatible Apple LLVM 8.1.0 (clang-802.0.42)
## python platform:     Darwin-16.7.0-x86_64-i386-64bit
## python executable:   /Users/hattori-h/.virtualenvs/py363/bin/python
## cpu model:           Intel(R) Core(TM) i5-5257U CPU @ 2.70GHz
## parameters:          loop=100000, cycle=1, extra=0

##                                       real    (total    = user    + sys)
uap                                    0.0875    0.0800    0.0800    0.0000
uap(non-cache)                        67.4911   65.7300   65.2100    0.5200
woothee                                1.9323    1.8800    1.8600    0.0200
fast-woothee                           1.4410    1.3300    1.3100    0.0200

## Ranking                               real
uap                                    0.0875  (100.0) ********************
fast-woothee                           1.4410  (  6.1) *
woothee                                1.9323  (  4.5) *
uap(non-cache)                        67.4911  (  0.1)

## Matrix                                real    [01]    [02]    [03]    [04]
[01] uap                               0.0875   100.0  1647.0  2208.5 77138.7
[02] fast-woothee                      1.4410     6.1   100.0   134.1  4683.7
[03] woothee                           1.9323     4.5    74.6   100.0  3492.8
[04] uap(non-cache)                   67.4911     0.1     2.1     2.9   100.0

```

[benchmark script](https://gist.github.com/hhatto/c951a981e8a3ee4d1bbcf96cb93d5f5e)
