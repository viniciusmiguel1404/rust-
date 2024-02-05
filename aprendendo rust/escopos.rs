fn main() 
{

	let total = 30;

	{
		let total = 40;
		println!("Trabalhou {} horas", total);
	}

	println!("Trabalhou {} horas", total);

}
