use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn get_num_cpus(mut cx: FunctionContext) -> JsResult<JsNumber> {
  Ok(cx.number(num_cpus::get() as f64))
}

struct Book {
  pub title: String,
  pub author: String,
  pub year: u32,
}

impl Book {
  fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let title = cx.string(self.title.clone());
    obj.set(cx, "title", title)?;

    let author = cx.string(&self.author);
    obj.set(cx, "author", author)?;

    let year = cx.number(self.year);
    obj.set(cx, "year", year)?;

    Ok(obj)
  }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("get", get_num_cpus)?;

    let book = Book {
        title: "Chadwick the Crab".to_string(),
        author: "Priscilla Cummings".to_string(),
        year: 2010,
    };

    let obj = book.to_object(&mut cx)?;
    cx.export_value("chadwick", obj)?;
    Ok(())
}
