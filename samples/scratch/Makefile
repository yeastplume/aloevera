include ../Makefile.vars
NAME = scratch

project.av:
	$(ALOEVERA) create project project.av
#	$(ALOEVERA) -p project.gx16 palette import palette_1 pixel-font.png
#	$(ALOEVERA) -p project.gx16 imageset import text_set_1 8 8 pixel-font.png
#	$(ALOEVERA) -p project.gx16 imageset format text_set_1 palette_1 1
#	$(ALOEVERA) -p project.gx16 asm ./output/
# $(ALOEVERA) -p project.gx16 asm -f basic ./output/

generate_resources: project.av

generate_basic: generate_resources
	cat $(NAME).bas > $(NAME).assembled.bas

$(NAME).assembled.bas: generate_basic

include ../common/Makefile
