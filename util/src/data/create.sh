#!/bin/bash
./dir2fat32.sh -f fat32_template.img 32 contents
gzip fat32_template.img
