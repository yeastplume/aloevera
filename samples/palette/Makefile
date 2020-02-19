NAME = poke_palette
ALOEVERA = "../../target/debug/aloevera"
RESOURCES = nes-palette.png

project.av: $(RESOURCES)
	$(ALOEVERA) create project.av
	$(ALOEVERA) -p project.av palette import palette_1 nes-palette.png
	$(ALOEVERA) -p project.av asm ./output/
	$(ALOEVERA) -p project.av asm -f basic ./output/

generate_resources: project.av

generate_basic: generate_resources
	cat $(NAME).bas ./output/palettes/palette_1.basic.inc > $(NAME).assembled.bas

$(NAME).assembled.bas: generate_basic

include ../common/Makefile
