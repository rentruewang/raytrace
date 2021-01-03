package source

import (
	"errors"
	"math"
)

// ErrNotHit indicates that the ray does not intersect with the object
var ErrNotHit = errors.New("ray does not intersect with anything")

// HitData stores data in one hit
type HitData struct {
	T   float64
	Err error

	Point, Normal Vector
	Matter        Material
}

// HitDataNew returns a new HitData
func HitDataNew(T float64, Err error, Point, Normal Vector, Matter Material) HitData {
	return HitData{T, Err, Point, Normal, Matter}
}

// Hittable represents something you can hit
type Hittable interface {
	Hit(source, towards Vector) (data HitData)
}

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

// Hit implements the interface for Hittable
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
		return HitData{T: neg, Point: point, Normal: sph.Normal(point), Matter: sph.matter}
	case pos > 0:
		point := source.Add(towards.MulS(pos))
		return HitData{T: pos, Point: point, Normal: sph.Normal(point), Matter: sph.matter}
	default:
		return HitData{Err: ErrNotHit}
	}
}
