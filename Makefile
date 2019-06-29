rabbitcc: src/main.rs

test: rabbitcc
	./test.sh

clean:
	rm -rf target/ bin/* tmp/*

.PHONY: test clean
