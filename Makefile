.PHONY:
	all
	rust
	rtest

all: rust lisp.lib

lisp.lib: as_lib.lisp Makefile
	sbcl/sbcl.exe --core sbcl/sbcl.core --script as_lib.lisp

rust:
	cd host_test && cargo build

rtest:
	host_test/target/debug/host_test
