include ../Makefile.vars

NAME = sd_card
RESOURCES = kq5.png

project.av: $(RESOURCES)
	rm -rf project.av ./output/
	$(ALOEVERA) create project project.av
	$(ALOEVERA) -p project.av palette import kq5_pal kq5.png
	$(ALOEVERA) -p project.av imageset import kq5_screen 320 200 kq5.png
	$(ALOEVERA) -p project.av imageset format kq5_screen kq5_pal 8
	$(ALOEVERA) -p project.av bitmap init kq5_bmp kq5_screen
	$(ALOEVERA) create sd_image kq5.img
	$(ALOEVERA) -p project.av asm -s kq5.img -f bin . select -a 0x0000 kq5_bmp KQ5.BIN
	$(ALOEVERA) -p project.av asm -s kq5.img -f bin . select -a 0x0000 kq5_pal KQ5PAL.BIN
	$(ALOEVERA) -p project.av asm -f bin . select -a 0x0000 kq5_bmp KQ5.BIN
	$(ALOEVERA) -p project.av asm -f bin . select -a 0x0000 kq5_pal KQ5PAL.BIN

generate_resources: project.av

#SD_CARD = -sdcard kq5.img

generate_basic: generate_resources
	cat $(NAME).bas > $(NAME).assembled.bas

$(NAME).assembled.bas: generate_basic

include ../common/Makefile
