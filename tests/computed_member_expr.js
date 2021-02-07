window["navigator"]; // Valid
[0, globalThis][1]; // Valid

window["ev"+ "al"](); // Invalid
globalThis["Deno"].run(); // Invalid
globalThis["Deno"]["run"](); // Invalid
a.b.c["c"].a.b(); // Invalid
[Deno][0].run(); // Invalid
(eval)(); // Invalid
