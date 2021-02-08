let c = (() => [eval])()[0] // Invalid
c() // Won't error `c` not found in scope.
