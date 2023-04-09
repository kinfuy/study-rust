use npm_init::npm::{Output, Package, Script};
use npm_init::{console, tips};

fn main() {
    console::log(tips::DEFAULT_TIP);
    let name = tips::get_input("package name:", "npm-init");
    let version = tips::get_input("version:", "1.0.0)");
    let description = tips::get_input("description", "");
    let entry = tips::get_input("entry point: ", "index.js");
    let test_command = tips::get_input(
        "test command",
        "echo \"Error: no test specified\" && exit 1",
    );
    let git_repo = tips::get_input("git repository:", "https://github.com/kinfuy/npm-init.git");
    let keyword = tips::get_input("keywords", "");
    let author = tips::get_input("author", "");
    let license = tips::get_input("license", "ISC");
    let is_create = tips::get_input("Is this OK?", "yes");

    let pkg = Package {
        name,
        version,
        description,
        entry,
        script: Script { test: test_command },
        git_repo,
        keyword,
        author,
        license,
        is_create,
    };

    pkg.output().unwrap();
}
