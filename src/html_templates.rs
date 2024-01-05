pub fn generate_random_page(random_number: i64) -> String {
    format!(
        r#"
        <html>
            <head>
                <style>
                @import url('https://fonts.googleapis.com/css2?family=Poppins&display=swap');
                    body {{
                        background-color: black;
                        display: flex;
                        flex-direction:column;
                        align-items: center;
                        justify-content: center;
                        height: 100vh;
                        margin: 0;
                    }}
                    button {{
                        background: linear-gradient(to right, #fd7152, #e75937);
                        color: white;
                        padding: 10px 20px;
                        font-size: 16px;
                        border: none;
                        border-radius: 5px;
                        cursor: pointer;
                        font-weight: bold;
                        font-family: 'Poppins', sans-serif;
                    }}
                    img {{
                        position: absolute;
                        top: 10px;
                        left: 10px;
                        height: 50px; /* Adjust the height as needed */
                        width: auto;
                    }}
                    .description {{
                        font-family: 'Poppins', sans-serif;
                        color: white;
                        padding:6px
                    }}
                    .generated {{
                        font-family: 'Poppins', sans-serif;
                        color: white;
                        font-weight: bold;
                        font-size:15px;
                        font-style:italic;
                        padding:4px;
                    }}
                </style>
            </head>
            <body>
            <img src="https://signoz.io/img/SigNozLogo-orange.svg" alt="SigNoz Logo">
            <div class="description">Sample Rust app with SigNoz monitoring</div> 
            <div class="generated">Generated Number : {}</div> 
                <form action="/" method="get">
                    <button type="submit">Go back</button>
                </form>
            </body>
        </html>
        "#, random_number
    )
}
pub fn generate_index_page() -> String {
    format!(
        r#"
    <html>
        <head>
            <style>
            @import url('https://fonts.googleapis.com/css2?family=Poppins&display=swap');
                body {{
                    background-color: black;
                    display: flex;
                    flex-direction:column;
                    align-items: center;
                    justify-content: center;
                    height: 100vh;
                    margin: 0;
                }}
                button {{
                    background: linear-gradient(to right, #fd7152, #e75937);
                    color: white;
                    padding: 14px 20px;
                    font-size: 16px;
                    border: none;
                    border-radius: 5px;
                    cursor: pointer;
                    font-weight: bold;
                    font-family: 'Poppins', sans-serif;
                }}
                img {{
                    position: absolute;
                    top: 10px;
                    left: 10px;
                    height: 50px; /* Adjust the height as needed */
                    width: auto;
                }}
                .description {{
                    font-family: 'Poppins', sans-serif;
                    color: white;
                    padding:6px;
                }}
            </style>
        </head>
        <body>
        <img src="https://signoz.io/img/SigNozLogo-orange.svg" alt="SigNoz Logo">
        <div class="description">Sample Rust app with SigNoz monitoring</div> 
            <form action="/generate_random" method="post">
                <button type="submit">Generate Random Number</button>
            </form>
        </body>
    </html>
"#
    )
}
