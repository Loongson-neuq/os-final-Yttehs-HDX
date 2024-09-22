.PHONY: all clean

CC = gcc
CFLAGS =

SRC = $(shell find . -name '*.c')
BUILD_DIR = build
OBJ = $(patsubst src/%.c, $(BUILD_DIR)/%.o, $(SRC))

TARGET = $(BUILD_DIR)/my_program

all: run

run: build
	@$(TARGET)

build: $(TARGET)

$(TARGET): $(OBJ)
	@mkdir -p $(BUILD_DIR)
	$(CC) -o $@ $^

$(BUILD_DIR)/%.o: src/%.c
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(BUILD_DIR)
