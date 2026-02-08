#[macro_use]
#[path = "../src/html.rs"]
mod html;

use std::fs;

fn main() {
    let page = html! {
        html[lang="en"] {
            head {
                meta[charset="UTF-8"] {}
                meta[name="viewport", content="width=device-width, initial-scale=1.0"] {}
                title { "HTML Macro Showcase" }
                style {
                    "body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; line-height: 1.6; }
table { border-collapse: collapse; width: 100%; margin: 20px 0; }
th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
th { background-color: #f2f2f2; }
code { background-color: #f4f4f4; padding: 2px 6px; border-radius: 3px; }
pre { background-color: #f4f4f4; padding: 15px; border-radius: 5px; overflow-x: auto; }
.highlight { background-color: yellow; }
form { margin: 20px 0; }
label { display: block; margin-top: 10px; }
input, textarea, select { width: 100%; padding: 8px; margin-top: 5px; }
button { margin-top: 10px; padding: 10px 20px; cursor: pointer; }
hr { margin: 30px 0; border: none; border-top: 2px solid #ddd; }
blockquote { border-left: 4px solid #ddd; padding-left: 20px; margin: 20px 0; color: #666; }
figure { margin: 20px 0; text-align: center; }
figcaption { font-style: italic; color: #666; margin-top: 10px; }"
                }
            }
            body {
                header {
                    h1 { "HTML Macro Showcase" }
                    p { "A comprehensive example of all available HTML macros" }
                }
                
                nav {
                    a[href="#headings"] { "Headings" }
                    " | "
                    a[href="#text"] { "Text" }
                    " | "
                    a[href="#lists"] { "Lists" }
                    " | "
                    a[href="#tables"] { "Tables" }
                    " | "
                    a[href="#forms"] { "Forms" }
                    " | "
                    a[href="#media"] { "Media" }
                }
                
                hr {}
                
                main {
                    section[id="headings"] {
                        h2 { "Headings" }
                        h1 { "This is H1" }
                        h2 { "This is H2" }
                        h3 { "This is H3" }
                        h4 { "This is H4" }
                        h5 { "This is H5" }
                        h6 { "This is H6" }
                    }
                    
                    hr {}
                    
                    section[id="text"] {
                        h2 { "Text Formatting" }
                        p { 
                            "This paragraph demonstrates various text formatting options. "
                            "You can use " strong { "strong (bold)" } " text, "
                            em { "emphasized (italic)" } " text, and "
                            code { "inline code" } " formatting."
                        }
                        
                        p {
                            "You can also use " span[class="highlight"] { "spans with classes" }
                            " for custom styling."
                        }
                        
                        p {
                            "Line breaks work like this:" br {}
                            "This is on a new line" br {}
                            "And this is on another line"
                        }
                        
                        blockquote {
                            p { "This is a blockquote. It's useful for highlighting quoted text or important information." }
                        }
                        
                        h3 { "Code Blocks" }
                        pre {
                            code {
                                "fn main() {\n"
                                "    let page = html! {\n"
                                "        html[lang=\"en\"] {\n"
                                "            body { p { \"Hello, world!\" } }\n"
                                "        }\n"
                                "    };\n"
                                "}"
                            }
                        }
                    }
                    
                    hr {}
                    
                    section[id="lists"] {
                        h2 { "Lists" }
                        
                        h3 { "Unordered List" }
                        ul {
                            li { "First item" }
                            li { "Second item" }
                            li { "Third item with " strong { "bold text" } }
                        }
                        
                        h3 { "Ordered List" }
                        ol {
                            li { "Step one" }
                            li { "Step two" }
                            li { "Step three" }
                        }
                        
                        h3 { "Nested Lists" }
                        ul {
                            li { 
                                "Parent item 1"
                                ul {
                                    li { "Child item 1.1" }
                                    li { "Child item 1.2" }
                                }
                            }
                            li { "Parent item 2" }
                        }
                    }
                    
                    hr {}
                    
                    section[id="tables"] {
                        h2 { "Tables" }
                        
                        table {
                            thead {
                                tr {
                                    th { "Name" }
                                    th { "Language" }
                                    th { "Year" }
                                }
                            }
                            tbody {
                                tr {
                                    td { "Rust" }
                                    td { "Systems Programming" }
                                    td { "2010" }
                                }
                                tr {
                                    td { "JavaScript" }
                                    td { "Web Development" }
                                    td { "1995" }
                                }
                                tr {
                                    td { "Python" }
                                    td { "General Purpose" }
                                    td { "1991" }
                                }
                            }
                        }
                    }
                    
                    hr {}
                    
                    section[id="forms"] {
                        h2 { "Forms" }
                        
                        form[action="/submit", method="post"] {
                            label[for="name"] { "Name:" }
                            input[type="text", id="name", name="name", placeholder="Enter your name"] {}
                            
                            label[for="email"] { "Email:" }
                            input[type="email", id="email", name="email", placeholder="your@email.com"] {}
                            
                            label[for="country"] { "Country:" }
                            select[id="country", name="country"] {
                                option[value=""] { "Select a country" }
                                option[value="us"] { "United States" }
                                option[value="uk"] { "United Kingdom" }
                                option[value="ca"] { "Canada" }
                                option[value="au"] { "Australia" }
                            }
                            
                            label[for="message"] { "Message:" }
                            textarea[id="message", name="message", rows="4", placeholder="Enter your message"] {}
                            
                            button[type="submit"] { "Submit Form" }
                        }
                    }
                    
                    hr {}
                    
                    section[id="media"] {
                        h2 { "Media Elements" }
                        
                        h3 { "Images" }
                        figure {
                            img[src="https://via.placeholder.com/400x200", alt="Placeholder image"] {}
                            figcaption { "Figure 1: A placeholder image" }
                        }
                        
                        h3 { "Canvas" }
                        p { "Canvas element for drawing graphics:" }
                        canvas[id="myCanvas", width="400", height="200", style="border: 1px solid #ddd;"] {}
                    }
                    
                    hr {}
                    
                    section[id="other"] {
                        h2 { "Other Elements" }
                        
                        article {
                            h3 { "Article Element" }
                            p { "This is an article element, typically used for self-contained content." }
                        }
                        
                        aside {
                            h4 { "Aside Element" }
                            p { "This is an aside, typically used for sidebars or tangentially related content." }
                        }
                    }
                }
                
                hr {}
                
                footer {
                    p { "This showcase demonstrates all available HTML macros in the templating system." }
                    p { 
                        "Generated with Rust • "
                        a[href="https://github.com"] { "View on GitHub" }
                    }
                }
            }
        }
    };
    
    fs::write("showcase.html", page).expect("Failed to write showcase.html");
    println!("Generated showcase.html");
}
