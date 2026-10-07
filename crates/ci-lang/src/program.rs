use ci_value::Value;

use crate::column::Column;
use crate::literal::literal;

/// J: the result's type (2 is characters), shape and ravel.
const J_SHOW: &str = "echo (>(2=3!:0 r){(<,'n'),<,'c'),'|',(\":$r),'|',\":,r";

/// BQN: F joins numbers with spaces; •Type 2 is a character.
const BQN_DEFS: &str = "F←{1↓∾(\" \"∾•Repr)¨𝕩}";
const BQN_SHOW: &str = "c←2=•Type⊑⥊r\n•Out (c⊑\"nc\")∾\"|\"∾(F≢r)∾\"|\"∾c◶⟨F,⊢⟩⥊r";

/// GNU APL: a blank prototype means characters.
const APL_SHOW: &str = "('nc'[1+' '≡↑0⍴R]),'|',(⍕⍴R),'|',⍕,R";

/// ngn/k: no shape primitive, so a general list of lists is taken as a
/// matrix (rows, then the first row's length) and an atom has no axes.
const K_SHOW: &str = r##"c:|/(@r)=`c`C
s:$[`A~@r;(#r),#*r;r~*r;!0;,#r]
`0:,/(("nc"c);"|";" "/$s;"|";$[c;,/r;" "/$(0#0),,/r])"##;

/// Kona: type 0 is a general list, 3 and -3 are characters, positive types
/// are atoms. `0: adds no newline, so the line ends with one: otherwise k's
/// next prompt is glued to the items.
const K3_SHOW: &str = r##"t:4:r
c:t _in 3 -3
s::[0=t;(#r),#*r;0<t;!0;,#r]
`0:,/(("nc"@c);"|";" "/$s;"|";:[c;,/r;" "/$,/r];"\n")"##;

/// The program that binds the inputs, evaluates `expr` and prints the
/// result: as a `kind|shape|items` line where the language can, otherwise in
/// its own display. `binding_text` is the bindings as X_eTaL writes them.
pub fn program(col: Column, binding: &[(String, Value)], binding_text: &str, expr: &str) -> String {
    if col == Column::Xetal {
        return format!("{binding_text}; {expr}");
    }
    let mut lines: Vec<String> = binding
        .iter()
        .map(|(n, v)| assign(col, n, &literal(col, v)))
        .collect();
    let (pre, last) = match col {
        Column::J => ("", format!("r =: {expr}\n{J_SHOW}")),
        Column::Bqn => (BQN_DEFS, format!("r←{expr}\n{BQN_SHOW}")),
        Column::GnuApl => ("", format!("R←{expr}\n{APL_SHOW}")),
        Column::K => ("", format!("r:{expr}\n{K_SHOW}")),
        Column::K3 => ("", format!("r:{expr}\n{K3_SHOW}")),
        _ => ("", expr.to_string()),
    };
    if !pre.is_empty() {
        lines.insert(0, pre.to_string());
    }
    lines.push(last);
    lines.join("\n")
}

/// `name ← value` in the column's language. APL2's cells in X_eTaL's data
/// use upper-case names, so GNU APL's bindings do too.
fn assign(col: Column, name: &str, value: &str) -> String {
    match col {
        Column::GnuApl => format!("{}←{value}", name.to_uppercase()),
        Column::J => format!("{name} =: {value}"),
        Column::Bqn | Column::Uiua => format!("{name} ← {value}"),
        _ => format!("{name}:{value}"),
    }
}
