

.PHONY : all
all: libmetaphor

.PHONY : libmetaphor
libmetaphor:
	rustc -g -L deps src/lib.rs
