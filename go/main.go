package main

import (
	"image"
	"image/color"
	"image/png"
	"log"
	"math/rand"
	"os"
	"path"
	"runtime"
	"time"
)

func main() {
	img := image.NewRGBA(image.Rect(0, 0, NX, NY))

	scene := Scenes()

	const TOTAL = NX * NY

	type RGB struct {
		idx     int
		R, G, B uint8
	}

	worker := func(i int, rgb chan<- RGB, genChan chan *rand.Rand) {
		// implicit throttle
		gen := <-genChan
		x, y := i/NY, i%NY
		r, g, b := scene.Color(x, y, NS, DEP, NX, NY, gen)
		// send the generator back
		genChan <- gen
		rgb <- RGB{i, r, g, b}
	}

	numCPU := runtime.NumCPU()
	rgbChan := make(chan RGB, numCPU)
	genChan := make(chan *rand.Rand, numCPU)

	rgbArr := make([][]RGB, NX)
	for i := 0; i < NX; i++ {
		rgbArr[i] = make([]RGB, NY)
	}

	for i := 0; i < numCPU; i++ {
		source := rand.NewSource(time.Now().Unix())
		gen := rand.New(source)
		genChan <- gen
	}

	for i := 0; i < TOTAL; i++ {
		go worker(i, rgbChan, genChan)
	}

	for i := 0; i < NX*NY; i++ {
		rgb := <-rgbChan
		x, y := rgb.idx/NY, rgb.idx%NY
		rgbArr[x][y] = rgb
	}

	for i := 0; i < NX; i++ {
		for j := 0; j < NY; j++ {
			rgb := rgbArr[i][j]
			img.Set(i, NY-1-j, color.RGBA{rgb.R, rgb.G, rgb.B, MAXU8 - 1})
		}
	}

	folder := "images"
	fname := "image.png"

	fullPath := path.Join(folder, fname)
	os.Mkdir(folder, 0644)

	var file *os.File
	var err error
	if file, err = os.Create(fullPath); err != nil {
		log.Fatalln(err)
	}
	defer file.Close()

	if err = png.Encode(file, img); err != nil {
		log.Fatalln(err)
	}
}
