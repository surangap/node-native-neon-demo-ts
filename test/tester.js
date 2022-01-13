const native = require('..')

console.log(native.hello())
console.log(native.get())
console.log(native.chadwick.title)
native.chadwick.title = "blah"
console.log(native.chadwick.title)
