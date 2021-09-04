-include Make.vars

ifndef LLVM_MOS
$(error Please set LLVM_MOS in Make.vars)
endif

ifndef LLVM_MOS_SDK
$(error Please set LLVM_MOS_SDK in Make.vars)
endif

CLANG		= $(LLVM_MOS)/bin/clang --config $(LLVM_MOS_SDK)/commodore/64.cfg -O2

C_SRCS		= $(wildcard src/*.c)
ASM_SRCS	= $(wlidcard src/*.s)
RUST_SRCS	= $(wildcard src/*.rs)

OUTDIR		= _build

OBJS		= $(patsubst src/%.c, $(OUTDIR)/%.c.o, $(C_SRCS)) \
		  $(patsubst src/%.s, $(OUTDIR)/%.s.o, $(ASM_SRCS)) \
		  $(patsubst src/%.rs, $(OUTDIR)/%.rs.ll, $(RUST_SRCS))
PRG		= $(OUTDIR)/charset.prg

.PHONY: all clean

all: $(PRG)

clean:
	rm -rf _build

$(OUTDIR)/%.c.o: src/%.c
	mkdir -p $(OUTDIR)
	$(CLANG) -c -o $@ $^

$(OUTDIR)/%.rs.ll: src/%.rs
	mkdir -p $(OUTDIR)
	rustc --crate-type=rlib \
	  -C debuginfo=0 \
	  -C opt-level=1 \
	  --emit=llvm-ir \
	  -o $@ $^

$(PRG): $(OBJS)
	mkdir -p $(OUTDIR)
	$(CLANG) -o $@ $^
