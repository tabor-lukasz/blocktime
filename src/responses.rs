pub fn get_not_found() -> String {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = r#"<!DOCTYPE html>
                    <html lang="en">
                        <head>
                            <meta charset="utf-8">
                            <title>Not found!</title>
                        </head>
                        <body>
                            <h1>Oops!</h1>
                            <p>Resource not found</p>
                        </body>
                    </html>"#;

    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    )
}

pub fn get_block_time(timestamp: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        timestamp.len(),
        timestamp
    )
}
