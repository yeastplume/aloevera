include ../Makefile.vars

NAME = sprites
RESOURCES = terra.png

project.av: $(RESOURCES)
	rm -rf project.av ./output/
	$(ALOEVERA) create project project.av
	$(ALOEVERA) -p project.av palette import terra_pal terra.png
	$(ALOEVERA) -p project.av imageset import terra_imageset 16 32 terra.png
	$(ALOEVERA) -p project.av imageset format terra_imageset terra_pal 4
	$(ALOEVERA) -p project.av sprite init terra_sprite terra_imageset
	$(ALOEVERA) -p project.av asm ./output/
	$(ALOEVERA) -p project.av asm -f basic ./output/
	$(ALOEVERA) -p project.av asm -f bin ./output/

generate_resources: project.av

generate_basic: generate_resources
	cat $(NAME).bas ./output/palettes/terra_pal.basic.inc ./output/sprites/terra_sprite.basic.inc > $(NAME).assembled.bas

$(NAME).assembled.bas: generate_basic

include ../common/Makefile
