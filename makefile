CFLAGS=-MMD -std=c2x -Wall -Werror -Wextra -I./pieces
FFLAGS=-fsanitize=address -fsanitize=undefined
CC=cc
CFLAGS=-Wall -Wextra -pedantic -std=c11
MMD=-MMD
Platforms=something
SDL2=3.1.6

TARGET=./target
SRCS=$(wildcard src/*/*.c)

OBJS=$(patsubst src/%.c,$(TARGET)/%.o,$(SRCS))
DEPS=$(OBJS:%.o=%.d)

EXE=break
LIBS=$(addprefix -l,)
VENDOR=./vendor

$(TARGET)/$(EXE): $(OBJS) $(TARGET)/main.o
	$(CC) -o $@ $^ $(LIBS)

-include $(DEPS)

$(TARGET)/%.o: src/%.c
	$(CC) $(CFLAGS) $(MMD) -c $< -o $@ 

$(TARGET)/main.o: src/main.c
	$(CC) $(CFLAGS) -c $< -o $@ 

target/download:
	wget https://github.com/libsdl-org/SDL/releases/download/preview-$(SDL2)/SDL3-$(SDL2).tar.gz -P $(VENDOR)
	#wget https://github.com/libsdl-org/SDL/releases/download/release-$(SDL2)/SDL2-$(SDL2)-win32-x64.tar.gz -P $(VENDOR)
	#wget https://github.com/libsdl-org/SDL/releases/download/release-$(SDL2)/SDL2-$(SDL2)-win32-arm64.tar.gz -P $(VENDOR)

.PHONY: clean
clean:
	rm -rf $(TARGET)
	mkdir $(TARGET)
	mkdir $(TARGET)/pieces
	mkdir $(TARGET)/controllers
