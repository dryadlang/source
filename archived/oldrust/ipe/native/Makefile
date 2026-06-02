# Detection of OS
ifeq ($(OS),Windows_NT)
    CC = ../../gcc/windows/bin/gcc.exe
    CFLAGS = -shared -O2
    LIBS = -lgdi32 -luser32 -lkernel32
    TARGET = ipe.dll
    CLEAN = del $(TARGET)
    SRC = ipe_helper.c
else
    CC = gcc
    CFLAGS = -shared -O2 -fPIC $(shell sdl2-config --cflags)
    LIBS = $(shell sdl2-config --libs)
    TARGET = ipe.so
    CLEAN = rm -f $(TARGET)
    SRC = ipe_helper_sdl2.c
endif

all: $(TARGET)

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(TARGET) $(LIBS)

clean:
	$(CLEAN)

