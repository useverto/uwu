{
  var declared = 1;
  declared // Valid
}

declared // Valid
not_declared; // Invalid

f() // Invalid
function f(a) { return a }
f() // Valid

eval // Invalid
