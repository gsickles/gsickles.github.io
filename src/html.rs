// HTML macro system

#[macro_export]
macro_rules! html {
    ($($content:tt)*) => {{
        let mut output = String::new();
        output.push_str("<!DOCTYPE html>\n");
        html_inner!(output, 0, $($content)*);
        output
    }};
}

#[macro_export]
macro_rules! html_inner {
    ($output:ident, $indent:expr, ($var:expr) $($rest:tt)*) => {
        $output.push_str($var);
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, $text:literal $($rest:tt)*) => {
        $output.push_str($text);
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, a [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str("<a");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</a>");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, button [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<button");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</button>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, button { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<button>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</button>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, span [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str("<span");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</span>");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, strong [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str("<strong");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</strong>");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, strong { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str("<strong>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</strong>");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, em [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str("<em");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</em>");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, em { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str("<em>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</em>");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, label [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<label");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</label>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, label { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<label>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</label>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, td [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<td");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</td>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, td { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<td>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</td>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, th [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<th");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</th>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, th { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<th>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</th>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, code [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str("<code");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</code>");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, code { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str("<code>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</code>");
        html_inner!($output, $indent, $($rest)*);
    };
    
    // Text container elements that should be on their own line: h1, h2, p, title
    ($output:ident, $indent:expr, h1 { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<h1>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</h1>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, h2 { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<h2>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</h2>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, h3 { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<h3>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</h3>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, h4 { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<h4>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</h4>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, h5 { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<h5>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</h5>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, h6 { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<h6>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</h6>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, p { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<p>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</p>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, title { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<title>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</title>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, style [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<style");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        html_inner!($output, $indent + 1, $($content)*);
        $output.push_str("\n");
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("</style>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, style { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<style>\n");
        html_inner!($output, $indent + 1, $($content)*);
        $output.push_str("\n");
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("</style>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, nav [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<nav");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        $output.push_str(&"\t".repeat($indent + 1));
        html_inner!($output, $indent + 1, $($content)*);
        if !$output.ends_with('\n') {
            $output.push('\n');
        }
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("</nav>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, nav { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<nav>\n");
        $output.push_str(&"\t".repeat($indent + 1));
        html_inner!($output, $indent + 1, $($content)*);
        if !$output.ends_with('\n') {
            $output.push('\n');
        }
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("</nav>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, li { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<li>");
        html_inner!($output, $indent + 1, $($content)*);
        if $output.ends_with('\n') {
            $output.push_str(&"\t".repeat($indent));
        }
        $output.push_str("</li>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, tr [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<tr");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        html_inner!($output, $indent + 1, $($content)*);
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("</tr>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, tr { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<tr>\n");
        html_inner!($output, $indent + 1, $($content)*);
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("</tr>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, option [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<option");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</option>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, option { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<option>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</option>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, figcaption [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<figcaption");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</figcaption>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, figcaption { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<figcaption>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</figcaption>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, ul [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        if !$output.is_empty() && !$output.ends_with('\n') {
            $output.push('\n');
        }
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<ul");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        html_inner!($output, $indent + 1, $($content)*);
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("</ul>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, ul { $($content:tt)* } $($rest:tt)*) => {
        if !$output.is_empty() && !$output.ends_with('\n') {
            $output.push('\n');
        }
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<ul>\n");
        html_inner!($output, $indent + 1, $($content)*);
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("</ul>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, ol [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        if !$output.is_empty() && !$output.ends_with('\n') {
            $output.push('\n');
        }
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<ol");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        html_inner!($output, $indent + 1, $($content)*);
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("</ol>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, ol { $($content:tt)* } $($rest:tt)*) => {
        if !$output.is_empty() && !$output.ends_with('\n') {
            $output.push('\n');
        }
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<ol>\n");
        html_inner!($output, $indent + 1, $($content)*);
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("</ol>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    // Markdown block - creates a div with markdown class for marked.js to process
    ($output:ident, $indent:expr, markdown { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<div class=\"markdown\">");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</div>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, pre { $($content:tt)* } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<pre>");
        html_inner!($output, $indent, $($content)*);
        $output.push_str("</pre>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, link [ $($attr_content:tt)* ] $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<link");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, br [ $($attr_content:tt)* ] $($rest:tt)*) => {
        $output.push_str("<br");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, br { } $($rest:tt)*) => {
        $output.push_str("<br>");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, br $($rest:tt)*) => {
        $output.push_str("<br>");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, hr [ $($attr_content:tt)* ] $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<hr");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, hr { } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<hr>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, hr $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<hr>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, img [ $($attr_content:tt)* ] { } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<img");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, img [ $($attr_content:tt)* ] $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<img");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, input [ $($attr_content:tt)* ] { } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<input");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, input [ $($attr_content:tt)* ] $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<input");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, canvas [ $($attr_content:tt)* ] { } $($rest:tt)*) => {
        $output.push_str(&"\t".repeat($indent));
        $output.push_str("<canvas");
        attrs_inner!($output, $($attr_content)*);
        $output.push_str("></canvas>\n");
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, $tag:ident [ $($attr_content:tt)* ] { $($content:tt)* } $($rest:tt)*) => {
        if !$output.is_empty() && !$output.ends_with('\n') {
            $output.push('\n');
        }
        $output.push_str(&"\t".repeat($indent));
        $output.push_str(concat!("<", stringify!($tag)));
        attrs_inner!($output, $($attr_content)*);
        $output.push_str(">\n");
        html_inner!($output, $indent + 1, $($content)*);
        $output.push_str(&"\t".repeat($indent));
        $output.push_str(concat!("</", stringify!($tag), ">\n"));
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr, $tag:ident { $($content:tt)* } $($rest:tt)*) => {
        if !$output.is_empty() && !$output.ends_with('\n') {
            $output.push('\n');
        }
        $output.push_str(&"\t".repeat($indent));
        $output.push_str(concat!("<", stringify!($tag), ">\n"));
        html_inner!($output, $indent + 1, $($content)*);
        $output.push_str(&"\t".repeat($indent));
        $output.push_str(concat!("</", stringify!($tag), ">\n"));
        html_inner!($output, $indent, $($rest)*);
    };
    
    ($output:ident, $indent:expr,) => {};
}

#[macro_export]
macro_rules! attrs_inner {
    ($output:ident, $attr:literal = $val:tt) => {
        $output.push_str(concat!(" ", $attr, "=\""));
        $output.push_str($val);
        $output.push_str("\"");
    };
    
    ($output:ident, $attr:literal = $val:tt, $($rest:tt)*) => {
        $output.push_str(concat!(" ", $attr, "=\""));
        $output.push_str($val);
        $output.push_str("\"");
        attrs_inner!($output, $($rest)*);
    };
    
    ($output:ident, $attr:ident = $val:tt) => {
        $output.push_str(concat!(" ", stringify!($attr), "=\""));
        $output.push_str($val);
        $output.push_str("\"");
    };
    
    ($output:ident, $attr:ident = $val:tt, $($rest:tt)*) => {
        $output.push_str(concat!(" ", stringify!($attr), "=\""));
        $output.push_str($val);
        $output.push_str("\"");
        attrs_inner!($output, $($rest)*);
    };
    
    ($output:ident,) => {};
}
