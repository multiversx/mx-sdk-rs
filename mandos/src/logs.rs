use super::*;

#[derive(Debug)]
pub struct CheckLog {
	pub address: BytesValue,
	pub identifier: BytesValue,
	pub topics: Vec<BytesValue>,
	pub data: BytesValue,
}

impl InterpretableFrom<CheckLogRaw> for CheckLog {
	fn interpret_from(from: CheckLogRaw, context: &InterpreterContext) -> Self {
		CheckLog {
			address: BytesValue::interpret_from(from.address, context),
			identifier: BytesValue::interpret_from(from.identifier, context),
			topics: from
				.topics
				.into_iter()
				.map(|t| BytesValue::interpret_from(t, context))
				.collect(),
			data: BytesValue::interpret_from(from.data, context),
		}
	}
}

#[derive(Debug)]
pub enum CheckLogs {
	Star,
	List(Vec<CheckLog>),
	DefaultStar,
}

impl CheckLogs {
	pub fn is_star(&self) -> bool {
		matches!(self, CheckLogs::Star)
	}
}

impl InterpretableFrom<CheckLogsRaw> for CheckLogs {
	fn interpret_from(from: CheckLogsRaw, context: &InterpreterContext) -> Self {
		match from {
			CheckLogsRaw::Star => CheckLogs::Star,
			CheckLogsRaw::List(l) => CheckLogs::List(
				l.into_iter()
					.map(|c| CheckLog::interpret_from(c, context))
					.collect(),
			),
			CheckLogsRaw::DefaultStar => CheckLogs::DefaultStar,
		}
	}
}
