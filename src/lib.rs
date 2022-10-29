use neon::prelude::*;

#[macro_use]
mod ui;

mod exercise;
mod project;
mod run;
mod verify;
mod entry;

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
    entry::cmd(args);
    Ok(cx.string("success"))
}

fn watch(mut cx: FunctionContext) -> JsResult<JsString> {
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Watch(entry::WatchArgs{})),
    };
    entry::cmd(args);
    Ok(cx.string("success"))
}

fn verify(mut cx: FunctionContext) -> JsResult<JsString> {
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Verify(entry::VerifyArgs{})),
    };
    entry::cmd(args);
    Ok(cx.string("success"))
}

fn reset(mut cx: FunctionContext) -> JsResult<JsString> {
    let name = cx.argument::<JsString>(0)?;
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Reset(entry::ResetArgs{ name: name.value(&mut cx) })),
    };
    entry::cmd(args);
    Ok(cx.string("success"))
}

fn run(mut cx: FunctionContext) -> JsResult<JsString> {
    let name = cx.argument::<JsString>(0)?;
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Run(entry::RunArgs{ name: name.value(&mut cx) })),
    };
    entry::cmd(args);
    Ok(cx.string("success"))
}

fn hint(mut cx: FunctionContext) -> JsResult<JsString> {
    let name = cx.argument::<JsString>(0)?;
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Hint(entry::HintArgs{ name: name.value(&mut cx) })),
    };
    entry::cmd(args);
    Ok(cx.string("success"))
}

fn lsp(mut cx: FunctionContext) -> JsResult<JsString> {
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::Lsp(entry::LspArgs{ })),
    };
    entry::cmd(args);
    Ok(cx.string("success"))
}

fn myverify(mut cx: FunctionContext) -> JsResult<JsString> {
    let args = entry::Args {
        nocapture: true,
        version: false,
        nested: Some(entry::Subcommands::MyVerify(entry::MyVerifyArgs{ })),
    };
    entry::cmd(args);
    Ok(cx.string("success"))
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
    Ok(())
}
