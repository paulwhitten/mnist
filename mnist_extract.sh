#!/bin/bash

# create outpout folder
OUTPUT="output"
if [ ! -d ${OUTPUT} ]; then
    mkdir -p ${OUTPUT};
fi

# extract gzipped data to the output folder
for file in ./data/gz/*.gz; do
    BASENAME=$(basename "${file}" .gz)
    gunzip -c -k "${file}" > "./${OUTPUT}/${BASENAME}"
done
