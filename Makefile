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

OUTDIR		= _build
RUSTDIR		= rs
RUSTFLAGS	= -C debuginfo=0 -C opt-level=1

OBJS		= \
		$(patsubst src/%.c, $(OUTDIR)/%.c.o, $(C_SRCS)) \
		$(patsubst src/%.s, $(OUTDIR)/%.s.o, $(ASM_SRCS))
RUST_LL		= $(RUSTDIR)/target/release/deps/chip8_c64-e21ff59526dd729a.ll

PRG		= $(OUTDIR)/charset.prg

.PHONY: all clean cargo

all: $(PRG)

clean:
	rm -rf _build
	cd $(RUSTDIR) && cargo clean

$(OUTDIR)/%.c.o: src/%.c
	mkdir -p $(OUTDIR)
	$(CLANG) -c -o $@ $^

$(RUST_LL): cargo

cargo:
	cd $(RUSTDIR) && cargo rustc --release -- $(RUSTFLAGS) --emit=llvm-ir

$(PRG): $(OBJS) $(RUST_LL)
	mkdir -p $(OUTDIR)
	$(CLANG) -o $@ $^
