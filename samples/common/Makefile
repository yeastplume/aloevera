# Common makefile include for examples
# Don't run make from this directory, run
# from subdirs

X16_RUN = $(X16_EMU) -rom $(X16_ROM) $(X16_EMU_FLAGS) $(DEBUG) $(SD_CARD)

# Ensure ld65 and ca65 are on your path
# Assembly flags
CPU = 6502
AS = ca65
# Add defines, if needed (-DWHATEVER)
ASFLAGS = -g --cpu $(CPU)

# Linker flags
LD = ld65
#Define segments and files in config.cfg
LDFLAGS = -C ../common/x16_config.cfg

OBJS_$(NAME): $(NAME).o
	$(LD) -o $(NAME).prg $(LDFLAGS) -m $(NAME).map $(NAME).o 

$(NAME).o: project.av $(NAME).s
	$(AS) $(ASFLAGS) $(NAME).s

run_fresh:
	$(X16_RUN)

run_asm: OBJS_$(NAME)
	$(X16_RUN) -prg $(NAME).prg 

run_bas: $(NAME).assembled.bas
	$(X16_RUN) -bas $(NAME).assembled.bas

all: OBJS_$(NAME)
clean:
	rm -rf ./output *.o *.prg *.map *.av *.log *.assembled.bas *.img
