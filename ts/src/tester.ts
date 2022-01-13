import {hello, getCpuCount, Book} from "../../neonWrapper/src/neonWrapper"

// call a string return function
console.log(hello());

// call a number return function
console.log(getCpuCount());

// access an object from native
const book = new Book();
console.log(book);

// change values in native object
book.title = "new titile";
console.log(book)
