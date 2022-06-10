package main

import (
	"errors"
	"os"
	"path/filepath"
	"strconv"
)

//FindPidByName return pid based on process name.
func FindPidByName(name string) (int, error) {
	dir, _ := filepath.Glob("/proc/*")
	for _, f := range dir {
		r, _ := os.ReadFile(f + "/comm")
		if string(r) == name+"\n" {
			pid, _ := strconv.Atoi(string(f[6:]))
			return pid, nil
		}
	}
	return -1, errors.New("pid not found")
}

//FindPidByNameAll find the pids of all processes with the same name and return an array.
func FindPidByNameAll(name string) ([]int, error) {
	var p []int
	dir, _ := filepath.Glob("/proc/*")
	for _, f := range dir {
		r, _ := os.ReadFile(f + "/comm")
		if string(r) == name+"\n" {
			pid, _ := strconv.Atoi(string(f[6:]))
			p = append(p, pid)
		}
	}

	if len(p) != 0 {
		return p, nil
	}

	return nil, errors.New("pid not found")

}

func Map[T any, R any](collection []T, iteratee func(T, int) R) []R {
	result := make([]R, len(collection))

	for i, item := range collection {
		result[i] = iteratee(item, i)
	}

	return result
}

func FilterMap[T any, R any](collection []T, callback func(T, int) (R, bool)) []R {
	result := []R{}

	for i, item := range collection {
		if r, ok := callback(item, i); ok {
			result = append(result, r)
		}
	}

	return result
}
