#[macro_use] extern crate rocket;

use rocket::response::content::RawHtml;
use reqwest;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct Word {
    american: String,
    british: String,
    indian: String,
}

#[derive(Deserialize)]
struct WordList {
    words: Vec<Word>,
}

// Function to fetch JSON data from the GitHub raw URL
async fn fetch_words() -> Result<WordList, reqwest::Error> {
    let url = "https://raw.githubusercontent.com/chromecruzer/self-learn/main/data.json"; // Replace with your actual URL
    let response = reqwest::get(url).await?;
    let json: WordList = response.json().await?;
    Ok(json)
}

// Function to fetch raw HTML content from a GitHub raw URL
async fn fetch_raw_html() -> Result<String, reqwest::Error> {
    let url = "https://raw.githubusercontent.com/chromecruzer/self-learn/main/index.html"; // Replace with your actual HTML file URL
    let response = reqwest::get(url).await?;
    let html = response.text().await?;
    Ok(html)
}

// Rocket route to serve the dynamic HTML table
#[get("/new-words")]
async fn new_words() -> Result<RawHtml<String>, RawHtml<String>> {
    match fetch_words().await {
        Ok(word_list) => {
            // Start building the HTML table
            let mut table = String::from(
                "<!DOCTYPE html>
                <html lang='en'>
                <head>
                    <meta charset='UTF-8'>
                    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
                    <style>
                        body {
                            font-family: Arial, sans-serif;
                            background: linear-gradient(135deg, #f0f0f0, #e0e0e0);
                            margin: 0;
                            padding: 20px;
                        }
                        table {
                            width: 100%;
                            border-collapse: collapse;
                            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1); 
                            background: white;
                        }
                        th, td {
                            border: 1px solid #ddd;
                            padding: 32px;
                            text-align: left;
                            transition: background 0.3s, transform 0.3s;
                        }
                        th {
                            background: linear-gradient(135deg, #4CAF50, #2E8B57);
                            color: white;
                        }
                        tr:nth-child(even) {
                            background: #f9f9f9;
                        }
                        tr:hover td {
                            background: #f1c40f;
                            color: #00000;
                            transform: scale(1.05);
                        }
                        .button-container {
                            margin-top: 20px;
                        }
                        .download-btn {
                            padding: 10px 20px;
                            font-size: 16px;
                            color: white;
                            background: #3498db;
                            border: none;
                            border-radius: 5px;
                            cursor: pointer;
                            text-decoration: none;
                        }
                        .download-btn:hover {
                            background: #2980b9;
                        }
                    </style>
                </head>
                <body>
                    <table id='wordTable'>
                        <tr>
                            <th>American English</th>
                            <th>British English</th>
                            <th>Indian English</th>
                        </tr>"
            );

            // Table rows populated with data
            for word in word_list.words {
                table.push_str(&format!(
                    "<tr>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                    </tr>",
                    word.american, word.british, word.indian
                ));
            }

            table.push_str("</table>");
            
            // Add a button to download as PDF
            table.push_str(
                "<div class='button-container'>
                    <button class='download-btn' onclick='downloadPDF()'>Download as PDF</button>
                </div>
                <script src='https://unpkg.com/jspdf@latest/dist/jspdf.umd.min.js'></script>
                <script src='https://unpkg.com/jspdf-autotable@latest/dist/jspdf.plugin.autotable.min.js'></script>
                <script>
                    window.onload = () => {
                        const { jsPDF } = window.jspdf;
                        document.querySelector('.download-btn').addEventListener('click', () => {
                            const doc = new jsPDF();
                            doc.autoTable({ html: '#wordTable' });
                            doc.save('table.pdf');
                        });
                    };
                </script>
                </body>
                </html>"
            );

            // Return the generated HTML
            Ok(RawHtml(table))
        },
        Err(_) => {
            // Return an error message if fetching data failed
            Ok(RawHtml(String::from("Failed to fetch data from the server.")))
        }
    }
}

// Rocket route to serve the raw HTML page from GitHub
#[get("/")]
async fn home() -> Result<RawHtml<String>, RawHtml<String>> {
    match fetch_raw_html().await {
        Ok(html) => Ok(RawHtml(html)),
        Err(_) => Ok(RawHtml(String::from("Failed to fetch HTML from the server.")))
    }
}

// Rocket launch configuration
#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    // Read the port from the environment variable 'PORT' provided by Render
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("Invalid port number");

    rocket::build()
        .configure(rocket::Config {
            port,
            ..rocket::Config::default()
        })
        .mount("/", routes![home, new_words])
}