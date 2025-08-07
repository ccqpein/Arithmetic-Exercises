package main

import (
	"fmt"
	"time"
)

type FooBar struct {
	n    int
	sigF chan struct{}
	sigB chan struct{}
}

func NewFooBar(n int) *FooBar {
	return &FooBar{
		n:    n,
		sigF: make(chan struct{}),
		sigB: make(chan struct{}),
	}
}

func (fb *FooBar) Foo(printFoo func()) {
	for i := 0; i < fb.n; i++ {
		// printFoo() outputs "foo". Do not change or remove this line.
		printFoo()

		fb.sigB <- struct{}{}
		<-fb.sigF
	}

	close(fb.sigB)
}

func (fb *FooBar) Bar(printBar func()) {
	for i := 0; i < fb.n; i++ {
		<-fb.sigB
		// printBar() outputs "bar". Do not change or remove this line.
		printBar()

		fb.sigF <- struct{}{}
	}

	close(fb.sigF)
}

func main() {
	fb := NewFooBar(1)

	go fb.Foo(func() { fmt.Print("Foo") })
	go fb.Bar(func() { fmt.Print("Bar\n") })

	time.Sleep(time.Second)
}
