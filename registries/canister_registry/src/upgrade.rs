// Project imports
use crate::registry::{CanisterCategory, CanisterDB, CanisterMetadata, Fleek};

// IC imports
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

// Rust imports
use std::collections::BTreeMap;
use std::iter::FromIterator;

fn get_data() -> BTreeMap<Principal, CanisterCategory> {
    let data: Vec<(&str, CanisterCategory)> = vec![
        ("xkbqi-2qaaa-aaaah-qbpqq-cai", CanisterCategory::NFT),
        ("uzhxd-ziaaa-aaaah-qanaq-cai", CanisterCategory::NFT),
        ("tde7l-3qaaa-aaaah-qansa-cai", CanisterCategory::NFT),
        ("qcg3w-tyaaa-aaaah-qakea-cai", CanisterCategory::NFT),
        ("owuqd-dyaaa-aaaah-qapxq-cai", CanisterCategory::NFT),
        ("nbg4r-saaaa-aaaah-qap7a-cai", CanisterCategory::NFT),
        ("mx7fv-viaaa-aaaah-aarsa-cai", CanisterCategory::NFT),
        ("kss7i-hqaaa-aaaah-qbvmq-cai", CanisterCategory::NFT),
        ("kqre2-2qaaa-aaaad-qamxa-cai", CanisterCategory::NFT),
        ("ryjl3-tyaaa-aaaaa-aaaba-cai", CanisterCategory::NFT),
        ("k4qsa-4aaaa-aaaah-qbvnq-cai", CanisterCategory::NFT),
        ("ja7sy-daaaa-aaaai-qaguq-cai", CanisterCategory::Social),
        ("gevsk-tqaaa-aaaah-qaoca-cai", CanisterCategory::NFT),
        ("e3izy-jiaaa-aaaah-qacbq-cai", CanisterCategory::NFT),
        ("dslea-eiaaa-aaaae-aaa3a-cai", CanisterCategory::Tools),
        ("d3ttm-qaaaa-aaaai-qam4a-cai", CanisterCategory::NFT),
        ("bxdf4-baaaa-aaaah-qaruq-cai", CanisterCategory::NFT),
        ("aanaa-xaaaa-aaaah-aaeiq-cai", CanisterCategory::Token),
        ("73xld-saaaa-aaaah-qbjya-cai", CanisterCategory::NFT),
        ("3db6u-aiaaa-aaaah-qbjbq-cai", CanisterCategory::NFT),
        ("2ji5m-raaaa-aaaah-aanoa-cai", CanisterCategory::Games),
        ("24pmb-qiaaa-aaaah-aannq-cai", CanisterCategory::Games),
        ("zfjzz-4aaaa-aaaah-aasbq-cai", CanisterCategory::NFT),
        ("sygsn-caaaa-aaaaf-qaahq-cai", CanisterCategory::Service),
        ("stz5m-sqaaa-aaaah-qaggq-cai", CanisterCategory::Games),
        ("rl4ub-oqaaa-aaaah-qbi3a-cai", CanisterCategory::Games),
        ("rglue-kyaaa-aaaah-qakca-cai", CanisterCategory::Service),
        ("rdmx6-jaaaa-aaaaa-aaadq-cai", CanisterCategory::Service),
        ("rdbii-uiaaa-aaaab-qadva-cai", CanisterCategory::Games),
        ("nntkg-vqaaa-aaaad-qamfa-cai", CanisterCategory::NFT),
        ("n7ib3-4qaaa-aaaai-qagnq-cai", CanisterCategory::Service),
        ("m7sm4-2iaaa-aaaab-qabra-cai", CanisterCategory::Tools),
        ("lm5fh-ayaaa-aaaah-aafua-cai", CanisterCategory::Games),
        ("ljyte-qiaaa-aaaah-qaiva-cai", CanisterCategory::Service),
        ("lc7ip-3iaaa-aaaah-aafva-cai", CanisterCategory::Games),
        ("l2jyf-nqaaa-aaaah-qadha-cai", CanisterCategory::Games),
        ("ivg37-qiaaa-aaaab-aaaga-cai", CanisterCategory::Games),
        ("i67uk-hiaaa-aaaae-qaaka-cai", CanisterCategory::Tools),
        ("h5aet-waaaa-aaaab-qaamq-cai", CanisterCategory::Social),
        ("grwwk-gaaaa-aaaah-aapma-cai", CanisterCategory::Social),
        ("dvr6e-lqaaa-aaaai-qam5a-cai", CanisterCategory::NFT),
        ("c7fao-laaaa-aaaae-aaa4q-cai", CanisterCategory::Social),
        ("7e6iv-biaaa-aaaaf-aaada-cai", CanisterCategory::Social),
        ("3pbcj-viaaa-aaaah-qaajq-cai", CanisterCategory::Service),
        ("bxhqr-vyaaa-aaaah-aaqza-cai", CanisterCategory::NFT),
        ("5ymop-yyaaa-aaaah-qaa4q-cai", CanisterCategory::Token),
        ("r7inp-6aaaa-aaaaa-aaabq-cai", CanisterCategory::Tools),
        ("ptodj-lqaaa-aaaah-qaeaq-cai", CanisterCategory::Tools),
        ("aaaaa-aa", CanisterCategory::Tools),
        ("rkp4c-7iaaa-aaaaa-aaaca-cai", CanisterCategory::Tools),
        ("ryjl3-tyaaa-aaaaa-aaaba-cai", CanisterCategory::Token),
        ("rno2w-sqaaa-aaaaa-aaacq-cai", CanisterCategory::Tools),
        ("rrkah-fqaaa-aaaaa-aaaaq-cai", CanisterCategory::Tools),
        ("qoctq-giaaa-aaaaa-aaaea-cai", CanisterCategory::Tools),
        ("snpdi-6yaaa-aaaah-aauaq-cai", CanisterCategory::Service),
        ("k4qsa-4aaaa-aaaah-qbvnq-cai", CanisterCategory::NFT),
        ("lhq4n-3yaaa-aaaai-qaniq-cai", CanisterCategory::NFT),
        ("hdxhu-qqaaa-aaaai-aasnq-cai", CanisterCategory::NFT),
        ("wxi2q-oiaaa-aaaaj-qab2q-cai", CanisterCategory::NFT),
        ("njgly-uaaaa-aaaah-qb6pa-cai", CanisterCategory::NFT),
        ("sr4qi-vaaaa-aaaah-qcaaq-cai", CanisterCategory::NFT),
        ("cihkf-qyaaa-aaaah-qb7jq-cai", CanisterCategory::NFT),
        ("q6hjz-kyaaa-aaaah-qcama-cai", CanisterCategory::NFT),
        ("qqfer-riaaa-aaaah-qcana-cai", CanisterCategory::NFT),
        ("r7nfb-aaaaa-aaaaj-qabla-cai", CanisterCategory::Tools),
        ("j2nsf-iqaaa-aaaai-qanha-cai", CanisterCategory::Service),
        ("oeee4-qaaaa-aaaak-qaaeq-cai", CanisterCategory::NFT),
        ("pnpu4-3aaaa-aaaah-qcceq-cai", CanisterCategory::NFT),
        ("bid2t-gyaaa-aaaah-qcdea-cai", CanisterCategory::NFT),
        ("btggw-4aaaa-aaaah-qcdgq-cai", CanisterCategory::NFT),
        ("dv6u3-vqaaa-aaaah-qcdlq-cai", CanisterCategory::NFT),
        ("crt3j-mqaaa-aaaah-qcdnq-cai", CanisterCategory::NFT),
        ("cnxby-3qaaa-aaaah-qcdpq-cai", CanisterCategory::NFT),
        ("ckwhm-wiaaa-aaaah-qcdpa-cai", CanisterCategory::NFT),
        ("cdvmq-aaaaa-aaaah-qcdoq-cai", CanisterCategory::NFT),
    ];

    BTreeMap::from_iter(
        data.into_iter()
            .map(|(k, v)| (Principal::from_text(&k).unwrap(), v)),
    )
}

#[derive(Deserialize, CandidType, Clone, PartialEq, Debug)]
pub struct CanisterMetadataV0 {
    name: String,
    description: String,
    url: String,
    logo_url: String,
    version: u32,
}

impl From<CanisterMetadataV0> for CanisterMetadata {
    fn from(cs: CanisterMetadataV0) -> Self {
        CanisterMetadata {
            name: cs.name,
            description: cs.description,
            url: cs.url,
            logo_url: cs.logo_url,
            category: CanisterCategory::Asset,
            version: cs.version,
        }
    }
}

#[derive(CandidType, Deserialize)]
struct StableStorageV0 {
    db: Vec<(Principal, CanisterMetadataV0)>,
    fleek: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
struct StableStorage {
    db: Vec<(Principal, CanisterMetadata)>,
    fleek: Vec<Principal>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let db = ic::get_mut::<CanisterDB>().archive();
    let fleek = ic::get_mut::<Fleek>().0.clone();

    let stable = StableStorage { db, fleek };

    match ic::stable_store((stable,)) {
        Ok(_) => (),
        Err(candid_err) => {
            trap(&format!(
                "An error occurred when saving to stable memory (pre_upgrade): {:?}",
                candid_err
            ));
        }
    };
}

#[post_upgrade]
pub fn post_upgrade() {
    if let Ok((stable,)) = ic::stable_restore::<(StableStorageV0,)>() {
        let data = get_data();
        let mut canister_list = Vec::with_capacity(stable.db.len());
        for (_key, canister_info) in stable.db.into_iter().enumerate() {
            let mut metadata_info: CanisterMetadata = canister_info.1.into();
            let principal_info: Principal = canister_info.0.into();

            match data.get(&principal_info) {
                Some(cat) => metadata_info.category = cat.clone(),
                None => metadata_info.category = CanisterCategory::Service,
            }

            canister_list.push((principal_info, metadata_info));
        }
        ic::get_mut::<CanisterDB>().load(canister_list);
        ic::store(Fleek(stable.fleek));
    }
}
