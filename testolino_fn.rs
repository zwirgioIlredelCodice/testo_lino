const START_HTML:&'static str = 
"<html>
<head>
<style>
table, td, th {
  border: 1px solid black;
}

table {
  width: 50%;
  border-collapse: collapse;
}

pre code {
  background-color: #eee;
  border: 1px solid #999;
  display: block;
  padding: 20px;
  page-break-inside: avoid;
  max-width: 50%;
  overflow-x: auto;
  word-wrap: break-word;
}

</style>
</head>
<body>
";
const END_HTML:&'static str = "</body>\n</html>";

const C_TO_REPLACE: [&'static str; 8] = [".#","#.",".$","$.","|#","|",".code","code."];
const C_REPLACE_WITH: [&'static str; 8] = ["<b>","</b>","<i>","</i>","</th><th>","</td><td>","<pre><code>","</code></pre>"];
	
const C_LINE: [&'static str; 5] = [".!",".*",".0",".|#",".|"];
const C_REPLACE_START: [&'static str; 5]  = ["<h1>","<ul><li>","<ol><li>","<table><tr><th>","<table><tr><td>"]; //elementi *2 elemento *2+1
const C_REPLACE_FINISH: [&'static str; 5] = ["\n</h1>","\n</li></ul>","\n</li></ol>","\n</th></tr></table>","\n</td></tr></table>"];


const C_ARGS:  [&'static str; 2] = [".link",".img"];
const C_R_ARGS_START: [&'static str; 2] = ["<a ","<img "];
const C_ARG_START: [&'static str; 2] = ["href=\"","src=\""];
const C_ARG_FINISH: [&'static str; 2] = ["\">","\" alt=\""];
const C_R_ARGS_FINISH: [&'static str; 2] = ["</a><br>\n","\"><br>\n"];

const C_ILLEGAL: [&'static str; 4] = ["</p><p>","</ul><ul>","</ol><ol>","</table><table>"];
	
fn replace_all(to_replace: &str, replace_with: &str, text: String) -> String {
	let mut s = text;
	while s.find(to_replace) != None {
		s = s.replace(to_replace,replace_with);
	}
	s
}
	
//function to delate illegal text
fn delate_c_illegal(text: String) -> String {
	let mut s = text;
	for index in 0..C_ILLEGAL.len() {
		s = replace_all(C_ILLEGAL[index],"",s);
	}
	s
}
	
//function to parse inline commands
fn p_inline_c(text: String) -> String {
	let mut s = text;
	for index in 0..C_TO_REPLACE.len() { //per tutti i comandi
		s = replace_all(C_TO_REPLACE[index],C_REPLACE_WITH[index],s);
	}
	s
}

//function to parse start line commands
fn p_startline_c(line: &str) -> String {
	let mut line_out = String::new();
	let mut is_not_command = false; //controlla se ?? un comando
	
	for index in 0..C_LINE.len() {
		if line.starts_with(C_LINE[index]) {
			line_out.push_str(C_REPLACE_START[index]);
			line_out.push_str(&line[C_LINE[index].len()..]);
			line_out.push_str(C_REPLACE_FINISH[index]);
			is_not_command = false;
			break; //cosi esce subito appena trova un comando perche?? non pu?? esseci pi?? di un comando
		}
		else {
			is_not_command = true;
		}
	}
	if is_not_command {
		for index in 0..C_ARGS.len() {
			if line.starts_with(C_ARGS[index]) {
			
				let mut content = line[C_ARGS[index].len()..].to_string();
			
				line_out.push_str(C_R_ARGS_START[index]);
			
				content = content.replace("[",C_ARG_START[index]); //sas
				content = content.replace("]",C_ARG_FINISH[index]); //sas
				line_out.push_str(&content);
			
				line_out.push_str(C_R_ARGS_FINISH[index]);
				is_not_command = false;
				break; //cosi esce subito appena trova un comando perche?? non pu?? esseci pi?? di un comando
			}
			else {
				is_not_command = true;
			}
		}
	}
	//questo blocco si occupa di assegnare un comando paragrafo alle righe senza altri comandi
	if is_not_command {
			line_out.push_str("<p>");
			line_out.push_str(&line);
			line_out.push_str("\n</p>");
	}
	line_out
}

pub fn testolino_to_html(text: String) -> String {
	let mut lines: Vec<&str> = text.split('\n').collect(); //fa un vettore content_inete le righe
	let mut html = String::new();
	html.push_str(START_HTML); //la roba che serve al inizzio
	
	for line_in in lines.iter_mut() {
		
		//questo blocco si occupa dei comandi a inizio linea
		let mut line_out = p_startline_c(&line_in);
		
		//questo blocco si occupa de comandi al interno della riga 
		line_out = p_inline_c(line_out);
		
		html.push_str(&line_out);
	}
	println!("comandi inizzio linea FATTO :)");
	println!("comandi interni alla linea FATTO :)");
	
	//questo blocco si occupa di togliere i comandi vuoti es </p><p> 
	html = delate_c_illegal(html);
	
	println!("togiere comandi vuoti vietati FATTO :)");
	
	html.push_str(END_HTML);
	
	html
}
