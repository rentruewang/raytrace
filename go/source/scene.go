package source

import (
	"math/rand"
)

// Scene represents the view
type Scene struct {
	source, corner, horizon, vertical Vector
	list                              Hittable
	aperture                          float64
}

// NewScene returns a new View
func NewScene(source, corner, horizon, vertical Vector, aperture float64) Scene {
	return Scene{source, corner, horizon, vertical, nil, aperture}
}

// Source returns the source of the scene
func (scn Scene) Source() Vector { return scn.source }

// Corner returns the corner of the scene
func (scn Scene) Corner() Vector { return scn.corner }

// Horizon returns the horizon of the scene
func (scn Scene) Horizon() Vector { return scn.horizon }

// Vertical returns the vertical of the scene
func (scn Scene) Vertical() Vector { return scn.vertical }

// Aperture returns the aperture of the scene
func (scn Scene) Aperture() float64 { return scn.aperture }

// SourceTo sets the source of the scene
func (scn *Scene) SourceTo(source Vector) { scn.source = source }

// CornerTo sets the corner of the scene
func (scn *Scene) CornerTo(corner Vector) { scn.corner = corner }

// HorizonTo sets the horizon of the scene
func (scn *Scene) HorizonTo(horizon Vector) { scn.horizon = horizon }

// VerticalTo sets the vertical of the scene
func (scn *Scene) VerticalTo(vertical Vector) { scn.vertical = vertical }

// ApertureTo returns the aperture of the scene
func (scn *Scene) ApertureTo(aperture float64) { scn.aperture = aperture }

// Save switches the Scene's backend
func (scn *Scene) Save(h Hittable) {
	scn.list = h
}

// ColorTrace tracks the color of a path
func (scn Scene) ColorTrace(starting, towards Vector, depth int, gen *rand.Rand) Vector {
	color := VectorUniform(1.)
	for d := 0; d < depth; d++ {
		if data := scn.Hit(starting, towards); data.HasHit() {
			matter := data.Matter()
			reflected := matter.Scatter(towards, data.Normal(), gen)
			color.IMul(matter.Albedo())
			starting, towards = data.Point(), reflected
		} else {
			t := .5 * (towards.Unit().Y() + 1.)
			background := VectorUniform(1.).MulS(1. - t).Add(NewVector(.5, .7, 1.).MulS(t))
			return color.Mul(background)
		}
	}
	return VectorO()
}

// Color determines the color at a given position
func (scn Scene) Color(x, y, ns, depth int, dx, dy float64, gen *rand.Rand) (r, g, b uint8) {
	i, j := RandomDisk(scn.aperture, gen)
	h, v := scn.horizon.Unit().MulS(i), scn.vertical.Unit().MulS(j)
	start := scn.source.Add(h).Add(v)

	var color Vector
	for s := 0; s < ns; s++ {
		i, j := (float64(x)+gen.Float64())/dx, (float64(y)+gen.Float64())/dy
		end := scn.corner.Add(scn.horizon.MulS(i)).Add(scn.vertical.MulS(j))
		towards := end.Sub(start)

		color.IAdd(scn.ColorTrace(start, towards, depth, gen))
	}

	pixel := color.DivS(float64(ns)).MulS(255.999)
	r = uint8(pixel.X())
	g = uint8(pixel.Y())
	b = uint8(pixel.Z())
	return
}

// Hit implements Hittable for Scene
func (scn Scene) Hit(source, towards Vector) HitData {
	return scn.list.Hit(source, towards)
}

// Bounds implements Hittable for Scene
func (scn Scene) Bounds() Box {
	return scn.list.Bounds()
}

// RandomDisk creates a random pair of tuple that lies in a unit disk
func RandomDisk(radius float64, gen *rand.Rand) (x, y float64) {
	for {
		x, y = gen.Float64(), gen.Float64()
		if x*x+y*y <= 1 {
			return x * radius, y * radius
		}
	}
}
