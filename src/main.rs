use std::fs;
use std::process::Command;

fn main() {
    //Initialize a tailwind directory with and input.css and output.css file
    let default_input_config = "\
@tailwind base;
@tailwind components;
@tailwind utilities;";

    let tailwind_directory = "tailwind";
    let _ = fs::create_dir(tailwind_directory);

    let input_filename: &str = &format!("{}/input.css", tailwind_directory);
    let output_filename: &str = &format!("{}/output.css", tailwind_directory);

    let _ = fs::write(input_filename, default_input_config);
    let _ = fs::write(output_filename, "");

    //initialize a default tailwind config that sets the main content page to be all html/js files in the project
    let default_tailwind_config = "\
/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [\"*.{html,js}\"],
    theme: {
    extend: {},
    },
    plugins: [],
}";

    let _ = fs::write("tailwind.config.js", default_tailwind_config);

    Command::new("bunx")
        .args([
            "tailwindcss",
            "-i",
            input_filename,
            "-o",
            output_filename,
            "--watch",
        ])
        .spawn()
        .expect("Failed to spawn bunx")
        .wait()
        .unwrap();
}
