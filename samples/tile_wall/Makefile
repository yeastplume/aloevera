include ../Makefile.vars
NAME = tile_wall
RESOURCES = tile_wall-map.png tile_wall-imageset-4bpp.png

project.av: $(RESOURCES)
	rm -rf project.av ./output/
	$(ALOEVERA) create project project.av
	$(ALOEVERA) -p project.av palette import tile_wall_pal tile_wall-imageset-4bpp.png
	$(ALOEVERA) -p project.av imageset import wall_tiles 16 16 tile_wall-imageset-4bpp.png
	$(ALOEVERA) -p project.av imageset format wall_tiles tile_wall_pal 4
	$(ALOEVERA) -p project.av tilemap init wall_tilemap wall_tiles 64 32 tile_4bpp
	$(ALOEVERA) -p project.av tilemap load -x 0 -y 10 wall_tilemap tile_wall-map.png
	$(ALOEVERA) -p project.av asm ./output/ all
	$(ALOEVERA) -p project.av asm -f basic ./output/ all
	$(ALOEVERA) -p project.av asm -f bin ./output/ all

generate_resources: project.av

generate_basic: generate_resources
	cat $(NAME).bas ./output/palettes/tile_wall_pal.basic.inc ./output/imagesets/wall_tiles.basic.inc ./output/tilemaps/wall_tilemap.basic.inc > $(NAME).assembled.bas

$(NAME).assembled.bas: generate_basic

include ../common/Makefile
