package source

import (
	"math"
	"math/rand"
)

// Vector represents an Nd vector
type Vector struct {
	x, y, z float64
}

// NewVector creates a vector from a given dimension
func NewVector(x, y, z float64) Vector { return Vector{x, y, z} }

// X returns vector.x
func (vec Vector) X() float64 { return vec.x }

// Y returns vector.y
func (vec Vector) Y() float64 { return vec.y }

// Z returns vector.z
func (vec Vector) Z() float64 { return vec.z }

// XTo sets value of x
func (vec *Vector) XTo(x float64) { vec.x = x }

// YTo sets value of y
func (vec *Vector) YTo(y float64) { vec.y = y }

// ZTo sets value of z
func (vec *Vector) ZTo(z float64) { vec.z = z }

// Equals compares if two vectors are equal
func (vec Vector) Equals(other Vector) bool { return vec == other }

// Add adds two vectors
func (vec Vector) Add(other Vector) Vector {
	return Vector{vec.x + other.x, vec.y + other.y, vec.z + other.z}
}

// Sub subtracts one vector from the other
func (vec Vector) Sub(other Vector) Vector {
	return Vector{vec.x - other.x, vec.y - other.y, vec.z - other.z}
}

// Mul multiplies two vectors
func (vec Vector) Mul(other Vector) Vector {
	return Vector{vec.x * other.x, vec.y * other.y, vec.z * other.z}
}

// Div divides one vector with another vector
func (vec Vector) Div(other Vector) Vector {
	return Vector{vec.x / other.x, vec.y / other.y, vec.z / other.z}
}

// AddS adds a vector to a scalar
func (vec Vector) AddS(other float64) Vector {
	return Vector{vec.x + other, vec.y + other, vec.z + other}
}

// SubS subtracts a scalar from a vector
func (vec Vector) SubS(other float64) Vector {
	return Vector{vec.x - other, vec.y - other, vec.z - other}
}

// MulS multiplies a vector to a scalar
func (vec Vector) MulS(other float64) Vector {
	return Vector{vec.x * other, vec.y * other, vec.z * other}
}

// DivS divides one vector with another vector
func (vec Vector) DivS(other float64) Vector {
	return Vector{vec.x / other, vec.y / other, vec.z / other}
}

// IAdd adds two vectors inplace
func (vec *Vector) IAdd(other Vector) { vec.x += other.x; vec.y += other.y; vec.z += other.z }

// ISub subtracts one vector from the other inplace
func (vec *Vector) ISub(other Vector) { vec.x -= other.x; vec.y -= other.y; vec.z -= other.z }

// IMul multiplies two vectors inplace
func (vec *Vector) IMul(other Vector) { vec.x *= other.x; vec.y *= other.y; vec.z *= other.z }

// IDiv divides one vector with another vector inplace
func (vec *Vector) IDiv(other Vector) { vec.x /= other.x; vec.y /= other.y; vec.z /= other.z }

// IAddS adds a vector to a scalar inplace
func (vec *Vector) IAddS(other float64) { vec.x += other; vec.y += other; vec.z += other }

// ISubS subtracts a scalar from a vector inplace
func (vec *Vector) ISubS(other float64) { vec.x -= other; vec.y -= other; vec.z -= other }

// IMulS multiplies a vector to a scalar inplace
func (vec *Vector) IMulS(other float64) { vec.x *= other; vec.y *= other; vec.z *= other }

// IDivS divides one vector with another vector inplace
func (vec *Vector) IDivS(other float64) { vec.x /= other; vec.y /= other; vec.z /= other }

// Dot is an inner product
func (vec Vector) Dot(other Vector) float64 { return vec.x*other.x + vec.y*other.y + vec.z*other.z }

// L2 means l2 norm
func (vec Vector) L2() float64 { return vec.Dot(vec) }

// Length is the length of a vector
func (vec Vector) Length() float64 { return math.Sqrt(vec.L2()) }

// Unit returns the unit direction of a vector
func (vec Vector) Unit() Vector { return vec.DivS(vec.Length()) }

// Sqrt computes element-wise Sqrt for vector
func (vec Vector) Sqrt() Vector { return Vector{math.Sqrt(vec.x), math.Sqrt(vec.y), math.Sqrt(vec.z)} }

// IsNaN indicates if a vector has an NaN
func (vec Vector) IsNaN() bool { return math.IsNaN(vec.x) || math.IsNaN(vec.y) || math.IsNaN(vec.z) }

// Cross is a cross product for two 3D vectors
func (vec Vector) Cross(other Vector) Vector {
	return Vector{
		vec.y*other.z - vec.z*other.y,
		vec.z*other.x - vec.x*other.z,
		vec.x*other.y - vec.y*other.x,
	}
}

// VectorRandom creates a random vector inside a sphere that has its radius given
func VectorRandom(radius float64, gen *rand.Rand) Vector {
	for {
		random := Vector{gen.Float64(), gen.Float64(), gen.Float64()}
		if random.L2() <= 1 {
			return random.MulS(radius)
		}
	}
}
