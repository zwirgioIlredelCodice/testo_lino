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


fn main() {
	let args: Vec<String> = env::args().collect(); //prende i parametri opzionali alla eseguzione del programma e li mette in un vettore
	let file_name_in = &args[1];
	let file_name_out = &args[2];
	
	//let START_HTML = "<!DOCTYPE html>\n<html>\n<head>\n<title>made by testolino</title>\n</head>\n<body>";
	//let END_HTML = "</body>\n</html>";
	
	//let C_TO_REPLACE = [".#","#.",".$","$.","|#","|"];
	//let C_REPLACE_WITH = ["<b>","</b>","<i>","</i>","</th><th>","</td><td>"];
	
	//let C_LINE = [".!",".*",".0",".|#",".|"];
	//let C_REPLACE_START = ["<h1>","<ul><li>","<ol><li>","<table><tr><th>","<table><tr><td>"]; //elementi *2 elemento *2+1
	//let C_REPLACE_FINISH = ["\n</h1>","\n</li></ul>","\n</li></ol>","\n</th></tr></table>","\n</td></tr></table>"];
	
	//let C_ILLEGAL = ["</p><p>","</ul><ul>","</ol><ol>","</table><table>"];
	
	let mut is_not_command: bool = false; //controlla se è un comando
	
	println!("leggo dal file {} ", file_name_in);
	
	let content_in = fs::read_to_string(file_name_in) //legge il contenuto del file in una stringa 
		.expect("Qualcosa è andato storto con la lettura del file :(");
	
	let mut lines: Vec<&str> = content_in.split('\n').collect(); //fa un vettore content_inete le righe
	let mut content_out = String::new(); //dove va tutto il testo cambiato
	
	content_out.push_str(START_HTML); //la roba che serve al inizzio
	
	for line_in in lines.iter_mut() {
		let mut line_out = String::new();
		
		//questo blocco si occupa dei comandi a inizio linea
		for index in 0..C_LINE.len() {
			if line_in.starts_with(C_LINE[index]) {
				line_out.push_str(C_REPLACE_START[index]);
				line_out.push_str(&line_in[C_LINE[index].len()..]);
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
				line_out.push_str(&line_in);
				line_out.push_str("\n</p>");
				is_not_command = false;
		}
		
		//questo blocco si occupa de comandi al interno della riga 
		for index in 0..C_TO_REPLACE.len() { //per tutti i comandi
			line_out = replace_all(C_TO_REPLACE[index],C_REPLACE_WITH[index],line_out);
		}
		content_out.push_str(&line_out);
	}
	println!("comandi inizzio linea FATTO :)");
	println!("comandi interni alla linea FATTO :)");
	
	//questo blocco si occupa di togliere i comandi vuoti es </p><p> 
	for index in 0..C_ILLEGAL.len() { //per tutti i comandi
		while content_out.find(C_ILLEGAL[index]) != None { //cambiare tutte le occorenze
			content_out = content_out.replace(C_ILLEGAL[index],""); //cambia 
		}
	}
	println!("togiere comandi vuoti vietati FATTO :)");
	
	content_out.push_str(END_HTML);
	
	fs::File::create(&file_name_out) //creazion del file 
		.expect("qualcosa è andato storto con la creazione del file :(");
	fs::write(file_name_out,&content_out)
			.expect("Qualcosa è andato storto con la scrittura del file :("); // scrive sul file
	println!("output nel file {}",file_name_out);
}