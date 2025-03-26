use aok::{OK, Void};
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

struct Case {
  原文: &'static str,
  术语: &'static [(&'static str, &'static str)],
  预期: &'static str,
}

const TEST_CASES: &[Case] = &[
  Case {
    原文: "故障告警test __故障__告警。 告警",
    术语: &[("告警", "alter"), ("故障", "incident")],
    预期: "Incident alter test __incident__alter。 Alter",
  },
  Case {
    原文: "test bound : 1-Bo **bo** __bo__ box xbo bo",
    术语: &[("bo", "边界")],
    预期: "test bound : 1-边界 **边界** __边界__ box xbo 边界",
  },
  Case {
    原文: "test case Bo BO",
    术语: &[("bo", "xx")],
    预期: "test case Xx XX",
  },
];

#[test]
fn test() -> Void {
  for case in TEST_CASES {
    let mut from_li = vec![];
    let mut to_li = vec![];
    case.术语.iter().for_each(|(k, v)| {
      from_li.push(k.to_owned());
      to_li.push(v.to_owned());
    });

    let term = tran_term::Term::load(from_li, to_li)?;
    let r = term.replace(case.原文).unwrap();
    info!("\n{}\n{}\n", case.原文, r);
    assert_eq!(r, case.预期);
  }
  OK
}
