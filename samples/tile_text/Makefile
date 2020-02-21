include ../Makefile.vars
NAME = tile_text
RESOURCES = pixel-font.png tilemap-banner-1bpp.png

project.av: $(RESOURCES)
	rm -rf project.gx16 ./output/
	$(ALOEVERA) create project project.av
	$(ALOEVERA) -p project.av palette import palette_1 pixel-font.png
	$(ALOEVERA) -p project.av imageset import text_set_1 8 8 pixel-font.png
	$(ALOEVERA) -p project.av imageset format text_set_1 palette_1 1
	$(ALOEVERA) -p project.av tilemap init tilemap_1 text_set_1 128 64 text_16
	$(ALOEVERA) -p project.av palette import palette_tilemap tilemap-banner-1bpp.png
	$(ALOEVERA) -p project.av tilemap load -p palette_tilemap -x 1 -y 10 -c 6 tilemap_1 tilemap-banner-1bpp.png
	$(ALOEVERA) -p project.av asm ./output/
	$(ALOEVERA) -p project.av asm -f basic ./output/

generate_resources: project.av

generate_basic: generate_resources
	cat $(NAME).bas ./output/imagesets/text_set_1.basic.inc ./output/tilemaps/tilemap_1.basic.inc > $(NAME).assembled.bas

$(NAME).assembled.bas: generate_basic

include ../common/Makefile
