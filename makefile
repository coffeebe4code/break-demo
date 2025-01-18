CFLAGS=-MMD -std=c2x -Wall -Werror -Wextra -I./pieces
FFLAGS=-fsanitize=address -fsanitize=undefined
CC=cc
MMD=-MMD
Platforms=something
SDL3=3.1.6

TARGET=./target
SRCS=$(wildcard src/*/*.c)

OBJS=$(patsubst src/%.c,$(TARGET)/%.o,$(SRCS))
DEPS=$(OBJS:%.o=%.d)

EXE=break
LIBS=$(addprefix -l,)
VENDOR=./vendor

$(TARGET)/$(EXE): $(OBJS) $(TARGET)/main.o | $(VENDOR)/sdl_built
	$(CC) $(CFLAGS) -o $@ $^ $(LIBS) `pkg-config sdl3 --cflags --libs`

-include $(DEPS)

$(TARGET)/%.o: src/%.c
	$(CC) $(CFLAGS) $(MMD) -c $< -o $@ `pkg-config sdl3 --cflags --libs`

$(TARGET)/main.o: src/main.c
	$(CC) $(CFLAGS) -c $< -o $@ `pkg-config sdl3 --cflags --libs`

$(VENDOR)/sdl_built: $(VENDOR)/sdl_unpacked
	sudo apt install libxext-dev --yes
	cd $(VENDOR)/SDL3-$(SDL3) && mkdir build && cd build && cmake -DCMAKE_BUILD_TYPE=Release .. && cmake --build . --config Release --parallel && sudo cmake --install . --config Release
	touch $(VENDOR)/sdl_built
	
$(VENDOR)/sdl_unpacked: $(VENDOR)/SDL3-$(SDL3).tar.gz
	cd $(VENDOR) && tar -xvzf SDL3-$(SDL3).tar.gz
	touch $(VENDOR)/sdl_unpacked

$(VENDOR)/SDL3-$(SDL3).tar.gz:
	wget https://github.com/libsdl-org/SDL/releases/download/preview-$(SDL3)/SDL3-$(SDL3).tar.gz -P $(VENDOR)

.PHONY: clean
clean:
	rm -rf $(TARGET)
	mkdir $(TARGET)
	mkdir $(TARGET)/pieces
	mkdir $(TARGET)/controllers
