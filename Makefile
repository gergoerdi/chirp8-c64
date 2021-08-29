C_SRCS		= $(wildcard *.c)
ASM_SRCS	= $(wlidcard *.s)
PRG		= charset.prg

OUTDIR		= _build

OBJS		= $(patsubst %.c, $(OUTDIR)/%.c.o, $(C_SRCS)) \
		  $(patsubst %.s, $(OUTDIR)/%.s.o, $(ASM_SRC))

.PHONY: all clean

all: $(PRG)

clean:
	rm -f $(PRG)

# $(OUTDIR)/%.c.o: %.c
# 	mkdir -p $(OUTDIR)
# 	cl65 -t c64 -c $< -o $@

# $(OUTDIR)/%.s.o: %.s
# 	cl65 -t c64 -c $< -o $@

%.prg: $(C_SRCS) $(ASM_SRCS)
	cl65 -t c64 $^ -o $@
