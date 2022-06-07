SUFFIX ?=
CPPFLAGS ?=

OUTLIB := libunwind$(SUFFIX).a

CC      := clang
CXX     := clang++
AR      := llvm-ar
ARFLAGS := rcs

CPP_OBJLIST := libunwind.o
C_OBJLIST   := UnwindLevel1.o UnwindLevel1-gcc-ext.o
S_OBJLIST   := UnwindRegistersRestore.o UnwindRegistersSave.o

CFLAGS   := -std=c99
CXXFLAGS := -std=c++11 -nostdinc++ -fno-exceptions -fno-rtti
CPPFLAGS := $(CPPFLAGS) -target mipsel-unknown-unknown -mcpu=mips2 \
	-msingle-float -fstrict-aliasing -funwind-tables -O3 \
	-D __LITTLE_ENDIAN__ -D __ELF__ -D _LIBUNWIND_IS_BAREMETAL \
	-D _LIBUNWIND_HAS_NO_THREADS -D _LIBUNWIND_IS_NATIVE_ONLY \
	-DNDEBUG \
	-I /usr/local/pspdev/psp/include/ \
	-I include

all:
	make ../psp/libunwind.a
	SUFFIX=_lto CPPFLAGS=-flto make ../psp/libunwind_lto.a

../psp/$(OUTLIB): $(OUTLIB)
	cp $^ ../psp/
	touch ../psp/build.rs
	make clean

$(OUTLIB): $(CPP_OBJLIST) $(C_OBJLIST) $(S_OBJLIST)
	$(AR) $(ARFLAGS) $(OUTLIB) $^

$(CPP_OBJLIST): %.o: src/%.cpp
	$(COMPILE.cc) $^

$(C_OBJLIST): %.o: src/%.c
$(S_OBJLIST): %.o: src/%.S
$(C_OBJLIST) $(S_OBJLIST):
	$(COMPILE.c) $^

.PHONY: all clean patch

patch:
	git submodule update --init --depth 1 -- ./rustc
	(cd rustc; git submodule update --init --depth 1 -- src/llvm-project)
	cp -r rustc/src/llvm-project/libunwind/{src,include} .
	patch -p0 < ./no-sdc1.patch

clean:
	rm -r *.o
