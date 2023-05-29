package main

import (
	"fmt"
	"io"
	"os"
	"reflect"
	"strings"

	"github.com/hashicorp/nomad/api"
)

func main() {
	job := reflect.TypeOf(api.Job{})
	w := TypeWalker{t: job}
	cg := CodeGenerator{tracker: make(TypeTracker), output: os.Stdout}
	if err := w.acceptVisitor(&cg); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

type TypeWalker struct {
	t reflect.Type
}

func (w *TypeWalker) acceptVisitor(v Visitor) error {
	switch w.t.Kind() {
	case reflect.Bool:
	case reflect.Int:
	case reflect.Int8:
	case reflect.Int16:
	case reflect.Int32:
	case reflect.Int64:
	case reflect.Uint:
	case reflect.Uint8:
	case reflect.Uint16:
	case reflect.Uint32:
	case reflect.Uint64:
	case reflect.Uintptr:
	case reflect.Float32:
	case reflect.Float64:
	case reflect.Complex64:
	case reflect.Complex128:
	case reflect.Array:
	case reflect.Chan:
	case reflect.Func:
	case reflect.Interface:
	case reflect.Map:
	case reflect.Pointer:
		return v.visitPointer(w.t)
	case reflect.Slice:
		return v.visitSlice(w.t)
	case reflect.String:
	case reflect.Struct:
		return v.visitStruct(w.t)
	case reflect.UnsafePointer:
	}
	// return fmt.Errorf("cannot handle type: %v", w.t)
	return nil
}

type Visitor interface {
	visitPointer(t reflect.Type) error
	visitSlice(t reflect.Type) error
	visitStruct(t reflect.Type) error
	visitStructField(index int, t reflect.StructField) error
}

type TypeTracker map[string]bool

func (t TypeTracker) seen(name string) bool {
	_, found := t[name]
	return found
}

func (t TypeTracker) generated(name string) bool {
	return t[name]
}

type CodeGenerator struct {
	tracker TypeTracker
	output  io.Writer
}

func (g *CodeGenerator) visitStruct(t reflect.Type) error {
	if _, err := fmt.Fprintf(g.output, "%s\n", t.Name()); err != nil {
		return err
	}
	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		if err := g.visitStructField(i, field); err != nil {
			return err
		}
	}
	return nil
}

func (g *CodeGenerator) visitStructField(i int, f reflect.StructField) error {
	if value, ok := f.Tag.Lookup("hcl"); ok {
		values := strings.Split(value, ",")
		if _, err := fmt.Fprintf(g.output, "  %s: %v\n", f.Name, values); err != nil {
			return err
		}
		if err := (&TypeWalker{t: f.Type}).acceptVisitor(g); err != nil {
			return err
		}
	}
	return nil
}

func (g *CodeGenerator) visitPointer(t reflect.Type) error {
	return (&TypeWalker{t: t.Elem()}).acceptVisitor(g)
}

func (g *CodeGenerator) visitSlice(t reflect.Type) error {
	return (&TypeWalker{t: t.Elem()}).acceptVisitor(g)
}
