use std::env;
use std::fs;
mod testolino_fn;

fn main() {
	let args: Vec<String> = env::args().collect(); //prende i parametri opzionali alla eseguzione del programma e li mette in un vettore
	let file_name_in = &args[1];
	let file_name_out = &args[2];
	
	println!("leggo dal file {} ", file_name_in);
	
	let content_in = fs::read_to_string(file_name_in) //legge il contenuto del file in una stringa 
		.expect("Qualcosa è andato storto con la lettura del file :(");
	
	let html = testolino_fn::testolino_to_html(content_in);
	
	fs::File::create(&file_name_out) //creazion del file 
		.expect("qualcosa è andato storto con la creazione del file :(");
	fs::write(file_name_out,&html)
			.expect("Qualcosa è andato storto con la scrittura del file :("); // scrive sul file
	println!("output nel file {}",file_name_out);
}