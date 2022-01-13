const native = require('../../index.node')

module.exports.hello = function () {
  return native.hello();
};

module.exports.getCpuCount = function () {
  return native.get();
};

class Book {
  constructor(){
    this.title = native.chadwick.title;
    this.author = native.chadwick.author;
    this.year = native.chadwick.year;
  }
}

module.exports.Book = Book;
