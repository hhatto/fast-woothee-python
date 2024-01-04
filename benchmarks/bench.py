from benchmarker import Benchmarker
from ua_parser import user_agent_parser as uap_parser
from woothee import parse as woothee_parse
from fast_woothee import parse as fw_parse


ua = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.104 Safari/537.36'
n = 1000 * 100

with Benchmarker(n) as bench:

    @bench("uap")
    def _uap(bm):
        parse = uap_parser.Parse
        for i in bm:
            parse(ua)

    @bench("uap(non-cache)")
    def _uap_noncache(bm):
        parser = uap_parser.ParseUserAgent
        for i in bm:
            parser(ua)

    @bench("woothee")
    def _woothee(bm):
        parse = woothee_parse
        for i in bm:
            parse(ua)

    @bench("fast-woothee")
    def _fast_woothee(bm):
        parse = fw_parse
        for i in bm:
            parse(ua)
