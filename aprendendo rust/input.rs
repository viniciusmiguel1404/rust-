use std::io;

fn convert_to_int(data_input: & String) -> i32
{
	let x = data_input.trim().parse::<i32>().unwrap();
	x
}

	
fn main()
{
	
	let mut num1 = String::new();
	io::stdin().read_line(&mut num1).expect("erro ao ler num1");
	let mut num2 = String::new();
	io::stdin().read_line(&mut num2).expect("erro ao ler num2");

	if convert_to_int(&num1) > convert_to_int(&num2)
	{
		println!("o numero {} eh maior que {}", num1, num2);
	}	
	else
	{
		println!("o numero {} eh menor ou igual a {}", num1, num2);
	}
}
