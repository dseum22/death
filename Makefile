IDIR =./
CC=g++
CFLAGS=-I$(IDIR) -g -Wall -std=gnu++17
ODIR=./

_DEPS = Env.hh Graph.hh Heap.hh Tests.hh
DEPS = $(patsubst %,$(IDIR)/%,$(_DEPS))

_OBJ = main.o Env.o Graph.o Heap.o Tests.o
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))


$(ODIR)/%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

main: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)

.PHONY: clean

clean:
	rm -f $(ODIR)/*.o *~ core $(INCDIR)/*~
