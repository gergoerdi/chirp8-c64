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
IR_SRCS		= $(wildcard src/*.ll)

OUTDIR		= _build
RUST_SRCDIR	= rs
RUST_BUILDDIR	= $(RUST_SRCDIR)/target/release/deps
RUSTFLAGS	= -C debuginfo=0 -C opt-level=1

OBJS		= \
		$(patsubst src/%.c, $(OUTDIR)/%.c.o, $(C_SRCS)) \
		$(patsubst src/%.s, $(OUTDIR)/%.s.o, $(ASM_SRCS)) \
		$(patsubst src/%.ll, $(OUTDIR)/%.ll.o, $(IR_SRCS))

PRG		= $(OUTDIR)/chip8.prg

.PHONY: all clean cargo

all: $(PRG)

clean:
	rm -rf _build
	cd $(RUST_SRCDIR) && cargo clean

$(OUTDIR)/%.c.o: src/%.c
	mkdir -p $(OUTDIR)
	$(CLANG) -c -o $@ $^

$(OUTDIR)/%.ll.o: src/%.ll
	mkdir -p $(OUTDIR)
	$(CLANG) -c -o $@ $^

$(RUST_IR_OBJS): cargo

cargo:
	cd $(RUST_SRCDIR) && \
		RUSTFLAGS="$(RUSTFLAGS) --emit=llvm-ir" \
		cargo rustc --release

$(PRG): cargo $(OBJS)
	mkdir -p $(OUTDIR)
	$(CLANG) -o $@ $(OBJS) $(wildcard $(RUST_BUILDDIR)/*.ll)
