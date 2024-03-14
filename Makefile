CC = clang
CFLAGS = -O2 -Wall

collatz: collatz.o
	$(CC) $(CFLAGS) -o collatz collatz.o

clean:
	-rm -f collatz.o collatz
