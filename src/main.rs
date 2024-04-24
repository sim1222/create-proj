use std::env;
use clap::Parser;

extern crate dirs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: String,
}

fn main() {
    let args = Args::parse();

    let home_dir = dirs::home_dir().unwrap();

    let repos = home_dir.join("source/repos");
    env::set_current_dir(repos).unwrap();

    let project_name = args.name.as_str();

    std::fs::create_dir(project_name).unwrap();

    let project_dir = format!("{}/{}", project_name, project_name);

    std::fs::create_dir(&project_dir).unwrap();

    let sln_file = include_str!("template/ex1-1-4.sln");
    let sln_file = sln_file.replace("{{{template}}}", project_name);

    let cpp_file = include_bytes!("template/ex1-1-4/ex1-1-4.cpp");

    let vcxproj_file = include_str!("template/ex1-1-4/ex1-1-4.vcxproj");
    let vcxproj_file = vcxproj_file
        .replace("{{{template}}}", project_name)
        .replace(
            "{{{template_no_minus}}}",
            project_name.replace('-', "").as_str(),
        );

    let vcxproj_filters_file = include_str!("template/ex1-1-4/ex1-1-4.vcxproj.filters");
    let vcxproj_filters_file = vcxproj_filters_file.replace("{{{template}}}", project_name);

    let vcxproj_user_file = include_str!("template/ex1-1-4/ex1-1-4.vcxproj.user");
    let vcxproj_user_file = vcxproj_user_file.replace("{{{template}}}", project_name);

    std::fs::write(format!("{}/{}.sln", project_name, project_name), sln_file).unwrap();
    std::fs::write(format!("{}/{}.cpp", &project_dir, project_name), cpp_file).unwrap();
    std::fs::write(
        format!("{}/{}.vcxproj", &project_dir, project_name),
        vcxproj_file,
    )
    .unwrap();
    std::fs::write(
        format!("{}/{}.vcxproj.filters", &project_dir, project_name),
        vcxproj_filters_file,
    )
    .unwrap();
    std::fs::write(
        format!("{}/{}.vcxproj.user", &project_dir, project_name),
        vcxproj_user_file,
    )
    .unwrap();

    println!("Project {} created", project_name);
    println!("Opening in Visual Studio...");

    let devenv_paths = [
        r#"C:\Program Files\Microsoft Visual Studio\2022\Professional\Common7\IDE\devenv.exe"#,
        r#"C:\Program Files (x86)\Microsoft Visual Studio\2022\Professional\Common7\IDE\devenv.exe"#,
        r#"C:\Program Files\Microsoft Visual Studio\2019\Professional\Common7\IDE\devenv.exe"#,
        r#"C:\Program Files (x86)\Microsoft Visual Studio\2019\Professional\Common7\IDE\devenv.exe"#,
    ];

    devenv_paths.iter().for_each(|path| {
        if std::fs::metadata(path).is_ok() {
            std::process::Command::new(path)
                .arg(format!("{}/{}.sln", project_name, project_name))
                .spawn()
                .expect("Failed to open Visual Studio");

            std::process::exit(0);
        }
    });
}
