package ds

import "errors"

type Stack[T any] struct {
	data []T
}

func NewStack[T any]() Stack[T] {
	return Stack[T]{}
}

func (s *Stack[T]) Push(v T) {
	s.data = append(s.data, v)
}

func (s *Stack[T]) Pop() (T, error) {
	n := len(s.data)
	if n == 0 {
		var zero T
		return zero, errors.New("Stack is empty")
	}

	elem := s.data[n-1]
	newData := s.data[:n-1]
	s.data = newData

	return elem, nil
}

func (s *Stack[T]) Peek() T {
	n := len(s.data)
	if n == 0 {
		var zero T
		return zero
	}

	return s.data[n-1]
}
