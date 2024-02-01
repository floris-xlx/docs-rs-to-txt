use reqwest::blocking::get;
use regex::Regex;
use std::fs::File;
use std::io::{
    Write,
    stdin
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let user_input: String = get_user_input()?;

        if user_input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        // make sure input starts with https://
        let url = if user_input.starts_with("https://") {
            user_input
        } else {
            format!("https://{}", user_input)
        };

        let html_body: String = get(&url)?.text()?;

        // get crate title
        let crate_title: String = get_crate_title(html_body.clone())?;

        // get all text in between <a> - </a>, <p> - </p>, <h1> - </h1>, <h2> - </h2>, <h3> - </h3>, <h4> - </h4>, <h5> - </h5>, <h6> - </h6>
        let all_text_anchors: String = get_all_text_anchors(html_body.clone())?;


        // add the title to a mutable var to be used in the file name and save it in the first line of the file and add a newline for the next line to add the anchors
        let docs_corpus: String = format!("{}\n{}", crate_title, all_text_anchors);

        // save html_body to txt
        let mut file: File = File::create(format!("{}.txt", crate_title))?;
        file.write_all(docs_corpus.as_bytes())?;

        println!("Enter 'exit' to quit or any key to continue...");
    }

    Ok(())
}

fn get_user_input() -> Result<String, Box<dyn std::error::Error>> {
    let mut input: String = String::new();
    println!("Enter a docs.rs URL:");
    stdin().read_line(&mut input)?;
    Ok(input)
}


fn get_crate_title(
    html_body: String
) -> Result<String, Box<dyn std::error::Error>> {
    let title: String = html_body
        .split("<title>")
        .collect::<Vec<&str>>()[1]
        .split("</title>")
        .collect::<Vec<&str>>()[0]
        .to_string();

    // clean up title so it can be used in a filename
    let replace_chars = vec![
        " ", "/", ":", "?", "!", "(", ")", "[", "]", "{", "}", ";", ",", ".", "'", "\"", "\\", "|", "<", ">", "*"
    ];
    // clean up title so it can be used in a filename
    let mut title: String = title;
    for char in replace_chars {
        title = title.replace(char, "");
    }

    Ok(title)
}


fn get_all_text_anchors(
    html_body: String
) -> Result<String, Box<dyn std::error::Error>> {
    let mut anchors: Vec<String> = Vec::new();
    let re = Regex::new(r"<a[^>]*>(.*?)</a>").unwrap();

    for cap in re.captures_iter(&html_body) {
        if let Some(matched) = cap.get(1) {
            anchors.push(matched.as_str().to_string());
        }
    }

    // get all <p> tags
    let re = Regex::new(r"<p[^>]*>(.*?)</p>").unwrap();
    for cap in re.captures_iter(&html_body) {
        if let Some(matched) = cap.get(1) {
            anchors.push(matched.as_str().to_string());
        }
    }

    // get all <h1> tags
    let re = Regex::new(r"<h1[^>]*>(.*?)</h1>").unwrap();
    for cap in re.captures_iter(&html_body) {
        if let Some(matched) = cap.get(1) {
            anchors.push(matched.as_str().to_string());
        }
    }

    // get all <h2> tags
    let re = Regex::new(r"<h2[^>]*>(.*?)</h2>").unwrap();
    for cap in re.captures_iter(&html_body) {
        if let Some(matched) = cap.get(1) {
            anchors.push(matched.as_str().to_string());
        }
    }

    // get all <h3> tags
    let re = Regex::new(r"<h3[^>]*>(.*?)</h3>").unwrap();
    for cap in re.captures_iter(&html_body) {
        if let Some(matched) = cap.get(1) {
            anchors.push(matched.as_str().to_string());
        }
    }

    // get all <h4> tags
    let re = Regex::new(r"<h4[^>]*>(.*?)</h4>").unwrap();
    for cap in re.captures_iter(&html_body) {
        if let Some(matched) = cap.get(1) {
            anchors.push(matched.as_str().to_string());
        }
    }

    // get all <h5> tags
    let re = Regex::new(r"<h5[^>]*>(.*?)</h5>").unwrap();
    for cap in re.captures_iter(&html_body) {
        if let Some(matched) = cap.get(1) {
            anchors.push(matched.as_str().to_string());
        }
    }

    // get all <h6> tags
    let re = Regex::new(r"<h6[^>]*>(.*?)</h6>").unwrap();
    for cap in re.captures_iter(&html_body) {
        if let Some(matched) = cap.get(1) {
            anchors.push(matched.as_str().to_string());
        }
    }

    // get all span tags
    let re = Regex::new(r"<span[^>]*>(.*?)</span>").unwrap();
    for cap in re.captures_iter(&html_body) {
        if let Some(matched) = cap.get(1) {
            anchors.push(matched.as_str().to_string());

        }
    }


    // clean out all <img> tags
    let re = Regex::new(r"<img[^>]*>").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // replace <code> with ```rust and </code> with ```
    let re = Regex::new(r"<code[^>]*>").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "\n```rust\n").to_string();
    }
    let re = Regex::new(r"</code[^>]*>").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "\n```").to_string();
    }



    // remove §
    let re = Regex::new(r"§").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // replace \n\n with \n
    let re = Regex::new(r"\n\n").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "\n").to_string();
    }

    // clean all href="fn.get.html" and its contents
    let re = Regex::new(r#"href=".*?""#).unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // clean all title="fn.get.html" and its contents
    let re = Regex::new(r#"title=".*?""#).unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // clean all class="fn" and its contents
    let re = Regex::new(r#"class=".*?""#).unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // clean all id="fn" and its contents
    let re = Regex::new(r#"id=".*?""#).unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // clean all d="fn" and its contents
    let re = Regex::new(r#"d=".*?""#).unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // remove remaining <a> tags
    let re = Regex::new(r"<a[^>]*>").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // remove remaining </a> tags
    let re = Regex::new(r"</a[^>]*>").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // remove all svg and path
    let re = Regex::new(r"<svg[^>]*>").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    let re = Regex::new(r"</svg[^>]*>").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }


    // remove all <!--! and --> whats in between
    let re = Regex::new(r"<!--!.*?-->").unwrap();

    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // remove all <div> and </div> whats in between
    let re = Regex::new(r"<div[^>]*>").unwrap();

    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // remove all <path />
    let re = Regex::new(r"<path[^>]*>").unwrap();

    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // remove all <path />

    let re = Regex::new(r"</path[^>]*>").unwrap();

    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // remove <span  aria-hidden="true">
    let re = Regex::new(r#"<span  aria-hidden="true">"#).unwrap();

    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "").to_string();
    }

    // make sure theres no more than 1 newline ever without checking '\n\n'
    for anchor in &mut anchors {
        *anchor = anchor.split('\n').filter(|line| !line.is_empty()).collect::<Vec<&str>>().join("\n");
    }


    // add ```rust infront of pub fn and ``` behind
    let re = Regex::new(r"pub fn").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "```rust\npub fn").to_string();
    }
    let re = Regex::new(r"```").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "```\n").to_string();
    }

    // make sure all ```rust is newlined before and after
    let re = Regex::new(r"```rust").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "\n```rust\n").to_string();
    }

    // make sure all ``` is newlined before and after

    let re = Regex::new(r"```").unwrap();
    for anchor in &mut anchors {
        *anchor = re.replace_all(&anchor, "\n```\n").to_string();
    }


    println!("anchors: {:?}", anchors);





    // combine all anchors into one string with newlines
    let anchors: String = anchors.join("\n");

    Ok(anchors)
}