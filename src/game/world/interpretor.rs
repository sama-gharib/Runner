//! Syntaxic analyzer

use macroquad::prelude::*;
use super::tokenizer::{Unit, Token};
use super::World;
use super::super::object::Object;
use super::super::object::ObjectKind;


/// States of the parsing automaton
enum InterpretorState {
	Initial,
	UnitDeclaration,
	UnitDefinition,
	ObjectDeclaration (Object),
	PositionDefinition (Object),
	InitialSpeedDefinition (Object),
	SizeDefinition (Object),
	Done
}

#[derive(Debug)]
pub struct InterpretorError {
	unexpected: Token,
	expected: Vec::<Token>
}

pub struct Interpretor;
impl Interpretor {
	/// TODO: Replace `Vec::<Token>` with an always valid type.
	/// i.e.: That type should be only created by Tokenizer::tokenize 
	pub fn interpret(tokens: Vec::<Token>) -> Result<World, InterpretorError> {
		let mut r = World::new();
		let mut unit = Vec2::new(1., 1.);
		let mut state = InterpretorState::Initial;
		
		for t in tokens {

			// Complete, deterministic, simple automaton.
			state = match state {
				InterpretorState::Initial => match t {
					Token::Unit => InterpretorState::UnitDeclaration,
					Token::Kind(k) => InterpretorState::ObjectDeclaration (Object::from(k)),
					_ => return Err(InterpretorError {
						unexpected: t,
						expected: vec![
							Token::Unit,
							Token::Kind(ObjectKind::Spike),
							Token::Kind(ObjectKind::Wall)
						]
					})
				},
				InterpretorState::UnitDeclaration => match t {
					Token::Is => InterpretorState::UnitDefinition,
					_ => return Err(InterpretorError {
						unexpected: t,
						expected: vec![Token::Is]
					})
				},
				InterpretorState::UnitDefinition => match t {
					Token::Vector(x, y) => {
						unit.x = x as f32;
						unit.y = y as f32;
						InterpretorState::Initial
					},
					_ => return Err(InterpretorError {
						unexpected: t,
						expected: vec![Token::Is]
					})
				},
				InterpretorState::ObjectDeclaration(obj) => match t {
					Token::At => InterpretorState::PositionDefinition(obj.size(unit)),
					Token::OfSize => InterpretorState::SizeDefinition(obj.size(unit)),
					Token::WithIS => InterpretorState::InitialSpeedDefinition(obj),
					Token::Kind(k) => {
						r.add_object(obj);
						InterpretorState::ObjectDeclaration(Object::from(k))
					},
					Token::EndOfFile => {
						r.add_object(obj);
						InterpretorState::Done
					}
					_ => return Err(InterpretorError {
						unexpected: t,
						expected: vec![
							Token::At,
							Token::OfSize,
							Token::WithIS
						]
					})
				},
				InterpretorState::PositionDefinition(obj) => match t {
					Token::Vector(x, y) => InterpretorState::ObjectDeclaration(obj.position(Vec2::new(x as f32 * unit.x, y as f32 * unit.y))),
					_ => return Err(InterpretorError {
						unexpected: t,
						expected: vec![
							Token::Vector(0, 0)
						]
					}) 
				},
				InterpretorState::SizeDefinition(obj) => match t {
					Token::Vector(x, y) => InterpretorState::ObjectDeclaration(obj.size(Vec2::new(x as f32 * unit.x, y as f32 * unit.y))),
					_ => return Err(InterpretorError {
						unexpected: t,
						expected: vec![
							Token::Vector(0, 0)
						]
					}) 
				},
				InterpretorState::InitialSpeedDefinition(obj) => match t {
					Token::Scalar(x, u) => InterpretorState::ObjectDeclaration(obj.speed(Vec2::new(x as f32 * if let Unit::Default = u{ unit.x } else {1.}, 0.))),
					_ => return Err(InterpretorError {
						unexpected: t,
						expected: vec![
							Token::Scalar(0, Unit::Default),
							Token::Scalar(0, Unit::Pixel)
						]
					}) 
				},
				InterpretorState::Done => return Err(InterpretorError {unexpected: t, expected: vec![Token::EndOfFile]})
			};
		}

		Ok(r)
	}
}