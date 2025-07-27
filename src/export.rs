// Handles exporting file to HTML
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::Write;
use std::fs;
use std::process::Command;

pub struct Export {
    
}

impl Export {
    pub fn export_as_styled_html(input_value: String, file_name: String) -> io::Result<()> {
    let css_styles: String = 
    String::from("html {
            background-color: black;
            color: white;
            font-family: \"Cascadia Code\", monospace, Arial;
            }
        html,
        body {
            display: flex;
            margin: 0;
            padding: 0;
            height: 100%;
            width: 100%;
        }
        .container {
            flex: 1;
            border: 2px solid white;
            box-sizing: border-box;
            margin: 20px;
        }
        .title-container {
            border-bottom: 2px solid white;
            padding: 0 10px;
        }
        .content-container {
            line-height: 7px;
            padding: 0 10px;
        }
        h1 {
            font-size: 1em;
        }
        li {
            list-style-type: disc;
            margin: 20px;
        }
        p,
        li {
            font-size: 1em;
        }");

        Export::export_as_html(input_value, file_name, css_styles)
    }

    pub fn export_as_plain_html(input_value: String, file_name: String) -> io::Result<()> {
        let css_styles = String::from("");
        Export::export_as_html(input_value, file_name, css_styles)
    }

    pub fn export_as_text(input_value: String, file_name: String) -> io::Result<()> {
        let path: String = format!("data/{}.txt", file_name);
        let path = Path::new(&path);
        let display = path.display();

        // opens a file in write-only mode
        fs::create_dir_all("data")?; // check for directory
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        
        // inserts input_value into the file
        match file.write_all(&input_value.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => (),
        }
        Ok(())
    }

    fn export_as_html(input_value: String, file_name: String, css_styles: String) -> io::Result<()> {
        let path: String = format!("data/{}.html", file_name);
        let path = Path::new(&path);
        let display = path.display();
        
        // opens a file in write-only mode
        fs::create_dir_all("data")?; // check for directory
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        
        let html_text = Export::format_html(&file_name, input_value, css_styles);
        
        // inserts input_value into the file
        match file.write_all(&html_text.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => (),
        }
        Ok(())
    }

    pub fn open_in_file_explorer() {
        let path = String::from("data");

        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "start", "", &path])
                .spawn()
                .expect("start command doesn't work");
        } 
        else if cfg!(target_os = "linux") {
            Command::new("xdg-open")
                .arg(&path)
                .spawn()
                .expect("open command doesn't work");
        }
        else if cfg!(target_os = "macos") {
            Command::new("open")
                .arg(&path)
                .spawn()
                .expect("open command doesn't work");
        };
    }

    fn format_html(file_name: &str, input_value: String, css_styles: String) -> String {
        let split_text: Vec<&str> = input_value.split('\n').collect();
        let text_content: Vec<_> = split_text
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, line)| {
                if line.len() > 0 {
                    if line.starts_with("* ") {
                        // parses list items
                        if index != 0 && index == split_text.len() {
                            if !split_text[index - 1].starts_with("* ") {
                                format!("\t\t\t<ul><li>{}</li>", line.strip_prefix("* ").unwrap())
                            }
                            else if !split_text[index + 1].starts_with("* ") {
                                format!("\t\t\t<li>{}</li></ul>", line.strip_prefix("* ").unwrap())
                            }
                            else {
                                format!("\t\t\t<li>{}</li>", line.strip_prefix("* ").unwrap())
                            };
                        }
                        format!("\t\t\t<li>{}</li>", line.strip_prefix("* ").unwrap())
                    }
                    else {
                        // paragraph html element
                        format!("\t\t\t<p>{}</p>", line)
                    }
                }
                else {
                    // blank line
                    String::from("\t\t\t<br>")
                }
            })
            .collect();
        // creates a new line for html formatting purposes only
        let text_content = text_content.join("\n"); // no visual effect on page        
        
        // keep alignment/tabbing like this to ensure resulting HTML file is formatted correctly
        let html_text = format!(
"<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <title>{}</title>

    <link rel=\"preconnect\" href=\"https://fonts.googleapis.com\" />
    <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin />
    <link
      href=\"https://fonts.googleapis.com/css2?family=Cascadia+Code:ital,wght@0,200..700;1,200..700&display=swap\"
      rel=\"stylesheet\"
    />

    <style>
        {}
    </style>
  </head>
  <body>
    <div class=\"container\">
        <div class=\"title-container\">
        <h1>{}</h1>
        </div>
        <div class=\"content-container\">
    {}
        </div>
    </div>
  </body>
</html>", file_name, css_styles, file_name, text_content);

        html_text
    }

    fn _copy_as_html() {
        // let mut file = match File::open(&path) {
        //     Err(why) => panic!("couldn't open {}: {}", display, why),
        //     Ok(file) => file,
        // };
    
        // let mut s = String::new();
        // match file.read_to_string(&mut s) {
        //     Err(why) => panic!("couldn't read {}: {}", display, why),
        //     Ok(_) => print!("{} contains:\n{}", display, s),
        // }
    }
}