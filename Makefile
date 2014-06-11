

.PHONY : all
all: libmetaphor

.PHONY : libmetaphor
libmetaphor:
	rustc -O -L deps src/lib.rs
