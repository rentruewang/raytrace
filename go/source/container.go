package source

import (
	"math"
	"sort"
)

// List is a container type backed by an array
type List struct {
	objects []Hittable
}

// NewList creates a new `List` instance
func NewList() List {
	return List{objects: make([]Hittable, 0)}
}

// AsList creates a new `List` from an exisiting slice
func AsList(list []Hittable) List { return List{list} }

// Register registers an object to a list
func (lst *List) Register(obj Hittable) {
	lst.objects = append(lst.objects, obj)
}

// Hit implements Hittable for List
func (lst List) Hit(source, towards Vector) HitData {
	towards = towards.Unit()
	closest := NewMiss()
	for _, obj := range lst.objects {
		data := obj.Hit(source, towards)
		if data.T() < closest.T() {
			closest = data
		}
	}
	return closest
}

// Bounds implements Hittable for List
func (lst List) Bounds() Box {
	objects := lst.objects
	switch len(objects) {
	case 0:
		panic("unreachable")
	case 1:
		return objects[0].Bounds()
	default:
		bnd := objects[0].Bounds()
		for i := 1; i < len(objects); i++ {
			bnd = Wraps(bnd, objects[i].Bounds())
		}
		return bnd
	}
}

// TupleFloat holds a pair of `int`s
type TupleFloat [2]float64

// NewTupleFloat returns a new `TupleFloat`
func NewTupleFloat(a, b float64) TupleFloat {
	return TupleFloat{a, b}
}

// Box is a box that bounds
type Box [3]TupleFloat

func ordered(x TupleFloat) TupleFloat {
	if x[0] > x[1] {
		return TupleFloat{x[1], x[0]}
	}
	return x
}

// NewBox creates a new Box
func NewBox(x, y, z TupleFloat) Box {
	return Box{ordered(x), ordered(y), ordered(z)}
}

func largerBound(a, b TupleFloat) (result TupleFloat) {
	if a[0] < b[0] {
		result[0] = a[0]
	} else {
		result[0] = b[0]
	}

	if a[1] > b[1] {
		result[1] = a[1]
	} else {
		result[1] = b[1]
	}

	return
}

// Wraps wraps two bounding boxes into a single box
func Wraps(a, b Box) Box {
	return NewBox(
		largerBound(a[0], b[0]),
		largerBound(a[1], b[1]),
		largerBound(a[2], b[2]),
	)
}

// Min returns the minimum corner in the bounding box
func (box Box) Min() Vector {
	return Vector{box[0][0], box[1][0], box[2][0]}
}

// Max returns the maximum corner in the bounding box
func (box Box) Max() Vector {
	return Vector{box[0][1], box[1][1], box[2][1]}
}

// Center returns the center of a Box
func (box Box) Center() Vector {
	return Vector{
		(box[0][0] + box[0][1]) / 2.,
		(box[1][0] + box[1][1]) / 2.,
		(box[2][0] + box[2][1]) / 2.,
	}
}

// Through shows if a ray passes through a box
func (box Box) Through(source, towards Vector) bool {
	min, max := box.Min(), box.Max()
	tMin, tMax := math.Inf(-1), math.Inf(1)

	for i := 0; i < 3; i++ {
		invB := 1. / towards[i]
		tSmall := (min[i] - source[i]) * invB
		tLarge := (max[i] - source[i]) * invB

		if invB < 0 {
			tSmall, tLarge = tLarge, tSmall
		}

		if tSmall > tLarge {
			panic("impossible")
		}

		if tSmall > tMin {
			tMin = tSmall
		}

		if tLarge < tMax {
			tMax = tLarge
		}
	}

	return tMin < tMax
}

// Axis is used as an enumeration
type Axis uint

const (
	// AxisX is the X axis
	AxisX Axis = iota
	// AxisY is the Y axis
	AxisY
	// AxisZ is the Z axis
	AxisZ
)

func maxVar(list []Hittable) Axis {
	avg := Vector{}
	for _, vec := range list {
		avg.IAdd(vec.Bounds().Center())
	}
	avg.IDivS(float64(len(list)))

	naiveVariance := Vector{}
	for _, vec := range list {
		naiveVariance.IAdd(vec.Bounds().Center().Sub(avg))
	}

	if naiveVariance.X() > naiveVariance.Y() && naiveVariance.X() > naiveVariance.Z() {
		return AxisX
	} else if naiveVariance.Y() > naiveVariance.Z() {
		return AxisY
	} else {
		return AxisZ
	}
}

// TreeNode represents a node in the tree structure
type TreeNode struct {
	box         Box
	left, right Hittable
}

// NewTreeNode creates a new TreeNode
func NewTreeNode(left, right Hittable) TreeNode {
	box := Wraps(left.Bounds(), right.Bounds())
	return TreeNode{box, left, right}
}

// Hit implements Hittable for TreeNode
func (tn TreeNode) Hit(source, towards Vector) HitData {
	if !tn.box.Through(source, towards) {
		return NewMiss()
	}

	leftHit := tn.left.Hit(source, towards)
	rightHit := tn.right.Hit(source, towards)

	leftHasHit := leftHit.HasHit()
	rightHasHit := rightHit.HasHit()

	switch {
	case leftHasHit && rightHasHit:
		if leftHit.T() < rightHit.T() {
			return leftHit
		} else {
			return rightHit
		}
	case leftHasHit && !rightHasHit:
		return leftHit
	case !leftHasHit && rightHasHit:
		return rightHit
	case !leftHasHit && !rightHasHit:
		return NewMiss()
	default:
		panic("unreachable")
	}
}

// Bounds implements Hittable for TreeNode
func (tn TreeNode) Bounds() Box {
	return tn.box
}

// Tree holds a tree of TreeNodes
type Tree struct{ root Hittable }

func recursivePartition(list []Hittable) Hittable {
	l := len(list)
	switch l {
	case 0:
		panic("unreachable")
	case 1:
		return list[0]
	case 2:
		return NewTreeNode(list[0], list[1])
	default:
		var compare func(i, j int) bool

		switch maxVar(list) {
		case AxisX:
			compare = func(i, j int) bool {
				return list[i].Bounds().Center().X() < list[j].Bounds().Center().X()
			}
		case AxisY:
			compare = func(i, j int) bool {
				return list[i].Bounds().Center().Y() < list[j].Bounds().Center().Y()
			}
		case AxisZ:
			compare = func(i, j int) bool {
				return list[i].Bounds().Center().Z() < list[j].Bounds().Center().Z()
			}
		default:
			panic("unreachable")
		}

		sort.Slice(list, compare)

		half := l / 2

		leftNode := recursivePartition(list[:half])
		rightNode := recursivePartition(list[half:])

		return NewTreeNode(leftNode, rightNode)
	}
}

// NewTree creates a new tree from a List
func NewTree(list List) Tree {
	objects := list.objects
	return Tree{recursivePartition(objects)}
}

// Hit implments Hittable for Tree
func (t Tree) Hit(source, towards Vector) HitData {
	return t.root.Hit(source, towards)
}

// Bounds implements Hittable for Tree
func (t Tree) Bounds() Box { return t.root.Bounds() }
