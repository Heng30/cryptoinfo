all:
	gcc -O2 -shared -fPIC -o libfoo.so foo.c foo.h

install: mkdir all
	mv -f libfoo.so ../../lib
	cp -f foo.h ../../include/foo
	-rm -f *.o
	@echo -e "Install libfoo.so successfully!\n"

mkdir:
	-mkdir -p ../../include/foo


