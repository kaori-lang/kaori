use foldhash::HashMap;

use crate::runtime::value::Value;

struct Closure(Box<[Value]>);
struct Map(HashMap<Value, Value>);
struct Array(Vec<Value>);
