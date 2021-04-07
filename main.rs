use std::env;
use std::fs;

//function to replace all occurency
fn replace_all(to_replace: &str, replace_with: &str, text: String) -> String {
	let mut s = text;
	while s.find(to_replace) != None {
		s = s.replace(to_replace,replace_with);
	}
	s
}

fn rm_illegal_text(illegal_text: &[str],text: String) -> String {
	let mut s = text;
	for index in 0..illegal_text.len() { //per tutti i comandi
		s = replace_all(index, "", s); 
	}
	s
}

fn main() {
	let args: Vec<String> = env::args().collect(); //prende i parametri opzionali alla eseguzione del programma e li mette in un vettore
	let file_name_in = &args[1];
	let file_name_out = &args[2];
	
	let start_html = "<!DOCTYPE html>\n<html>\n<head>\n<title>made by testolino</title>\n</head>\n<body>";
	let end_html = "</body>\n</html>";
	
	let command_to_replace = [".#","#.",".$","$.","|#","|"];
	let replace_to_command = ["<b>","</b>","<i>","</i>","</th><th>","</td><td>"];
	
	let line_command = [".!",".*",".0",".|#",".|"];
	let replace_line_start = ["<h1>","<ul><li>","<ol><li>","<table><tr><th>","<table><tr><td>"]; //elementi *2 elemento *2+1
	let replace_line_finish = ["\n</h1>","\n</li></ul>","\n</li></ol>","\n</th></tr></table>","\n</td></tr></table>"];
	
	let empy_command = ["</p><p>","</ul><ul>","</ol><ol>","</table><table>"];
	
	let mut is_not_command: bool = false; //controlla se è un comando
	
	println!("leggo dal file {} ", file_name_in);
	
	let content_in = fs::read_to_string(file_name_in) //legge il contenuto del file in una stringa 
		.expect("Qualcosa è andato storto con la lettura del file :(");
	
	let mut lines: Vec<&str> = content_in.split('\n').collect(); //fa un vettore content_inete le righe
	let mut content_out = String::new(); //dove va tutto il testo cambiato
	
	content_out.push_str(start_html); //la roba che serve al inizzio
	
	for line_in in lines.iter_mut() {
		let mut line_out = String::new();
		
		//questo blocco si occupa dei comandi a inizio linea
		for index in 0..line_command.len() {
			if line_in.starts_with(line_command[index]) {
				line_out.push_str(replace_line_start[index]);
				line_out.push_str(&line_in[line_command[index].len()..]);
				line_out.push_str(replace_line_finish[index]);
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
		for index in 0..command_to_replace.len() { //per tutti i comandi
			line_out = replace_all(command_to_replace[index],replace_to_command[index],line_out);
		}
		content_out.push_str(&line_out);
	}
	println!("comandi inizzio linea FATTO :)");
	println!("comandi interni alla linea FATTO :)");
	
	//questo blocco si occupa di togliere i comandi vuoti es </p><p> 
	for index in 0..empy_command.len() { //per tutti i comandi
		while content_out.find(empy_command[index]) != None { //cambiare tutte le occorenze
			content_out = content_out.replace(empy_command[index],""); //cambia 
		}
	}
	println!("togiere comandi vuoti vietati FATTO :)");
	
	content_out.push_str(end_html);
	
	fs::File::create(&file_name_out) //creazion del file 
		.expect("qualcosa è andato storto con la creazione del file :(");
	fs::write(file_name_out,&content_out)
			.expect("Qualcosa è andato storto con la scrittura del file :("); // scrive sul file
	println!("output nel file {}",file_name_out);
}