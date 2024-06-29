.PHONY: all

RAW = $(wildcard data/*/*.raw)
IN = $(RAW:.raw=.in)

all: $(IN)

data/%.in: data/%.raw
	cargo run --release --bin eval < $< > $@
