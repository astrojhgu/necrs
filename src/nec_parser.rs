use pest::iterators::Pair;

use crate::raw_nec_context::RawNecContext;

#[derive(Parser)]
#[grammar = "nec.pest"]
pub struct NecParser;

pub fn parse_numbers(x: Pair<'_, Rule>) -> (Vec<i32>, Vec<f64>) {
    let mut ilist = Vec::new();
    let mut flist = Vec::new();
    for r in x.into_inner() {
        match r.as_rule() {
            Rule::int_num => ilist.push(r.as_str().parse::<i32>().unwrap()),
            Rule::floating_num => flist.push(r.as_str().parse::<f64>().unwrap()),
            _ => {}
        }
    }
    (ilist, flist)
}

pub fn parse_nec_file(pair: Pair<'_, Rule>) -> RawNecContext {
    let mut context = RawNecContext::new();
    for nec_item in pair.into_inner() {
        match nec_item.as_rule() {
            Rule::Geom => {
                for geom_item in nec_item.into_inner() {
                    match geom_item.as_rule() {
                        Rule::Wire => {
                            for wire_item in geom_item.into_inner() {
                                let (ilist, flist) = parse_numbers(wire_item.clone());
                                match wire_item.as_rule() {
                                    Rule::Gw => {
                                        assert_eq!(ilist.len(), 2);
                                        assert_eq!(flist.len(), 7);
                                        context.nec_wire(
                                            ilist[0], ilist[1], flist[0], flist[1], flist[2],
                                            flist[3], flist[4], flist[5], flist[6], 1.0, 1.0,
                                        );
                                    }
                                    Rule::Gc => {
                                        unimplemented!();
                                    }
                                    _ => {
                                        unreachable!()
                                    }
                                }
                            }
                        }
                        Rule::Gm => {
                            let (ilist, flist) = parse_numbers(geom_item.clone());
                            assert_eq!(ilist.len(), 2);
                            assert!(flist.len() >= 6);
                            context.nec_gm_card(
                                ilist[0],
                                ilist[1],
                                flist[0],
                                flist[1],
                                flist[2],
                                flist[3],
                                flist[4],
                                flist[5],
                                if flist.len() == 6 { 0 } else { flist[6] as i32 },
                            );
                        }
                        Rule::Ge => {
                            let (ilist, flist) = parse_numbers(geom_item.clone());
                            assert_eq!(ilist.len(), 1);
                            assert_eq!(flist.len(), 0);
                            context.nec_geometry_complete(ilist[0]);
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
            }
            Rule::Gn => {
                let (ilist, mut flist) = parse_numbers(nec_item.clone());
                while flist.len() < 6 {
                    flist.push(0.0);
                    flist.push(0.0);
                }
                assert_eq!(ilist.len(), 2);
                context.nec_gn_card(
                    ilist[0], ilist[1], flist[0], flist[1], flist[2], flist[3], flist[4], flist[5],
                );
            }
            Rule::Ek => {
                let (mut ilist, flist) = parse_numbers(nec_item.clone());
                assert_eq!(flist.len(), 0);
                assert!(ilist.len() <= 1);
                while ilist.len() < 1 {
                    ilist.push(0);
                }
                context.nec_ek_card(ilist[0]);
            }
            Rule::Tl => {
                let (ilist, flist) = parse_numbers(nec_item.clone());
                assert_eq!(ilist.len(), 4);
                assert_eq!(flist.len(), 6);
                context.nec_tl_card(
                    ilist[0], ilist[1], ilist[2], ilist[3], flist[0], flist[1], flist[2], flist[3],
                    flist[4], flist[5],
                );
            }
            Rule::Ex => {
                let (ilist, mut flist) = parse_numbers(nec_item.clone());
                assert_eq!(ilist.len(), 4);
                assert!(flist.len() >= 2);
                while flist.len() < 6 {
                    flist.push(0.0);
                }
                context.nec_ex_card(
                    ilist[0], ilist[1], ilist[2], ilist[3], flist[0], flist[1], flist[2], flist[3],
                    flist[4], flist[5],
                );
            }
            _ => {}
        }
    }
    context
}

/*
#[pest_consume::parser]
impl NecParser{
    fn int_num(input: Node)->Result<i32>{
        input.as_str()
        .parse::<i32>()
        .map_err(|e| input.error(e))
    }

    fn floating_num(input:Node)->Result<f64>{
        input.as_str()
        .parse::<f64>()
        .map_err(|e| input.error(e))
    }

    fn Wire(input: Node)->Result<Wire>{
        let c=input.into_children();
        let gw=c.next().unwrap().as_rule().unwrap();
        let gw_elements=gw.into_inner()
    }

}
*/
