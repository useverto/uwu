// `a` when immediatly invoked should throw IIFE error.
let a = (() => {
    return {
        1: [eval, 0] // --> eval not found in scope. (0)
    }
})()[1][0]; // --> Computed call expression.

a()[1][0] // --> (0)
