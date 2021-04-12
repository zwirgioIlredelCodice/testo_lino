use std::env;
use std::fs;

const START_HTML:&'static str = "<!DOCTYPE html>\n<html>\n<head>\n<title>made by testolino</title>\n</head>\n<body>";
const END_HTML:&'static str = "</body>\n</html>";
	
const C_TO_REPLACE: [&'static str; 6] = [".#","#.",".$","$.","|#","|"];
const C_REPLACE_WITH: [&'static str; 6] = ["<b>","</b>","<i>","</i>","</th><th>","</td><td>"];
	
const C_LINE: [&'static str; 5] = [".!",".*",".0",".|#",".|"];
const C_REPLACE_START: [&'static str; 5]  = ["<h1>","<ul><li>","<ol><li>","<table><tr><th>","<table><tr><td>"]; //elementi *2 elemento *2+1
const C_REPLACE_FINISH: [&'static str; 5] = ["\n</h1>","\n</li></ul>","\n</li></ol>","\n</th></tr></table>","\n</td></tr></table>"];
	
const C_ILLEGAL: [&'static str; 4] = ["</p><p>","</ul><ul>","</ol><ol>","</table><table>"];

//function to replace all occurency
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
	let mut is_not_command = false; //controlla se è un comando
	
	for index in 0..C_LINE.len() {
		if line.starts_with(C_LINE[index]) {
			line_out.push_str(C_REPLACE_START[index]);
			line_out.push_str(&line[C_LINE[index].len()..]);
			line_out.push_str(C_REPLACE_FINISH[index]);
			is_not_command = false;
			break; //cosi esce subito appena trova un comando percheè non può esseci più di un comando
		}
		else {
			is_not_command = true;
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

fn testolino_to_html(text: String) -> String {
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

fn main() {
	let args: Vec<String> = env::args().collect(); //prende i parametri opzionali alla eseguzione del programma e li mette in un vettore
	let file_name_in = &args[1];
	let file_name_out = &args[2];
	
	println!("leggo dal file {} ", file_name_in);
	
	let content_in = fs::read_to_string(file_name_in) //legge il contenuto del file in una stringa 
		.expect("Qualcosa è andato storto con la lettura del file :(");
	
	let html = testolino_to_html(content_in);
	
	fs::File::create(&file_name_out) //creazion del file 
		.expect("qualcosa è andato storto con la creazione del file :(");
	fs::write(file_name_out,&html)
			.expect("Qualcosa è andato storto con la scrittura del file :("); // scrive sul file
	println!("output nel file {}",file_name_out);
}