# Image-Processing-CLI

## Introduction

This is a very simple image editing tool.

It offers some image effects that you can apply to your images.

There are some commands that can generate image by their own.

## How to use

By typing `-h` or `--help` you can see what commands are available.

The core cammands aquire three arguments : **effect, input file and output file**.

> You can see those three commands in `-h` menu.

## Effects

1. Blur :

`-e blur -i input.png -o output.png`

2. Brighten :

`-e brighten -i input.png -o output.png`

3. Crop :

`-e crop -i input.png -o output.png`

4. Rotate :

`-e rotate -i input.png -o output.png`

5. Invert :

`-e invert -i input.png -o output.png`

6. Gray Scale :

`-e grayscale -i input.png -o output.png`

## Image Generator

1. Generate :

`-e generate -o output.png`

2. Fractal :

`-e fractal -o output.png`

## ToDo

- [ ] Stackable commands

- [ ] Effect value should be an argument