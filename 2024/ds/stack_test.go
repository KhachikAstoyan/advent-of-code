package ds_test

import (
	"testing"

	"github.com/KhachikAstoyan/advent-of-code/2024/ds"
)

func TestStack(t *testing.T) {
	t.Run("push", func(t *testing.T) {
		s := ds.NewStack[int]()
		s.Push(10)
		got := s.Peek()
		if got != 10 {
			t.Errorf("Peek() = %d; want 10", got)
		}

		s.Push(20)
		got = s.Peek()
		if got != 20 {
			t.Errorf("Peek() = %d; want 20", got)
		}
	})

	t.Run("pop", func(t *testing.T) {
		s := ds.NewStack[int]()
		s.Push(1)
		s.Push(2)

		popped, _ := s.Pop()
		if popped != 2 {
			t.Errorf("Pop() = %d; want 2", popped)
		}
		last, _ := s.Pop()
		if last != 1 {
			t.Errorf("Pop() = %d; want 1", last)
		}

		_, err := s.Pop()
		if err == nil {
			t.Errorf("error of Pop() = nil; expected error")
		}
	})
}
