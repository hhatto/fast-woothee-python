all:
	@echo "usage"

test:
	rm -rf woothee-python
	git clone https://github.com/woothee/woothee-python.git
	cd woothee-python && git submodule init && git submodule update
	cd woothee-python && sed 's/woothee/fast_woothee/' tests/test.py > temp_test.py
	cd woothee-python && head -n236 temp_test.py > tests/test.py && tail -n51 temp_test.py >> tests/test.py
	# cd woothee-python && mv temp_test.py tests/test.py
	rm -f temp_test.py
	cd woothee-python && make
