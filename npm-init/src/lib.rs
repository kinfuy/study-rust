pub mod tips {
    use std::io;
    pub const DEFAULT_TIP: &str = "This utility will walk you through creating a package.json file.
It only covers the most common items, and tries to guess sensible defaults.

See `npm help init` for definitive documentation on these fields
and exactly what they do.

Use `npm install <pkg>` afterwards to install a package and
save it as a dependency in the package.json file.

Press ^C at any time to quit.";

    pub fn get_input(msg: &str, default: &str) -> String {
        println!("{}{}:", msg, format!("({})", default));
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.replace("\n", "");
        if input == "".to_owned() {
            default.to_string()
        } else {
            input
        }
    }
}

pub mod console {
    pub fn log(msg: &str) {
        println!("{}", msg)
    }
    pub fn warn() {}
    pub fn error() {}
    pub fn success() {}
}

pub mod npm {
    use serde::{Deserialize, Serialize};
    use std::{fs, io};

    #[derive(Serialize, Deserialize)]
    pub struct Script {
        pub test: String,
    }
    #[derive(Serialize, Deserialize)]
    pub struct Package {
        pub name: String,
        pub version: String,
        pub description: String,
        pub entry: String,
        pub script: Script,
        pub git_repo: String,
        pub keyword: String,
        pub author: String,
        pub license: String,
        pub is_create: String,
    }

    pub trait Output {
        /// 输出到package.json
        fn output(&self) -> Result<(), io::Error>
        where
            Self: Serialize;
    }
    impl Output for Package {
        fn output(&self) -> Result<(), std::io::Error>
        where
            Self: Serialize,
        {
            let serialized = serde_json::to_string(&self).unwrap();
            fs::write("package.json", serialized)?;
            Ok(())
        }
    }
}
