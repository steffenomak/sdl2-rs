all:
	rustpkg build sdl2
	rustpkg install sdl2
	rustpkg build hello-world
	rustpkg install hello-world

all_with_debug:
	rustpkg build -Zextra-debug-info sdl2
	rustpkg install sdl2
	rustpkg build -Zextra-debug-info hello-world
	rustpkg install hello-world

clean:
	rustpkg clean sdl2
	rustpkg clean hello-world

run:
	./bin/hello-world

debug:
	gdb ./bin/hello-world

