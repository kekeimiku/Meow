package main

import (
	"errors"
	"math"
)

const (
	U16Max = math.MaxUint16
	U32Max = math.MaxUint32
)

type VecMinValue struct {
	Orig        *Vec
	SmallOffset *struct {
		base    uint
		offsets []uint16
	}
	BigOffset *struct {
		base    uint
		offsets []uint32
	}
	Small *struct {
		vec []uint16
	}
	Big *struct {
		vec []uint32
	}
}

type Vec struct {
	vec []uint
}

func (*Vec) New(v []uint) *VecMinValue {
	return &VecMinValue{
		Orig: &Vec{v},
		SmallOffset: &struct {
			base    uint
			offsets []uint16
		}{},
		BigOffset: &struct {
			base    uint
			offsets []uint32
		}{},
		Small: &struct{ vec []uint16 }{},
		Big:   &struct{ vec []uint32 }{},
	}
}

func (v *VecMinValue) Pack() *VecMinValue {
	len := len(v.Orig.vec)
	if len < 3 {
		return v
	}

	low := v.Orig.vec[0]
	high := v.Orig.vec[len-1]
	size := high - low

	if size <= U16Max && high >= U16Max {
		return &VecMinValue{
			Orig: &Vec{},
			SmallOffset: &struct {
				base    uint
				offsets []uint16
			}{
				size,
				Map(v.Orig.vec, func(x uint, _ int) uint16 {
					return uint16(x - size)
				}),
			},
			BigOffset: &struct {
				base    uint
				offsets []uint32
			}{},
			Small: &struct{ vec []uint16 }{},
			Big:   &struct{ vec []uint32 }{},
		}
	}

	if high <= U16Max {
		return &VecMinValue{
			Orig: &Vec{},
			SmallOffset: &struct {
				base    uint
				offsets []uint16
			}{},
			BigOffset: &struct {
				base    uint
				offsets []uint32
			}{},
			Small: &struct{ vec []uint16 }{
				Map(v.Orig.vec, func(x uint, _ int) uint16 {
					return uint16(x)
				}),
			},
			Big: &struct{ vec []uint32 }{},
		}
	}

	if size <= U32Max && high >= U32Max {
		return &VecMinValue{
			Orig: &Vec{},
			SmallOffset: &struct {
				base    uint
				offsets []uint16
			}{},
			BigOffset: &struct {
				base    uint
				offsets []uint32
			}{
				size,
				Map(v.Orig.vec, func(x uint, _ int) uint32 {
					return uint32(x - size)
				}),
			},
			Small: &struct{ vec []uint16 }{},
			Big:   &struct{ vec []uint32 }{},
		}
	}

	if high <= U32Max {
		return &VecMinValue{
			Orig: &Vec{},
			SmallOffset: &struct {
				base    uint
				offsets []uint16
			}{},
			BigOffset: &struct {
				base    uint
				offsets []uint32
			}{},
			Small: &struct{ vec []uint16 }{},
			Big: &struct{ vec []uint32 }{
				Map(v.Orig.vec, func(x uint, _ int) uint32 {
					return uint32(x)
				}),
			},
		}
	}

	return v
}

func (v *VecMinValue) Len() int {
	orig := len(v.Orig.vec)
	if orig != 0 {
		return orig
	}

	small_offsets := len(v.SmallOffset.offsets)
	if small_offsets != 0 {
		return small_offsets
	}

	big_offsets := len(v.BigOffset.offsets)
	if big_offsets != 0 {
		return big_offsets
	}

	small := len(v.Small.vec)
	if small != 0 {
		return small
	}

	big := len(v.Big.vec)
	if big != 0 {
		return big
	}

	return 0
}

func (v *VecMinValue) Remove(index int) {
	if len(v.Orig.vec) != 0 {
		v.Orig.vec = append(v.Orig.vec[:index], v.Orig.vec[index+1:]...)
	}

	if len(v.SmallOffset.offsets) != 0 {
		v.SmallOffset.offsets = append(v.SmallOffset.offsets[:index], v.SmallOffset.offsets[index+1:]...)
	}

	if len(v.BigOffset.offsets) != 0 {
		v.BigOffset.offsets = append(v.BigOffset.offsets[:index], v.BigOffset.offsets[index+1:]...)
	}

	if len(v.Small.vec) != 0 {
		v.Small.vec = append(v.Small.vec[:index], v.Small.vec[index+1:]...)
	}

	if len(v.Big.vec) != 0 {
		v.Big.vec = append(v.Big.vec[:index], v.Big.vec[index+1:]...)
	}

}

func (v *VecMinValue) SwapRemove(index int) {
	if len(v.Orig.vec) != 0 {

	}

	if len(v.SmallOffset.offsets) != 0 {

	}

	if len(v.BigOffset.offsets) != 0 {

	}

	if len(v.Small.vec) != 0 {

	}

	if len(v.Big.vec) != 0 {

	}

}

func (v *VecMinValue) Get(index int) {

}

type Iterator struct {
	max       int
	currValue VecMinValue
	err       error
}

func (v *VecMinValue) Iter() *Iterator {
	var max = v.Len()
	var err error
	if max < 0 {
		err = errors.New("'max' should be >= 0")
	}

	return &Iterator{
		max:       max,
		currValue: *v,
		err:       err,
	}
}

// func (v *VecMinValue) Next() *Iterator {
// 	var max = v.Len()
// 	if v.Iter().err != nil {
// 		fmt.Println("adadada")
// 	}

// 	return &Iterator{
// 		max: max,
// 		currValue: *v,
// 	}
// }
