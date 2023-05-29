package main

import (
	"errors"
	"fmt"
	"io"
	"os"
	"reflect"
	"strings"

	"github.com/iancoleman/strcase"

	// v1.5.6: go get github.com/hashicorp/nomad/api@8af70885c02ab921dedbdf6bc406a1e886866f80
	"github.com/hashicorp/nomad/api"
)

func main() {
	job := reflect.TypeOf(api.Job{})
	w := TypeWalker{t: job}
	cg := CodeGenerator{tracker: make(TypeTracker), output: os.Stdout}
	cg.emitFileStart()
	if err := w.acceptVisitor(&cg); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	cg.emitFileEnd()
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

	if err := g.emitStructStart(t.Name()); err != nil {
		return err
	}

	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		if err := g.visitStructField(i, field); err != nil {
			return err
		}
	}

	if err := g.emitStructEnd(); err != nil {
		return err
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

type tagAttrs struct {
	configName string
	label      bool
	optional   bool
	block      bool
}

func (g *CodeGenerator) visitStructField(i int, f reflect.StructField) error {
	if value, ok := f.Tag.Lookup("hcl"); ok {
		values := strings.Split(value, ",")
		attrs := tagAttrs{
			configName: values[0],
		}
		for _, attr := range values[1:] {
			switch attr {
			case "label":
				attrs.label = true
			case "optional":
				attrs.optional = true
			case "block":
				attrs.block = true
			default:
				return fmt.Errorf("unknown tag attribute: %s", attr)
			}
		}

		if err := g.emitStructField(f, attrs); err != nil {
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

func (g *CodeGenerator) emitFileStart() error {
	_, err := fmt.Fprintf(g.output, `use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
`)
	return err
}

func (g *CodeGenerator) emitFileEnd() error {
	return nil
}

func (g *CodeGenerator) emitStructStart(name string) error {
	_, err := fmt.Fprintf(g.output, `
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct %s {
`, name)
	return err
}

func (g *CodeGenerator) emitStructEnd() error {
	_, err := fmt.Fprintf(g.output, "}\n")
	return err
}

func (g *CodeGenerator) emitStructField(f reflect.StructField, attrs tagAttrs) error {
	fieldName := attrs.configName
	if fieldName == "" {
		fieldName = strcase.ToSnake(f.Name)
	}
	serName := f.Name
	deserName := fieldName
	switch fieldName {
	case "type", "static":
		fieldName = "r#" + fieldName
	}

	t := f.Type
	repeated := false
	if t.Kind() == reflect.Slice {
		repeated = true
		t = t.Elem()
	}
	if t.Kind() == reflect.Ptr {
		t = t.Elem()
	}

	isMap := false
	rustType := "()"
	switch t.Kind() {
	case reflect.Bool:
		rustType = "bool"
	case reflect.Int:
		rustType = "isize"
	case reflect.Int8:
		rustType = "i8"
	case reflect.Int16:
		rustType = "i16"
	case reflect.Int32:
		rustType = "i32"
	case reflect.Int64:
		rustType = "i64"
	case reflect.Uint:
		rustType = "usize"
	case reflect.Uint8:
		rustType = "u8"
	case reflect.Uint16:
		rustType = "u16"
	case reflect.Uint32:
		rustType = "u32"
	case reflect.Uint64:
		rustType = "u64"
	case reflect.Float32:
		rustType = "f32"
	case reflect.Float64:
		rustType = "f64"
	case reflect.Map:
		isMap = true
		if t.Elem().Kind() == reflect.Interface {
			rustType = "serde_json::Value"
		} else if t.Elem().Kind() == reflect.String {
			rustType = "String"
		} else if t.Elem().Kind() == reflect.Pointer && t.Elem().Elem().Kind() == reflect.Struct {
			rustType = t.Elem().Elem().Name()
		} else if t.Elem().Kind() == reflect.Slice && t.Elem().Elem().Kind() == reflect.String { // TODO: more flexible
			repeated = true
			rustType = "String"
		} else {
			// rustType = "()"
			return fmt.Errorf("unexpected map value type: %v", t.Elem())
		}
	case reflect.String:
		rustType = "String"
	case reflect.Struct:
		rustType = t.Name()
	default:
		return fmt.Errorf("unexpected field type: %v", t)
	}

	if repeated {
		rustType = "Vec<" + rustType + ">"
	}
	if isMap {
		rustType = "IndexMap<String, " + rustType + ">"
	}
	if attrs.optional || attrs.block {
		rustType = "Option<" + rustType + ">"
	}

	_, err := fmt.Fprintf(g.output, `
    #[serde(rename(deserialize = %q, serialize = %q), default)]
    pub %s: %s,
`, deserName, serName, fieldName, rustType)
	return err
}
