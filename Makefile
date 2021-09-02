-include Make.vars

ifndef LLVM_MOS
$(error Please set LLVM_MOS in Make.vars)
endif

ifndef LLVM_MOS_SDK
$(error Please set LLVM_MOS_SDK in Make.vars)
endif

CLANG		= $(LLVM_MOS)/bin/clang --config $(LLVM_MOS_SDK)/commodore/64.cfg -O2

C_SRCS		= $(wildcard *.c)
ASM_SRCS	= $(wlidcard *.s)
RUST_SRCS	= $(wildcard *.rs)

OUTDIR		= _build

OBJS		= $(patsubst %.c, $(OUTDIR)/%.c.o, $(C_SRCS)) \
		  $(patsubst %.s, $(OUTDIR)/%.s.o, $(ASM_SRCS)) \
		  $(patsubst %.rs, $(OUTDIR)/%.rs.ll, $(RUST_SRCS))
PRG		= $(OUTDIR)/charset.prg

.PHONY: all clean

all: $(PRG)

clean:
	rm -rf _build

$(OUTDIR)/%.c.o: %.c
	mkdir -p $(OUTDIR)
	$(CLANG) -c $^ -o $@

$(OUTDIR)/%.rs.ll: %.rs
	mkdir -p $(OUTDIR)
	rustc --crate-type=rlib \
	  -C debuginfo=0 \
	  -C opt-level=1 \
	  $^ \
	  --emit=llvm-ir -o $@

$(PRG): $(OBJS)
	mkdir -p $(OUTDIR)
	$(CLANG) $^ -o $@
