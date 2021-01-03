package main

import (
	"math"
	src "raytrace/source"
)

// NX is the width of the image
const NX = 800

// NY is the height of the image
const NY = 400

// NS is the number of samples per pixel
const NS = 100

// DEP is the depth of our recursive path finder
const DEP = 50

// MAXU8 is the maximum number a byte can hold
const MAXU8 = 256

// DEG is the width of the viewport in terms of degrees
const DEG = 90.

// RATIO is the height width ratio
const RATIO = NX / NY

// APERTURE is the radius of the lens
const APERTURE = 0.01

// Scenes are set up here
func Scenes() src.Scene {
	eye := src.NewVector(0., 0., -.3)
	lookat := src.NewVector(0, 0, -1)
	viewup := src.NewVector(0, 1, 0)

	vision := lookat.Sub(eye)

	rad := ((DEG / 2.) / 180.) * math.Pi
	height := math.Tan(rad) * vision.Length()
	width := height * RATIO

	unit := vision.Unit()
	proj := unit.MulS(viewup.Dot(unit))
	viewup = (viewup.Sub(proj)).Unit()
	horizon := vision.Cross(viewup).Unit()

	viewup.IMulS(height)
	horizon.IMulS(width)

	scene := src.NewScene(
		eye,
		lookat.Sub(viewup).Sub(horizon),
		horizon.MulS(2),
		viewup.MulS(2),
		APERTURE,
	)

	scene.Register(
		src.NewSphere(
			src.NewVector(0, 0, -1), .5,
			src.NewGlass(
				src.NewVector(1, 1, 1), 0, 1.5,
			)))
	scene.Register(
		src.NewSphere(
			src.NewVector(0, -100.5, -1), 100,
			src.NewMatte(
				src.NewVector(.9, .9, 0),
			)))
	scene.Register(
		src.NewSphere(
			src.NewVector(-1, 0, -1), .5,
			src.NewMatte(
				src.NewVector(.8, .3, .3),
			)))
	scene.Register(
		src.NewSphere(
			src.NewVector(1, 0, -1), .5,
			src.NewMetal(
				src.NewVector(.95, .95, .95), .1,
			)))

	return scene
}
