//! Lexer for the world definition language

use super::super::resource_manager::*;
use macroquad::prelude::*;
use super::super::object::ObjectKind;

#[derive(Debug)]
pub enum TokenizerError {
	UnknownToken (String),
	UnexpectedEOF,
	UnknownUnit (String)
}


enum VectorParsingState {
	Initial,
	X,
	Y,
	Bidimensional,
	ScalarDefault,
	ScalarUnit,
	Done
}

#[derive(Debug)]
pub enum Unit {
	Default,
	Pixel
}

#[derive(Debug)]
pub enum Token {
	Unit,
	Kind (ObjectKind),
	Is,
	At,
	OfSize,
	WithIS,
	Vector (i32, i32),
	Scalar (i32, Unit),
	EndOfFile
}
impl Token {
	async fn from(s: &str, rm: &mut ResourceManager) -> Result::<Self, TokenizerError> {
		match s {
			"Unit" => Ok(Token::Unit),
			"Spike" | "Player" | "Wall" => Ok(Token::Kind(ObjectKind::from((s, rm)).await)),
			"is" => Ok(Token::Is),
			"at" => Ok(Token::At),
			"ofsize" => Ok(Token::OfSize),
			"wiso" => Ok(Token::WithIS),
			_ => {
				// Parsing vector and scalar litterals

			  	let mut state = VectorParsingState::Initial;

			  	let word = format!("{s}\n");
			  	let mut buffer = String::new();
			  	let mut parsed = (false, 0, 0);

			  	for c in word.chars() {
			  		buffer.push(c);
			  		
			  		// Complete, deterministic, simple automaton
			  		state = match state {
			  			VectorParsingState::Initial => match c {
			  				'(' => {
			  					buffer.clear();
			  					parsed.0 = true;
			  					VectorParsingState::X
			  				},
			  				'0' ..= '9' => VectorParsingState::ScalarDefault,
			  				 _ => return Err(TokenizerError::UnknownToken(s.to_string()))
			  			},
			  			VectorParsingState::X => match c {
			  				'0' ..= '9' => VectorParsingState::X,
			  				',' => {
			  					buffer.pop().unwrap();
			  					parsed.1 = buffer.parse::<i32>().unwrap();
			  					buffer.clear();
			  					VectorParsingState::Y
			  				},
			  				 _ => return Err(TokenizerError::UnknownToken(s.to_string()))
			  			},
			  			VectorParsingState::Y => match c {
			  				'0' ..= '9' => VectorParsingState::Y,
			  				')' => {
			  					buffer.pop().unwrap();
			  					parsed.2 = buffer.parse::<i32>().unwrap();
			  					buffer.clear();
			  					VectorParsingState::Bidimensional
			  				},
			  				 _ => return Err(TokenizerError::UnknownToken(s.to_string()))
			  			},
			  			VectorParsingState::Bidimensional => match c {
			  				'\n' => VectorParsingState::Done,
			  				_ => return Err(TokenizerError::UnknownToken(s.to_string())),
			  			},
			  			VectorParsingState::ScalarDefault => match c {
			  				'0' ..= '9' => VectorParsingState::ScalarDefault,
			  				'\n' => {
			  					buffer.pop().unwrap();
			  					parsed.1 = buffer.parse::<i32>().unwrap();
			  					buffer.clear();
			  					VectorParsingState::Done
			  				},
			  				'p' => {
			  					buffer.pop().unwrap();
			  					parsed.1 = buffer.parse::<i32>().unwrap();
			  					buffer.clear();
			  					buffer.push('p');
			  					VectorParsingState::ScalarUnit
			  				},
			  				 _ => return Err(TokenizerError::UnknownToken(s.to_string()))
			  			},
			  			VectorParsingState::ScalarUnit => {
			  				if c == '\n' {
			  					buffer.pop().unwrap();
			  					// unit = buffer.clone();
			  					// buffer.clear();
			  					VectorParsingState::Done
			  				} else {
			  					VectorParsingState::ScalarUnit
			  				}
			  			},
			  			VectorParsingState::Done => return Err(TokenizerError::UnknownToken(s.to_string()))
			  		}
			  	}

			  	if let VectorParsingState::Done = state {
			  		if parsed.0 {
			  			Ok(Self::Vector (parsed.1, parsed.2))
			  		} else {
			  			Ok(Self::Scalar (
			  				parsed.1,
			  				match &buffer[..] {
			  					"px" => Unit::Pixel,
			  					""   => Unit::Default,
			  					_ =>  return Err(TokenizerError::UnknownUnit(buffer))
			  				}
			  			))
			  		}
			  	} else {
			  		Err(TokenizerError::UnexpectedEOF)
			  	}
			}
		}
	}
}

pub struct Tokenizer;
impl Tokenizer {

	pub async fn tokenize(source: &str, rm: &mut ResourceManager) -> Result::<Vec::<Token>, TokenizerError> {
		let mut r = Vec::<Token>::new();

		// Removing blank characters in litterals
		let mut s = String::new();
		let mut level = 0;
		for c in source.chars() {
			if c == ')' { level -= 1 }
			if c == '(' { level += 1 }
			if level != 0 && c != ' ' || level == 0 {
				s.push(c);
			}
		}

		// Splitting words and removing commentaries
		let mut binding = s
			.split("\n")
			.filter(|x| x.len() > 0 && &x[0..1] != "#")
			.map(|x| x.split(" "))
			.flatten()
			.collect::<Vec::<&str>>();

		// Removing empty words
		binding = binding
			.into_iter()
			.filter(|x| x.len() != 0)
			.collect();

		// Simplifying
		binding = Self::collapse(binding, &["with", "initial", "speed", "of"], "wiso");
		binding = Self::collapse(binding, &["of", "size"], "ofsize");

		// Translating in tokens
		for word in binding {
			r.push(Token::from(word, rm).await?);
		}

		r.push(Token::EndOfFile);

		Ok(r)
	}

	/// Removes any target sequence and put replacement instead in source
	fn collapse<'a>(source: Vec<&'a str>, target: &[&str], replacement: &'a str) -> Vec::<&'a str> {
		let mut r = Vec::<&str>::new();
		let mut i = 0;
		while i < source.len() {
			if i < source.len() - target.len() {
				let mut eq = true;
				for j in i..(i+target.len()) {
					if source[j] != target[j-i] {
						eq = false;
					}
				}
				if eq {
					r.push(replacement);
					i += target.len() - 1;
				} else {
					r.push(source[i]);
				}
			} else {
				r.push(source[i]);
			}
			i+=1;
		}

		r
	}

}