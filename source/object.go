package source

import (
	"math"
)

// Sphere represents a sphere
type Sphere struct {
	center Vector
	radius float64
	matter Material
}

// NewSphere creates a new sphere
func NewSphere(center Vector, radius float64, matter Material) Sphere {
	return Sphere{center, radius, matter}
}

// Center returns the center of a sphere
func (sph Sphere) Center() Vector { return sph.center }

// Radius returns the radius of a sphere
func (sph Sphere) Radius() float64 { return sph.radius }

// Matter returns the material the sphere is
func (sph Sphere) Matter() Material { return sph.matter }

// CenterTo sets the center of a sphere
func (sph *Sphere) CenterTo(center Vector) { sph.center = center }

// RadiusTo sets the radius of a sphere
func (sph *Sphere) RadiusTo(radius float64) { sph.radius = radius }

// MatterTo sets the material of a sphere
func (sph *Sphere) MatterTo(matter Material) { sph.matter = matter }

// Normal computes the normal of a point
func (sph Sphere) Normal(point Vector) Vector { return point.Sub(sph.center) }

// Hit implements Hittable for Sphere
func (sph Sphere) Hit(source, towards Vector) HitData {
	radius := sph.radius

	oc := sph.Normal(source)
	a := towards.L2()
	b := oc.Dot(towards)
	c := oc.L2() - radius*radius

	base := math.Sqrt(b*b - a*c)
	neg, pos := (-b-base)/a, (-b+base)/a

	switch {
	case neg > 0:
		point := source.Add(towards.MulS(neg))
		return NewHit(neg, point, sph.Normal(point), sph.matter)
	case pos > 0:
		point := source.Add(towards.MulS(pos))
		return NewHit(pos, point, sph.Normal(point), sph.matter)
	default:
		return NewMiss()
	}
}

// Bounds implements Hittable for Sphere
func (sph Sphere) Bounds() Box {
	min := sph.center.SubS(sph.radius)
	max := sph.center.AddS(sph.radius)

	return NewBox(
		TupleFloat{min[0], max[0]},
		TupleFloat{min[1], max[1]},
		TupleFloat{min[2], max[2]},
	)
}
