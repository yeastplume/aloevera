#!/bin/bash
./dir2fat32.sh -f -S 512 fat32_template.img 33 contents
gzip fat32_template.img
