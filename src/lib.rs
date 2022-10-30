
use neon::{prelude::*};
use once_cell::sync::OnceCell;

#[macro_use]
mod ui;

mod exercise;
mod project;
mod run;
mod verify;
mod entry;

type BoxedConfig = JsBox<Config>;

pub struct Config {
    pub exercises_path: String,
    pub check_list_path: String,
}
impl Finalize for Config {}

pub static PATHCONFIG: OnceCell<Config> = OnceCell::new();

fn init(mut cx: FunctionContext) -> JsResult<JsString> {
    let js_obj = cx.argument::<BoxedConfig>(0)?;
    let exercises_path = js_obj.exercises_path.to_owned();
    let check_list_path = js_obj.check_list_path.to_owned();
    match PATHCONFIG.set(Config { exercises_path, check_list_path }) {
        Ok(_) =>  Ok(cx.string("success")),
        Err(_) =>  Ok(cx.string("error")),
    }
}

fn list(mut cx: FunctionContext) -> JsResult<JsString> {
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::List(entry::ListArgs{
            paths: false, 
            names: false,
            filter: None, 
            unsolved: true, 
            solved: true 
        })),
    };
    entry::cmd(args).unwrap();
    Ok(cx.string("success"))
}

fn watch(mut cx: FunctionContext) -> JsResult<JsString> {
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Watch(entry::WatchArgs{})),
    };
    entry::cmd(args).unwrap();
    Ok(cx.string("success"))
}

fn verify(mut cx: FunctionContext) -> JsResult<JsString> {
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Verify(entry::VerifyArgs{})),
    };
    entry::cmd(args).unwrap();
    Ok(cx.string("success"))
}

fn reset(mut cx: FunctionContext) -> JsResult<JsString> {
    let name = cx.argument::<JsString>(0)?;
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Reset(entry::ResetArgs{ name: name.value(&mut cx) })),
    };
    entry::cmd(args).unwrap();
    Ok(cx.string("success"))
}

fn run(mut cx: FunctionContext) -> JsResult<JsString> {
    let name = cx.argument::<JsString>(0)?;
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Run(entry::RunArgs{ name: name.value(&mut cx) })),
    };
    entry::cmd(args).unwrap();
    Ok(cx.string("success"))
}

fn hint(mut cx: FunctionContext) -> JsResult<JsString> {
    let name = cx.argument::<JsString>(0)?;
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Hint(entry::HintArgs{ name: name.value(&mut cx) })),
    };
    entry::cmd(args).unwrap();
    Ok(cx.string("success"))
}

fn lsp(mut cx: FunctionContext) -> JsResult<JsString> {
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Lsp(entry::LspArgs{ })),
    };
    entry::cmd(args).unwrap();
    Ok(cx.string("success"))
}

fn myverify(mut cx: FunctionContext) -> JsResult<JsString> {
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::MyVerify(entry::MyVerifyArgs{ })),
    };
    if let Some(result) = entry::cmd(args).unwrap() {
        Ok(cx.string(result))
    } else {
        Ok(cx.string("error"))
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("list", list)?;
    cx.export_function("watch", watch)?;
    cx.export_function("verify", verify)?;
    cx.export_function("reset", reset)?;
    cx.export_function("hint", hint)?;
    cx.export_function("lsp", lsp)?;
    cx.export_function("myverify", myverify)?;
    cx.export_function("run", run)?;
    cx.export_function("init", init)?;
    Ok(())
}
