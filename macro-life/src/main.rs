macro_rules! myvec {
	($($x:expr), *) => ({  // pattern should match expr $X:expr only, ignoring commas, where there is zero or more due to the outer $(,*), store expr in $x.
		let mut v = Vec::new();
		$(v.push($x);)*
		v
	});
	($($x:expr,)*) => (myvec![$($x),*])  // pattern says match expr ($x:expr,) with a trailing comma, the outer $(*) => means to match zero or more, store expr in $x.
}

// struct Request {
// 	url: String,
// 	body: String,
// 	token: Option<String>,
// }

// struct RequestBuilder {
// 	token: Option<String>,
// }

// impl RequestBuilder {
// 	pub fn new() -> Self {
// 		RequestBuilder{token: None }
// 	}

// 	pub fn token(mut self, token: String) -> Self {
// 		self.token = Some(token);
// 		self
// 	}

// 	pub fn url(self, url: String) -> RequestWithUrlBuilder {
// 		RequestWithUrlBuilder{
// 			url,
// 			token: self.token,
// 		}
// 	}
// }

// struct RequestWithUrlBuilder {
// 	url: String,
// 	token: String,
// }

// impl RequestWithUrlBuilder {
// 	pub fn token(mut self, token: String) -> Self {
// 		self.token = Some(token);
// 		self
// 	}

// 	pub fn body(self, body: String) -> FullRequestBuilder {
// 		FullRequestBuilderque{
// 			body,
// 			token: self.token,
// 			url: self.url,
// 		}
// 	}
// }

// struct FullRequestBuilder {
// 	body: String,
// 	url: String,
// 	token: Option<String>,
// }

// impl FullRequestBuilder {
// 	pub fn token(mut self, token: String) -> Self {
// 		self.token = Some(token);
// 		self
// 	}

// 	pub fn build(self) -> Request {
// 		Request {
// 			url: self.url,
// 			body: self.body,
// 			token: self.token,
// 		}
// 	}
// }

fn main() {
	let a = myvec![1,2,3,4,5,6];
	println!("{:?}", a);

	let an = myvec![1,2,3,4,6,];
	println!("{:?}", an);

	// let req = RequestBuilder::new().url(String::new("some-url")).body(String::new("some-body")).build();

    println!("Hello, world!");
}
