// Handles exporting file to HTML
// use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io;
// use std::io::Read;
use std::io::Write;
use std::fs;

pub struct Export {

}

impl Export {
    pub fn export_as_html(input_value: &str, file_name: String) -> io::Result<()> {
        let path: String = format!("data/{}.html", file_name);
        let path = Path::new(&path);
        let display = path.display();

        // opens a file in write-only mode
        fs::create_dir_all("data")?; // check for directory
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // println!("{:?}", input_value.split("\n").collect::<Vec<_>>());
        let html_text = Export::format_html(&file_name, input_value);
        println!("{}", html_text);

        // inserts input_value into the file
        match file.write_all(&html_text.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => (),
        }
        Ok(())
    }

    fn format_html(file_name: &str, input_value: &str) -> String {
        let text_content: Vec<_> = input_value.split('\n')
            .map(|line| {
                if line.len() > 0 {
                    format!("\t\t\t<p>{}</p>", line)
                }
                else {
                    String::from("\t\t\t<br>")
                }
            })
            .collect();
        // creates a new line for html formatting purposes only
        let text_content = text_content.join("\n"); // no visual effect on page

        let css_styles = "
            html {
                background-color: black;
                color: white;
                font: 1em 'arial';
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
                padding: 20px;
                box-sizing: border-box;
                margin: 20px;
            }
            h1 {
                font-size: 1.5em;
            }
        ";
        
        // keep alignment/tabbing like this to ensure resulting HTML file is formatted correctly
        let html_text = format!(
"<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <title>{}</title>
    <style>
        {}
    </style>
  </head>
  <body>
    <div class=\"container\">
      <h1>{}</h1>
{}
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