CC = ../../gcc/windows/bin/gcc.exe
CFLAGS = -shared -O2
LIBS = -lgdi32 -luser32 -lkernel32

all: ipe.dll

ipe.dll: ipe_helper.c
	$(CC) $(CFLAGS) ipe_helper.c -o ipe.dll $(LIBS)

clean:
	del ipe.dll
