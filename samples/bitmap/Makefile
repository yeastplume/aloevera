NAME = bitmap
ALOEVERA = "../../target/debug/aloevera"
RESOURCES = kq5.png

project.av: $(RESOURCES)
	rm -rf project.av ./output/
	$(ALOEVERA) create project.av
	$(ALOEVERA) -p project.av palette import kq5_pal kq5-halved.png
	$(ALOEVERA) -p project.av imageset import kq5_screen 320 100 kq5-halved.png
	$(ALOEVERA) -p project.av imageset format kq5_screen kq5_pal 8
	$(ALOEVERA) -p project.av bitmap init kq5_bmp kq5_screen
	$(ALOEVERA) -p project.av asm ./output/
	$(ALOEVERA) -p project.av asm -f basic ./output/

generate_resources: project.av

generate_basic: generate_resources
	cat $(NAME).bas ./output/palettes/tile_wall_pal.basic.inc ./output/imagesets/wall_tiles.basic.inc ./output/tilemaps/wall_tilemap.basic.inc > $(NAME).assembled.bas

$(NAME).assembled.bas: generate_basic

include ../common/Makefile
