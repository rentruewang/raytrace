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

	pb "github.com/cheggaaa/pb/v3"
)

// None is nothing
type None struct {
}

func main() {
	img := image.NewRGBA(image.Rect(0, 0, NX, NY))

	scene := Scenes()

	const TOTAL = NX * NY

	progbar := pb.StartNew(TOTAL)

	done := make(chan None)

	type RGB struct {
		idx     int
		R, G, B uint8
	}

	numCPU := runtime.NumCPU()
	rgbChan := make(chan RGB, numCPU)
	genChan := make(chan *rand.Rand, numCPU)
	rgbArr := make([][]RGB, NX)
	for i := 0; i < NX; i++ {
		rgbArr[i] = make([]RGB, NY)
	}

	worker := func(i int, rgb chan<- RGB, doneChan chan<- None) {
		// Using the number of generators as an implicit throttle
		gen := <-genChan

		x, y := i/NY, i%NY
		r, g, b := scene.Color(x, y, NS, DEP, NX, NY, gen)

		// send the generator back
		genChan <- gen
		rgb <- RGB{i, r, g, b}
		doneChan <- None{}
	}

	go func() {
		for i := 0; i < TOTAL; i++ {
			if i%PROGRESS == 0 {
				progbar.SetCurrent(int64(i))
			}
			<-done
		}
	}()

	for i := 0; i < numCPU; i++ {
		source := rand.NewSource(time.Now().Unix())
		gen := rand.New(source)
		genChan <- gen
	}

	for i := 0; i < TOTAL; i++ {
		go worker(i, rgbChan, done)
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
	var err error
	err = os.Mkdir(folder, 0744)
	if err != nil {
		log.Fatalln(err)
	}

	var file *os.File
	if file, err = os.Create(fullPath); err != nil {
		log.Fatalln(err)
	}
	defer file.Close()

	if err = png.Encode(file, img); err != nil {
		log.Fatalln(err)
	}
}
