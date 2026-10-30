from pathlib import Path
r=Path('outputs/SpiritAshResummon')
p=r/'src/policy.rs'
s=p.read_text();s+='''
#[cfg(test)]
mod multi_tests {
    use super::*;
    #[test]
    fn upgrades_have_one_type_and_reject_ambiguous_maps() {
        let map=[(2000,1),(2000,2),(3000,3)];
        assert_eq!(unique_family(&map,2000),unique_family(&map,2010));
        assert_eq!(unique_family(&map,3010),Some((3000,vec![3])));
        assert_eq!(unique_family(&[(2000,1),(2005,2)],2007),None);
        assert_eq!(unique_family(&[(2000,1),(3000,1)],2000),None);
        assert_eq!(unique_family(&map,2011),None);
    }
    #[test]
    fn each_selected_type_controls_its_action() {
        assert_eq!(multi_action(2,false,true,false,true),Action::AddGroup);
        assert_eq!(multi_action(2,true,true,true,true),Action::Heal);
        assert_eq!(multi_action(2,true,true,false,true),Action::Reject);
        assert_eq!(multi_action(2,true,true,true,false),Action::Reject);
        for state in [-1,3,i32::MAX] {
            assert_eq!(multi_action(state,false,true,true,true),Action::Reject);
        }
        for live in [false,true] {
            assert_eq!(multi_action(2,live,false,true,true),Action::Reject);
        }
    }
}
''';p.write_text(s)
p=r/'config/SpiritAshResummon.ini';p.write_text(p.read_text().replace('[SpiritAsh]','[SpiritAsh]\nAllowMultipleAshTypes = true'))
p=r/'src/runtime.rs';s=p.read_text().replace('or allow a second Ash. The no-group idle path already returns Vanilla.','or allow an unvalidated spawn. The no-group idle path returns Vanilla.');p.write_text(s)
