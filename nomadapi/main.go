package main

import (
	"errors"
	"fmt"
	"io"
	"os"
	"reflect"
	"strings"

	// v1.5.6: go get github.com/hashicorp/nomad/api@8af70885c02ab921dedbdf6bc406a1e886866f80
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
		return nil // TODO
	case reflect.Int:
		return nil // TODO
	case reflect.Int8:
		return nil // TODO
	case reflect.Int16:
		return nil // TODO
	case reflect.Int32:
		return nil // TODO
	case reflect.Int64:
		return nil // TODO
	case reflect.Uint:
		return nil // TODO
	case reflect.Uint8:
		return nil // TODO
	case reflect.Uint16:
		return nil // TODO
	case reflect.Uint32:
		return nil // TODO
	case reflect.Uint64:
		return nil // TODO
	case reflect.Float32:
		return nil // TODO
	case reflect.Float64:
		return nil // TODO
	case reflect.Array:
		return v.visitArray(w.t)
	case reflect.Map:
		return v.visitMap(w.t)
	case reflect.Pointer:
		return v.visitPointer(w.t)
	case reflect.Slice:
		return v.visitSlice(w.t)
	case reflect.String:
		return nil // TODO
	case reflect.Struct:
		return v.visitStruct(w.t)
	}
	return fmt.Errorf("unexpected type: %v [%s]", w.t, w.t.Name())
}

type Visitor interface {
	visitArray(t reflect.Type) error
	visitMap(t reflect.Type) error
	visitPointer(t reflect.Type) error
	visitSlice(t reflect.Type) error
	visitStruct(t reflect.Type) error
	visitStructField(index int, t reflect.StructField) error
}

type TypeTracker map[string]bool

func (t TypeTracker) seen(name string) bool {
	return t[name]
}

func (t TypeTracker) add(name string) {
	if t == nil {
		t = make(TypeTracker)
	}
	t[name] = true
}

type CodeGenerator struct {
	tracker TypeTracker
	output  io.Writer
	toVisit []reflect.Type
}

func (g *CodeGenerator) visitStruct(t reflect.Type) error {
	if g.tracker.seen(t.Name()) {
		return nil
	}
	g.tracker.add(t.Name())
	if _, err := fmt.Fprintf(g.output, "%s\n", t.Name()); err != nil {
		return err
	}
	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		if err := g.visitStructField(i, field); err != nil {
			return err
		}
	}

	var toVisit []reflect.Type
	toVisit, g.toVisit = g.toVisit, nil

	for _, t := range toVisit {
		if err := (&TypeWalker{t: t}).acceptVisitor(g); err != nil {
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
		g.toVisit = append(g.toVisit, f.Type)
	}
	return nil
}

func (g *CodeGenerator) visitMap(t reflect.Type) error {
	if t.Key().Kind() != reflect.String {
		return errors.New("map key must be string")
	}
	if t.Elem().Kind() == reflect.Interface && t.Name() == "" {
		// TODO: JSON value
		return nil
	}
	return (&TypeWalker{t: t.Elem()}).acceptVisitor(g)
}

func (g *CodeGenerator) visitPointer(t reflect.Type) error {
	return (&TypeWalker{t: t.Elem()}).acceptVisitor(g)
}

func (g *CodeGenerator) visitSlice(t reflect.Type) error {
	return (&TypeWalker{t: t.Elem()}).acceptVisitor(g)
}

func (g *CodeGenerator) visitArray(t reflect.Type) error {
	return (&TypeWalker{t: t.Elem()}).acceptVisitor(g)
}
