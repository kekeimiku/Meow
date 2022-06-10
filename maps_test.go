package main

import (
	"bufio"
	"strings"
	"testing"

	"github.com/google/go-cmp/cmp"
)

const test_maps string = `563ea224a000-563ea2259000 r--p 00000000 103:05 5920780 /usr/bin/fish
563ea23ea000-563ea2569000 rw-p 00000000 00:00 0 [heap]
7f9e08000000-7f9e08031000 rw-p 00000000 00:00 0`

func Test_ProcMaps(t *testing.T) {

	var lines []string
	sc := bufio.NewScanner(strings.NewReader(test_maps))
	for sc.Scan() {
		lines = append(lines, sc.Text())
	}

	var maps []*ProcMap
	for _, v := range lines {
		m, err := parseProcMap(v)
		if err != nil {
			t.Error(err)
		}
		maps = append(maps, m)
	}

	eqm := []*ProcMap{
		{
			StartAddr: 94827008270336,
			EndAddr:   94827008331776,
			Perms: &ProcMapPermissions{
				Read:    true,
				Write:   false,
				Execute: false,
				Shared:  false,
				Private: true,
			},
			Pathname: "/usr/bin/fish",
		},
		{
			StartAddr: 94827009974272,
			EndAddr:   94827011543040,
			Perms: &ProcMapPermissions{
				Read:    true,
				Write:   true,
				Execute: false,
				Shared:  false,
				Private: true,
			},
			Pathname: "[heap]",
		},
		{
			StartAddr: 140316715778048,
			EndAddr:   140316715978752,
			Perms: &ProcMapPermissions{
				Read:    true,
				Write:   true,
				Execute: false,
				Shared:  false,
				Private: true,
			},
			Pathname: "",
		},
	}

	if !cmp.Equal(maps, eqm) {
		t.Error("test parse maps error")
	}

}
