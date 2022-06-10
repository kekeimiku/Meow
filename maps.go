package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// 对于当前需求我们只需要 startaddr, endaddr, perm, pathname
// ProcMap contains the process memory-mappings of the process,
// read from /proc/[pid]/maps
type ProcMap struct {
	// The start address of current mapping.
	StartAddr uintptr
	// The end address of the current mapping
	EndAddr uintptr
	// The permissions for this mapping
	Perms *ProcMapPermissions
	// The file or psuedofile (or empty==anonymous)
	Pathname string
}

// ProcMapPermissions contains permission settings read from /proc/[pid]/maps
type ProcMapPermissions struct {
	// mapping has the [R]ead flag set
	Read bool
	// mapping has the [W]rite flag set
	Write bool
	// mapping has the [X]ecutable flag set
	Execute bool
	// mapping has the [S]hared flag set
	Shared bool
	// mapping is marked as [P]rivate (copy on write)
	Private bool
}

// parseAddress just converts a hex-string to a uintptr
func parseAddress(s string) (uintptr, error) {
	a, err := strconv.ParseUint(s, 16, 0)
	if err != nil {
		return 0, err
	}

	return uintptr(a), nil
}

// parseAddresses parses the start-end address
func parseAddresses(s string) (uintptr, uintptr, error) {
	toks := strings.Split(s, "-")
	if len(toks) < 2 {
		return 0, 0, fmt.Errorf("invalid address")
	}

	saddr, err := parseAddress(toks[0])
	if err != nil {
		return 0, 0, err
	}

	eaddr, err := parseAddress(toks[1])
	if err != nil {
		return 0, 0, err
	}

	return saddr, eaddr, nil
}

// parsePermissions parses a token and returns any that are set.
func parsePermissions(s string) (*ProcMapPermissions, error) {
	if len(s) < 4 {
		return nil, fmt.Errorf("invalid permissions token")
	}

	perms := ProcMapPermissions{}
	for _, ch := range s {
		switch ch {
		case 'r':
			perms.Read = true
		case 'w':
			perms.Write = true
		case 'x':
			perms.Execute = true
		case 'p':
			perms.Private = true
		case 's':
			perms.Shared = true
		}
	}

	return &perms, nil
}

// parseProcMap will attempt to parse a single line within a proc/[pid]/maps
// buffer.
func parseProcMap(text string) (*ProcMap, error) {
	fields := strings.Fields(text)
	if len(fields) < 5 {
		return nil, fmt.Errorf("truncated procmap entry")
	}

	saddr, eaddr, err := parseAddresses(fields[0])
	if err != nil {
		return nil, err
	}

	perms, err := parsePermissions(fields[1])
	if err != nil {
		return nil, err
	}

	pathname := ""

	if len(fields) >= 5 {
		pathname = strings.Join(fields[5:], " ")
	}

	return &ProcMap{
		StartAddr: saddr,
		EndAddr:   eaddr,
		Perms:     perms,
		Pathname:  pathname,
	}, nil
}

// ProcMaps reads from /proc/[pid]/maps to get the memory-mappings of the
// process.
func ProcMaps(path string) ([]*ProcMap, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	maps := []*ProcMap{}
	scan := bufio.NewScanner(file)

	for scan.Scan() {
		m, err := parseProcMap(scan.Text())
		if err != nil {
			return nil, err
		}

		maps = append(maps, m)
	}

	return maps, nil
}
