import argparse
import sys
import struct
import numpy as np
from PIL import Image as im

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Extracts an MNIST image")
    parser.add_argument('-f','--file', help='The MNIST image file name', required=True)
    parser.add_argument('-n','--number', type=int, help='The zero based index of the element to extract', required=True)
    args = parser.parse_args()

    try:
        with open(args.file, "rb") as f:
            image_data = f.read()
            data_position = 16
            (image_magic, N, rows, columns) = struct.unpack(">iiii", image_data[:data_position])
            if args.number >= N:
                print("Image index", args.number, "too high.  There are only", N , "images in the input.")
                sys.exit()
            pixels_in_image = rows * columns
            start_image_pos = args.number * pixels_in_image + data_position
            pixels = struct.unpack("B" * pixels_in_image, image_data[start_image_pos: start_image_pos + pixels_in_image])
            img_array = np.zeros((rows, columns), dtype=np.uint8)
            index = 0
            for x in range(rows):
                for y in range(columns):
                    img_array[x,y] = pixels[index]
                    index += 1
            img = im.fromarray(img_array)
            img.save(str(args.number) + ".png")
    except OSError:
        print("Could not open file:", args.file)
        sys.exit()
