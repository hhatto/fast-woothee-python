# fast-woothee-python

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
## python version:      3.6.2
## python compiler:     GCC 4.2.1 Compatible Apple LLVM 8.0.0 (clang-800.0.42.1)
## python platform:     Darwin-15.6.0-x86_64-i386-64bit
## python executable:   python
## cpu model:           Intel(R) Core(TM) i5-5257U CPU @ 2.70GHz
## parameters:          loop=100000, cycle=1, extra=0

##                                       real    (total    = user    + sys)
uap                                    0.0832    0.0800    0.0800    0.0000
woothee                                2.0031    1.9100    1.8900    0.0200
fast-woothee                           1.3058    1.2800    1.2700    0.0100

## Ranking                               real
uap                                    0.0832  (100.0) ********************
fast-woothee                           1.3058  (  6.4) *
woothee                                2.0031  (  4.2) *

## Matrix                                real    [01]    [02]    [03]
[01] uap                               0.0832   100.0  1569.5  2407.6
[02] fast-woothee                      1.3058     6.4   100.0   153.4
[03] woothee                           2.0031     4.2    65.2   100.0
```

[benchmark script](https://gist.github.com/hhatto/c951a981e8a3ee4d1bbcf96cb93d5f5e)
