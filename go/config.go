package main

import (
	"math"
	"math/rand"
	src "raytrace/source"
	"time"
)

// NX is the width of the image
const NX = 400

// NY is the height of the image
const NY = 200

// NS is the number of samples per pixel
const NS = 100

// DEP is the depth of our recursive path finder
const DEP = 10

// MAXU8 is the maximum number a byte can hold
const MAXU8 = 256

// DEG is the width of the viewport in terms of degrees
const DEG = 30.

// RATIO is the height width ratio
const RATIO = NX / NY

// APERTURE is the radius of the lens
const APERTURE = 0.

// TREE is whether or not to use tree structure
const TREE = true

func randomMaterial(materialCode, blur, refractive float64, albedo src.Vector) src.Material {
	mat := uint(materialCode * 3)
	blur /= 2
	refractive++
	albedo.IAddS(1.)
	albedo.IDivS(2.)
	switch mat {
	case 0:
		return src.NewMatte(albedo)
	case 1:
		return src.NewMetal(albedo, blur)
	case 2:
		return src.NewGlass(albedo, blur, refractive)
	default:
		panic("unreachable")
	}
}

// Scenes are set up here
func Scenes() src.Scene {
	eye := src.NewVector(13, 2, 3)
	lookat := src.NewVector(0, 0, 0)
	viewup := src.NewVector(0, 1, 0)

	vision := lookat.Sub(eye)

	rad := math.Pi * DEG / 360.
	height := math.Tan(rad) * vision.Length()
	width := height * RATIO

	unit := vision.Unit()
	proj := unit.MulS(viewup.Dot(unit))
	viewup = (viewup.Sub(proj)).Unit()
	horizon := vision.Cross(viewup).Unit()

	viewup.IMulS(height)
	horizon.IMulS(width)

	list := src.NewList()

	gen := rand.New(rand.NewSource(time.Now().Unix()))
	for i := -11; i < 11; i++ {
		for j := -11; j < 11; j++ {
			iF := float64(i)
			jF := float64(j)
			center := src.NewVector(
				iF+.9*gen.Float64(), .2, jF+.9*gen.Float64(),
			)

			list.Register(
				src.NewSphere(
					center, .2, randomMaterial(
						gen.Float64(), gen.Float64(), gen.Float64(), src.VectorRandom(gen))))
		}
	}

	list.Register(src.NewSphere(
		src.NewVector(0., -1000., 0.), 1000, src.NewMatte(src.VectorUniform(.9)),
	))

	list.Register(src.NewSphere(
		src.VectorJ(), 1., src.NewGlass(
			src.VectorUniform(1.), 0., 1.5,
		)),
	)

	list.Register(src.NewSphere(
		src.NewVector(-4., 1., 0.), 1., src.NewMatte(src.NewVector(.4, .2, .1)),
	))

	list.Register(src.NewSphere(
		src.NewVector(4., 1., 0.), 1.,
		src.NewMetal(src.NewVector(.7, .6, .5), 0.),
	))

	scene := src.NewScene(
		eye,
		lookat.Sub(viewup).Sub(horizon),
		horizon.MulS(2.),
		viewup.MulS(2.),
		APERTURE,
	)

	var hittable src.Hittable

	if TREE {
		hittable = src.NewTree(list)
	} else {
		hittable = list
	}

	scene.Save(hittable)

	return scene
}
