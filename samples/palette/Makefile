include ../Makefile.vars

NAME = poke_palette
RESOURCES = nes-palette.png

project.av: $(RESOURCES)
	$(ALOEVERA) create project project.av
	$(ALOEVERA) -p project.av palette import palette_1 crashy.png
	$(ALOEVERA) -p project.av imageset import chopper 8 8 tileset.png
	$(ALOEVERA) -p project.av imageset format chopper palette_1 4

generate_resources: project.av

generate_basic: generate_resources
	cat $(NAME).bas ./output/palettes/palette_1.basic.inc > $(NAME).assembled.bas

$(NAME).assembled.bas: generate_basic

include ../common/Makefile
