local base = { says: "hi" };

function (foo) {
  sum: 2 + 3,
  external: base {
    tlaFoo: foo,
    extVarBar: std.extVar("bar"),
  },
}
