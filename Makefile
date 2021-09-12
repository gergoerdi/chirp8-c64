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
		$(patsubst src/%.c, $(OUTDIR)/%.c.ll, $(C_SRCS)) \
		$(patsubst src/%.s, $(OUTDIR)/%.s.o, $(ASM_SRCS)) \
		$(patsubst src/%.ll, $(OUTDIR)/%.ll.bc, $(IR_SRCS)) \

PRG		= $(OUTDIR)/chirp8.prg
ROMS		= $(wildcard roms/*.ch8)

.PHONY: all clean cargo

all: $(OUTDIR)/chirp8.d64

clean:
	rm -rf _build
	cd $(RUST_SRCDIR) && cargo clean

$(OUTDIR)/%.c.ll: src/%.c
	mkdir -p $(OUTDIR)
	$(CLANG) -S -o $@ $^

$(OUTDIR)/%.c.s: $(OUTDIR)/%.c.ll
	mkdir -p $(OUTDIR)
	$(LLVM_MOS)/bin/llc -o $@ $^

$(OUTDIR)/%.c.o: src/%.c
	mkdir -p $(OUTDIR)
	$(CLANG) -c -o $@ $^

$(OUTDIR)/%.c.bc: src/%.c
	mkdir -p $(OUTDIR)
	$(CLANG) -c -o $@ $^

$(OUTDIR)/%.ll.bc: src/%.ll
	mkdir -p $(OUTDIR)
	$(CLANG) -c -o $@ $^

$(RUST_IR_OBJS): cargo

cargo:
	cd $(RUST_SRCDIR) && \
		RUSTFLAGS="$(RUSTFLAGS) --emit=llvm-bc" \
		cargo rustc --release

$(PRG): cargo $(OBJS)
	mkdir -p $(OUTDIR)
	$(CLANG) -o $@ $(OBJS) $(wildcard $(RUST_BUILDDIR)/*.bc)

$(OUTDIR)/chirp8.s: cargo $(OBJS)
	mkdir -p $(OUTDIR)
	$(CLANG) -Wl,--lto-emit-asm -o $@ $(OBJS) $(wildcard $(RUST_BUILDDIR)/*.bc)

$(OUTDIR)/chirp8.d64: $(PRG) $(ROMS)
	c1541 -format "chirp-8",8 d64 $@ \
	  -write $(PRG) $(basename $(notdir $(PRG))) \
	  $(foreach file,$(ROMS), -write $(file) $(shell echo $(notdir $(file)) | tr [A-Z] [a-z]))
