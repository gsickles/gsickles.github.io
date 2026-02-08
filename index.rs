#[macro_use]
#[path = "src/html.rs"]
mod html;

use std::fs;
use minify_js::{minify, TopLevelMode, Session};

fn minify_javascript(js: &str) -> String {
    let session = Session::new();
    let mut out = Vec::new();
    match minify(&session, TopLevelMode::Global, js.as_bytes(), &mut out) {
        Ok(_) => String::from_utf8(out).unwrap_or_else(|_| js.to_string()),
        Err(_) => js.to_string()
    }
}

fn main() {
    let main_js = include_str!("site/js/main.js");
    let minified_main_js = minify_javascript(main_js);
    
    let page = html! {
        html[lang="en"] {
            head {
                meta[charset="UTF-8"] {}
                meta[name="viewport", content="width=device-width, initial-scale=1.0"] {}
                title { "/usr/gsickles" }
                script { (include_str!("site/js/marked.min.js")) }
                style { (include_str!("site/css/main.css")) }
                link[rel="icon", href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>></text></svg>"]
            }
            body[class="dark-mode"] {
                canvas[id="snake-canvas", style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; z-index: -1; pointer-events: none;"] {}
                
                header {
                    h1 {
                        span[onclick="toggleLife()", style="cursor: pointer;"] { "Garrett Sickles" }
                        button[class="theme-toggle", onclick="toggleTheme()"] { "[+]" }
                    }
                }
                
                nav {
                    div {
                        a[href="#", onclick="showSection('about'); return false;", class="active", "data-section"="about"] { "home" }
                        a[href="#", onclick="showSection('blog'); return false;", "data-section"="blog"] { "blog" }
                        a[href="#", onclick="showSection('projects'); return false;", "data-section"="projects"] { "proj" }
                        a[href="#", onclick="showSection('games'); return false;", "data-section"="games"] { "game" }
                        a[href="#", onclick="showSection('tools'); return false;", "data-section"="tools"] { "tool" }
                        a[href="#", onclick="showSection('configs'); return false;", "data-section"="configs"] { "conf" }
                    }
                }
                
                main[id="content"] {
                    div[id="about", class="section"] {
                        article {
                            h2 { "About" }
                            p { "I like video games, coffee, and building stuff." }
                        }
                    }
                    
                    div[id="blog", class="section", style="display: none;"] {
                        article {
                            h2 { "Blog" }
                            div[id="how-this-site-is-built"] {
                                h3[class="collapsible-title", onclick="toggleCode(this); return false;"] {
                                    span[class="toggle-indicator"] { "[+]" }
                                    " How This Site Is Built"
                                    span[class="blog-date"] { "2024-03-08" }
                                    a[href="#how-this-site-is-built", class="entry-link", onclick="event.stopPropagation(); copyEntryLink(this); return false;", title="Copy link"] { "🔗" }
                                }
                                div[class="blog-post"] {
                                    markdown {
                                        (include_str!("site/content/blog/how-this-site-is-built.md"))
                                    }
                                }
                            }
                        }
                    }
                    
                    div[id="projects", class="section", style="display: none;"] {
                        article {
                            h2 { "Projects" }
                            p { "Here are some of the things I've been working on." }
                            div[id="gsickles-github-io"] {
                                h3[class="collapsible-title", onclick="toggleCode(this); return false;"] {
                                    span[class="toggle-indicator"] { "[+]" }
                                    " gsickles.github.io"
                                    a[href="#gsickles-github-io", class="entry-link", onclick="event.stopPropagation(); copyEntryLink(this); return false;", title="Copy link"] { "🔗" }
                                }
                                div[class="project-detail"] {
                                    markdown {
                                        (include_str!("site/content/projects/gsickles-github-io.md"))
                                    }
                                }
                            }
                        }
                    }
                    
                    div[id="games", class="section", style="display: none;"] {
                        article {
                            h2 { "Games" }
                            p { "Click to play!" }
                            p {
                                a[href="site/games/pong.html", target="_blank"] { "Pong" }
                                " - Classic two-player paddle game. Play against the CPU!"
                            }
                            p {
                                a[href="site/games/asteroids.html", target="_blank"] { "Asteroids" }
                                " - Fly through space and destroy asteroids."
                            }
                            p {
                                a[href="site/games/minesweeper.html", target="_blank"] { "Minesweeper" }
                                " - Classic logic puzzle game. Find all the mines!"
                            }
                            p {
                                a[href="site/games/conway.html", target="_blank"] { "Conway's Game of Life" }
                                " - Cellular automaton simulator with classic patterns."
                            }
                        }
                    }
                    
                    div[id="tools", class="section", style="display: none;"] {
                        article {
                            h2 { "Tools" }
                            p {
                                a[href="https://gchq.github.io/CyberChef/", target="_blank"] { "CyberChef" }
                                " - The Cyber Swiss Army Knife - a web app for encryption, encoding, compression and data analysis."
                            }
                            p {
                                a[href="https://godbolt.org/", target="_blank"] { "Godbolt Compiler Explorer" }
                                " - Compiler Explorer is an interactive online compiler which shows the assembly output of compiled C++, Rust, Go (and many more) code."
                            }
                            p {
                                a[href="https://www.overleaf.com/", target="_blank"] { "Overleaf, Online LaTeX Editor" }
                                " - An online LaTeX editor that's easy to use. No installation, real-time collaboration, version control, hundreds of LaTeX templates, and more."
                            }
                            p {
                                a[href="https://github.com/ChrisTitusTech/winutil", target="_blank"] { "Chris Titus Tech's Windows Utility" }
                                " - Streamline Windows 11 installs, debloat with tweaks, troubleshoot with config, and fix Windows updates."
                            }
                            
                        }
                    }
                    
                    div[id="configs", class="section", style="display: none;"] {
                        article {
                            h2 { ".gitconfig" }
                            p {
                                a[href="#", class="toggle-code", onclick="toggleCode(this); return false;"] { "[show]" }
                                " Git configuration with dag alias"
                            }
                            pre {
                                button[class="copy-btn", onclick="copyCode(this)"] { "Copy" }
                                code[contenteditable="true"] {
                                    (include_str!("site/content/config/.gitconfig"))
                                }
                            }
                        }
                        article {
                            h2 { ".bashrc" }
                            p {
                                a[href="#", class="toggle-code", onclick="toggleCode(this); return false;"] { "[show]" }
                                " Bash configuration and aliases"
                            }
                            pre {
                                button[class="copy-btn", onclick="copyCode(this)"] { "Copy" }
                                code[contenteditable="true"] {
                                    (include_str!("site/content/config/.bashrc"))
                                }
                            }
                        }
                    }
                }
                
                footer {
                    p {
                        "Garrett Sickles - garrett@sickles.dev - "
                        a[href="https://github.com/gsickles", target="_blank"] { "GitHub" }
                    }
                }
                
                script { (&minified_main_js) }
            }
        }
    };
    
    fs::write("index.html", page).expect("Failed to write index.html");
    println!("Generated index.html");
}
