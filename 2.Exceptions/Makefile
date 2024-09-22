.PHONY: all help

CC = gcc
CFLAGS = -Wall -g

all: help

run: $(FILE).c
	$(CC) $(CFLAGS) -o $(FILE).out $(FILE).c
	./$(FILE).out

help:
	@echo "Usage: make run FILE=<filename_without_extension>"
	@echo "Example: make run FILE=hello"

clean:
	rm -f *.out
