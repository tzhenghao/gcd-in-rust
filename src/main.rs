fn gcd(mut n: u64, mut m: u64) -> u64 {

	assert!(n != 0 && m != 0);
	while m != 0 {
		if m < n {
			let t = m; m = n; n = t;
		}
		m = m % n;
	}
	n
}

use std::io::Write;
use std::str::FromStr;

extern crate iron;
#[macro_use] extern crate mime;

extern crate router;
use router::Router;

extern crate urlencoded;

use iron::prelude::*;
use iron::status;

//fn main() {
//
//	let mut numbers = Vec::new();
//
//	for arg in std::env::args().skip(1) {
//
//		numbers.push(u64::from_str(&arg)
//					.expect("error parsing argument"));
//	}
//
//	if numbers.len() == 0 {
//		writeln!(std::io::stderr(), "Usage: gcd NUMBER...").unwrap();
//		std::process::exit(1);
//	}
//
//	let mut d = numbers[0];
//	for m in &numbers[1..] {
//		d = gcd(d, *m);
//	}
//
//    println!("Hello, world!");
//	println!("The greatest common divisor of {:?} is {}",
//				numbers, d);
//}

fn main() {

	let mut router = Router::new();

	router.get("/", get_form);
	router.post("/gcd", post_gcd);

	println!("Serving on http://localhost:3000...");
	Iron::new(router).http("localhost:3000").unwrap();
}


#[test]
fn test_gcd() {
	assert_eq!(gcd(2 * 5 * 11 * 17,
					3 * 7 * 13 * 19),
					1);

	assert_eq!(gcd(2 * 3  * 5 * 11 * 17,
					3 * 7 * 11 * 13 * 19),
					3 * 11);
}

#[allow(unused_variables)]
fn get_form(request: &mut Request) -> IronResult<Response> {

	let mut response = Response::new();

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(r#"
		<title>GCD Calculator</title>
		<form action="/gcd" method="post">
			<input type="text" name="n"/>
			<input type="text name="n'/>
			<button type="submit">Compute GCD</button>
		</form>
	"#);

	Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {

	let mut response = Response::new();

	let hashmap;
	match request.get_ref::<UrlEncodedBody>() {
		Err(e) => {
			response.set_mut(status::BadRequest);
			response.set_mut(format!("Error parsing form data: {:?}\n", e));

			return Ok(response);
		}
		Ok(map) => { hashmap = map; }
	}

	let unparsed_numbers;
	match hashmap.get("n") {

		None => {
			response.set_mut(status::BadRequest);
			response.set_mut(format!("form data has no 'n' parameters\n"));
			return Ok(response);
		}
		Some(nums) => { unparsed_numbers = nums; }
	}

	let mut numbers = Vec::new();

	for unparsed in unparsed_numbers {
		match u64::from_str(&unparsed) {
			Err(_) => {
				response.set_mut(status::BadRequest);
				response.set_mut(format!("Value for 'n' parameter not a number: {:?}\n", unparsed));
				return Ok(response);
			}
			Ok(n) => { numbers.push(n); }
		}
	}

	let mut d = numbers[0];
	for m in &numbers[1..] {
		d = gcd(d, *m);
	}

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
							numbers, d));
	Ok(response)
}
