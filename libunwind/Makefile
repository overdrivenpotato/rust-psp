.PHONY: clean all

all:
	git submodule update --init --depth 1 -- ./rustc
	(cd rustc; git submodule update --init --depth 1 -- src/llvm-project)

	cp -r rustc/src/llvm-project/libunwind/{src,include} .
	patch -p0 < ./no-sdc1.patch

	$(CXX) -target mips -mcpu=mips2 -msingle-float -mlittle-endian -std=c99 -std=c++11 \
		-nostdinc++ -fno-exceptions -fno-rtti -fstrict-aliasing -funwind-tables \
		-D __LITTLE_ENDIAN__ -D __ELF__ -D _LIBUNWIND_IS_BAREMETAL \
		-D _LIBUNWIND_HAS_NO_THREADS -D _LIBUNWIND_IS_NATIVE_ONLY \
		-O3 \
		-DNDEBUG \
		-I /usr/local/pspdev/psp/include/ \
		-I include -c \
		src/UnwindLevel1-gcc-ext.c \
		src/UnwindLevel1.c \
		src/UnwindRegistersRestore.S \
		src/UnwindRegistersSave.S \
		src/libunwind.cpp

	psp-ar rcs libunwind.a *.o
	rm *.o

	touch ../build.rs

clean:
	-rm -r src include libunwind.a
