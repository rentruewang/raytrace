package source

import (
	"math"
	"math/rand"
)

// Material takes an input ray and scatters it
type Material interface {
	Scatter(input, normal Vector, gen *rand.Rand) (output Vector)
	Albedo() (rgb Vector)
}

// Matte is a mathematically ideal lambertian material
type Matte struct{ albedo Vector }

// NewMatte returns a new type of matte
func NewMatte(albedo Vector) Matte { return Matte{albedo} }

// Albedo returns the albedo matte has
func (mat Matte) Albedo() Vector { return mat.albedo }

// AlbedoTo sets the albedo of a matte
func (mat *Matte) AlbedoTo(albedo Vector) { mat.albedo = albedo }

// Scatter implementation for Matte, given a unit length normal vector
func (mat Matte) Scatter(input, normal Vector, gen *rand.Rand) Vector {
	normal = normal.Unit()
	return VectorRandomBall(1, gen).Add(normal)
}

// Metal represents the metal type
type Metal struct {
	albedo Vector
	blur   float64
}

// NewMetal creates a new kind of metal
func NewMetal(albedo Vector, blur float64) Metal { return Metal{albedo, blur} }

// Blur returns the blur a metal could have
func (met Metal) Blur() float64 { return met.blur }

// Albedo returns the albedo metal has
func (met Metal) Albedo() Vector { return met.albedo }

// BlurTo sets the blur a metal could have
func (met *Metal) BlurTo(blur float64) { met.blur = blur }

// AlbedoTo sets the albedo of a metal
func (met *Metal) AlbedoTo(albedo Vector) { met.albedo = albedo }

// Scatter implementation for Metal, given a unit length normal vector
func (met Metal) Scatter(input, normal Vector, gen *rand.Rand) Vector {
	input = input.Unit()
	normal = normal.Unit()
	random := VectorRandomBall(met.blur, gen)
	casted := normal.MulS(input.Dot(normal) * 2)
	return random.Add(input).Sub(casted)
}

// Glass represents dielectirc materials
type Glass struct {
	albedo           Vector
	blur, refractive float64
}

// NewGlass creates glass
func NewGlass(albedo Vector, blur, refractive float64) Glass { return Glass{albedo, blur, refractive} }

// Albedo returns the albedo for glass
func (gls Glass) Albedo() Vector { return gls.albedo }

// Blur returns the blur for glass
func (gls Glass) Blur() float64 { return gls.blur }

// Refractive returns the refractive coefficient for glass
func (gls Glass) Refractive() float64 { return gls.refractive }

// AlbedoTo sets the value of the albedo for glass
func (gls *Glass) AlbedoTo(albedo Vector) { gls.albedo = albedo }

// BlurTo sets the value of the blur for glass
func (gls *Glass) BlurTo(blur float64) { gls.blur = blur }

// RefractiveTo sets the value of the refractive coefficient for glass
func (gls *Glass) RefractiveTo(refractive float64) { gls.refractive = refractive }

// Schlick approximates the probability a ray is reflected
func Schlick(cosine, ratio float64) float64 {
	r := (1. - ratio) / (1. + ratio)
	sq := r * r
	return sq + (1.-sq)*math.Pow(1.-cosine, 5)
}

// Scatter implementation for Glass, given a unit length normal vector
func (gls Glass) Scatter(input, normal Vector, gen *rand.Rand) Vector {
	input = input.Unit()
	normal = normal.Unit()
	cosine := input.Dot(normal)

	var ratio float64
	if cosine < 0 {
		ratio = 1. / gls.refractive
	} else {
		ratio = gls.refractive
	}

	sineSquared := 1. - cosine*cosine
	cosineSquared := 1. - ratio*ratio*sineSquared
	refract := cosine <= 0. || cosineSquared >= 0.

	random := rand.Float64()
	randomBlur := VectorRandomBall(gls.blur, gen)
	if refract && random > Schlick(math.Abs(cosine), gls.refractive) {
		firstTerm := input.Add(normal.MulS(cosine))
		secondTerm := normal.MulS(math.Sqrt(cosineSquared))
		return firstTerm.MulS(ratio).Sub(secondTerm).Add(randomBlur)
	}
	casted := normal.MulS(input.Dot(normal) * 2.)
	return randomBlur.Add(input).Sub(casted)
}
