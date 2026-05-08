// AUTO-GENERATED. Source: schema/netbox-4.5.10.json :: WritableInterfaceRequest
// Run `cargo run -p nbx-codegen -- schema/netbox-4.5.10.json src/generated/resources/` to regenerate.

use clap::Args;
use clap::ValueEnum;
use serde_json::{Map, Value, json};

use crate::commands::{GlobalOptions, ResourceSpec};
use crate::commands::insert_optional_bool_field;
use crate::commands::tags_value;
use crate::commands::insert_optional_string_field;
use crate::error::NbxResult;

pub const RESOURCE: ResourceSpec = ResourceSpec {
    app: "dcim",
    name: "interfaces",
    api_path: "/api/dcim/interfaces/",
    detail_path: "/api/dcim/interfaces/{id}/",
    default_lookup_field: "name",
};

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InterfaceDuplex {
    Half,
    Full,
    Auto,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InterfaceMode {
    Access,
    Tagged,
    #[serde(rename = "tagged-all")]
    #[value(name = "tagged-all")]
    TaggedAll,
    #[serde(rename = "q-in-q")]
    #[value(name = "q-in-q")]
    QInQ,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InterfacePoeMode {
    Pd,
    Pse,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InterfacePoeType {
    #[serde(rename = "type1-ieee802.3af")]
    #[value(name = "type1-ieee802.3af")]
    Type1Ieee8023af,
    #[serde(rename = "type2-ieee802.3at")]
    #[value(name = "type2-ieee802.3at")]
    Type2Ieee8023at,
    #[serde(rename = "type3-ieee802.3bt")]
    #[value(name = "type3-ieee802.3bt")]
    Type3Ieee8023bt,
    #[serde(rename = "type4-ieee802.3bt")]
    #[value(name = "type4-ieee802.3bt")]
    Type4Ieee8023bt,
    #[serde(rename = "passive-24v-2pair")]
    #[value(name = "passive-24v-2pair")]
    Passive24v2pair,
    #[serde(rename = "passive-24v-4pair")]
    #[value(name = "passive-24v-4pair")]
    Passive24v4pair,
    #[serde(rename = "passive-48v-2pair")]
    #[value(name = "passive-48v-2pair")]
    Passive48v2pair,
    #[serde(rename = "passive-48v-4pair")]
    #[value(name = "passive-48v-4pair")]
    Passive48v4pair,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InterfaceRfChannel {
    #[serde(rename = "2.4g-1-2412-22")]
    #[value(name = "2.4g-1-2412-22")]
    Two4g1241222,
    #[serde(rename = "2.4g-2-2417-22")]
    #[value(name = "2.4g-2-2417-22")]
    Two4g2241722,
    #[serde(rename = "2.4g-3-2422-22")]
    #[value(name = "2.4g-3-2422-22")]
    Two4g3242222,
    #[serde(rename = "2.4g-4-2427-22")]
    #[value(name = "2.4g-4-2427-22")]
    Two4g4242722,
    #[serde(rename = "2.4g-5-2432-22")]
    #[value(name = "2.4g-5-2432-22")]
    Two4g5243222,
    #[serde(rename = "2.4g-6-2437-22")]
    #[value(name = "2.4g-6-2437-22")]
    Two4g6243722,
    #[serde(rename = "2.4g-7-2442-22")]
    #[value(name = "2.4g-7-2442-22")]
    Two4g7244222,
    #[serde(rename = "2.4g-8-2447-22")]
    #[value(name = "2.4g-8-2447-22")]
    Two4g8244722,
    #[serde(rename = "2.4g-9-2452-22")]
    #[value(name = "2.4g-9-2452-22")]
    Two4g9245222,
    #[serde(rename = "2.4g-10-2457-22")]
    #[value(name = "2.4g-10-2457-22")]
    Two4g10245722,
    #[serde(rename = "2.4g-11-2462-22")]
    #[value(name = "2.4g-11-2462-22")]
    Two4g11246222,
    #[serde(rename = "2.4g-12-2467-22")]
    #[value(name = "2.4g-12-2467-22")]
    Two4g12246722,
    #[serde(rename = "2.4g-13-2472-22")]
    #[value(name = "2.4g-13-2472-22")]
    Two4g13247222,
    #[serde(rename = "5g-32-5160-20")]
    #[value(name = "5g-32-5160-20")]
    Fiveg32516020,
    #[serde(rename = "5g-34-5170-40")]
    #[value(name = "5g-34-5170-40")]
    Fiveg34517040,
    #[serde(rename = "5g-36-5180-20")]
    #[value(name = "5g-36-5180-20")]
    Fiveg36518020,
    #[serde(rename = "5g-38-5190-40")]
    #[value(name = "5g-38-5190-40")]
    Fiveg38519040,
    #[serde(rename = "5g-40-5200-20")]
    #[value(name = "5g-40-5200-20")]
    Fiveg40520020,
    #[serde(rename = "5g-42-5210-80")]
    #[value(name = "5g-42-5210-80")]
    Fiveg42521080,
    #[serde(rename = "5g-44-5220-20")]
    #[value(name = "5g-44-5220-20")]
    Fiveg44522020,
    #[serde(rename = "5g-46-5230-40")]
    #[value(name = "5g-46-5230-40")]
    Fiveg46523040,
    #[serde(rename = "5g-48-5240-20")]
    #[value(name = "5g-48-5240-20")]
    Fiveg48524020,
    #[serde(rename = "5g-50-5250-160")]
    #[value(name = "5g-50-5250-160")]
    Fiveg505250160,
    #[serde(rename = "5g-52-5260-20")]
    #[value(name = "5g-52-5260-20")]
    Fiveg52526020,
    #[serde(rename = "5g-54-5270-40")]
    #[value(name = "5g-54-5270-40")]
    Fiveg54527040,
    #[serde(rename = "5g-56-5280-20")]
    #[value(name = "5g-56-5280-20")]
    Fiveg56528020,
    #[serde(rename = "5g-58-5290-80")]
    #[value(name = "5g-58-5290-80")]
    Fiveg58529080,
    #[serde(rename = "5g-60-5300-20")]
    #[value(name = "5g-60-5300-20")]
    Fiveg60530020,
    #[serde(rename = "5g-62-5310-40")]
    #[value(name = "5g-62-5310-40")]
    Fiveg62531040,
    #[serde(rename = "5g-64-5320-20")]
    #[value(name = "5g-64-5320-20")]
    Fiveg64532020,
    #[serde(rename = "5g-100-5500-20")]
    #[value(name = "5g-100-5500-20")]
    Fiveg100550020,
    #[serde(rename = "5g-102-5510-40")]
    #[value(name = "5g-102-5510-40")]
    Fiveg102551040,
    #[serde(rename = "5g-104-5520-20")]
    #[value(name = "5g-104-5520-20")]
    Fiveg104552020,
    #[serde(rename = "5g-106-5530-80")]
    #[value(name = "5g-106-5530-80")]
    Fiveg106553080,
    #[serde(rename = "5g-108-5540-20")]
    #[value(name = "5g-108-5540-20")]
    Fiveg108554020,
    #[serde(rename = "5g-110-5550-40")]
    #[value(name = "5g-110-5550-40")]
    Fiveg110555040,
    #[serde(rename = "5g-112-5560-20")]
    #[value(name = "5g-112-5560-20")]
    Fiveg112556020,
    #[serde(rename = "5g-114-5570-160")]
    #[value(name = "5g-114-5570-160")]
    Fiveg1145570160,
    #[serde(rename = "5g-116-5580-20")]
    #[value(name = "5g-116-5580-20")]
    Fiveg116558020,
    #[serde(rename = "5g-118-5590-40")]
    #[value(name = "5g-118-5590-40")]
    Fiveg118559040,
    #[serde(rename = "5g-120-5600-20")]
    #[value(name = "5g-120-5600-20")]
    Fiveg120560020,
    #[serde(rename = "5g-122-5610-80")]
    #[value(name = "5g-122-5610-80")]
    Fiveg122561080,
    #[serde(rename = "5g-124-5620-20")]
    #[value(name = "5g-124-5620-20")]
    Fiveg124562020,
    #[serde(rename = "5g-126-5630-40")]
    #[value(name = "5g-126-5630-40")]
    Fiveg126563040,
    #[serde(rename = "5g-128-5640-20")]
    #[value(name = "5g-128-5640-20")]
    Fiveg128564020,
    #[serde(rename = "5g-132-5660-20")]
    #[value(name = "5g-132-5660-20")]
    Fiveg132566020,
    #[serde(rename = "5g-134-5670-40")]
    #[value(name = "5g-134-5670-40")]
    Fiveg134567040,
    #[serde(rename = "5g-136-5680-20")]
    #[value(name = "5g-136-5680-20")]
    Fiveg136568020,
    #[serde(rename = "5g-138-5690-80")]
    #[value(name = "5g-138-5690-80")]
    Fiveg138569080,
    #[serde(rename = "5g-140-5700-20")]
    #[value(name = "5g-140-5700-20")]
    Fiveg140570020,
    #[serde(rename = "5g-142-5710-40")]
    #[value(name = "5g-142-5710-40")]
    Fiveg142571040,
    #[serde(rename = "5g-144-5720-20")]
    #[value(name = "5g-144-5720-20")]
    Fiveg144572020,
    #[serde(rename = "5g-149-5745-20")]
    #[value(name = "5g-149-5745-20")]
    Fiveg149574520,
    #[serde(rename = "5g-151-5755-40")]
    #[value(name = "5g-151-5755-40")]
    Fiveg151575540,
    #[serde(rename = "5g-153-5765-20")]
    #[value(name = "5g-153-5765-20")]
    Fiveg153576520,
    #[serde(rename = "5g-155-5775-80")]
    #[value(name = "5g-155-5775-80")]
    Fiveg155577580,
    #[serde(rename = "5g-157-5785-20")]
    #[value(name = "5g-157-5785-20")]
    Fiveg157578520,
    #[serde(rename = "5g-159-5795-40")]
    #[value(name = "5g-159-5795-40")]
    Fiveg159579540,
    #[serde(rename = "5g-161-5805-20")]
    #[value(name = "5g-161-5805-20")]
    Fiveg161580520,
    #[serde(rename = "5g-163-5815-160")]
    #[value(name = "5g-163-5815-160")]
    Fiveg1635815160,
    #[serde(rename = "5g-165-5825-20")]
    #[value(name = "5g-165-5825-20")]
    Fiveg165582520,
    #[serde(rename = "5g-167-5835-40")]
    #[value(name = "5g-167-5835-40")]
    Fiveg167583540,
    #[serde(rename = "5g-169-5845-20")]
    #[value(name = "5g-169-5845-20")]
    Fiveg169584520,
    #[serde(rename = "5g-171-5855-80")]
    #[value(name = "5g-171-5855-80")]
    Fiveg171585580,
    #[serde(rename = "5g-173-5865-20")]
    #[value(name = "5g-173-5865-20")]
    Fiveg173586520,
    #[serde(rename = "5g-175-5875-40")]
    #[value(name = "5g-175-5875-40")]
    Fiveg175587540,
    #[serde(rename = "5g-177-5885-20")]
    #[value(name = "5g-177-5885-20")]
    Fiveg177588520,
    #[serde(rename = "6g-1-5955-20")]
    #[value(name = "6g-1-5955-20")]
    Sixg1595520,
    #[serde(rename = "6g-3-5965-40")]
    #[value(name = "6g-3-5965-40")]
    Sixg3596540,
    #[serde(rename = "6g-5-5975-20")]
    #[value(name = "6g-5-5975-20")]
    Sixg5597520,
    #[serde(rename = "6g-7-5985-80")]
    #[value(name = "6g-7-5985-80")]
    Sixg7598580,
    #[serde(rename = "6g-9-5995-20")]
    #[value(name = "6g-9-5995-20")]
    Sixg9599520,
    #[serde(rename = "6g-11-6005-40")]
    #[value(name = "6g-11-6005-40")]
    Sixg11600540,
    #[serde(rename = "6g-13-6015-20")]
    #[value(name = "6g-13-6015-20")]
    Sixg13601520,
    #[serde(rename = "6g-15-6025-160")]
    #[value(name = "6g-15-6025-160")]
    Sixg156025160,
    #[serde(rename = "6g-17-6035-20")]
    #[value(name = "6g-17-6035-20")]
    Sixg17603520,
    #[serde(rename = "6g-19-6045-40")]
    #[value(name = "6g-19-6045-40")]
    Sixg19604540,
    #[serde(rename = "6g-21-6055-20")]
    #[value(name = "6g-21-6055-20")]
    Sixg21605520,
    #[serde(rename = "6g-23-6065-80")]
    #[value(name = "6g-23-6065-80")]
    Sixg23606580,
    #[serde(rename = "6g-25-6075-20")]
    #[value(name = "6g-25-6075-20")]
    Sixg25607520,
    #[serde(rename = "6g-27-6085-40")]
    #[value(name = "6g-27-6085-40")]
    Sixg27608540,
    #[serde(rename = "6g-29-6095-20")]
    #[value(name = "6g-29-6095-20")]
    Sixg29609520,
    #[serde(rename = "6g-31-6105-320")]
    #[value(name = "6g-31-6105-320")]
    Sixg316105320,
    #[serde(rename = "6g-33-6115-20")]
    #[value(name = "6g-33-6115-20")]
    Sixg33611520,
    #[serde(rename = "6g-35-6125-40")]
    #[value(name = "6g-35-6125-40")]
    Sixg35612540,
    #[serde(rename = "6g-37-6135-20")]
    #[value(name = "6g-37-6135-20")]
    Sixg37613520,
    #[serde(rename = "6g-39-6145-80")]
    #[value(name = "6g-39-6145-80")]
    Sixg39614580,
    #[serde(rename = "6g-41-6155-20")]
    #[value(name = "6g-41-6155-20")]
    Sixg41615520,
    #[serde(rename = "6g-43-6165-40")]
    #[value(name = "6g-43-6165-40")]
    Sixg43616540,
    #[serde(rename = "6g-45-6175-20")]
    #[value(name = "6g-45-6175-20")]
    Sixg45617520,
    #[serde(rename = "6g-47-6185-160")]
    #[value(name = "6g-47-6185-160")]
    Sixg476185160,
    #[serde(rename = "6g-49-6195-20")]
    #[value(name = "6g-49-6195-20")]
    Sixg49619520,
    #[serde(rename = "6g-51-6205-40")]
    #[value(name = "6g-51-6205-40")]
    Sixg51620540,
    #[serde(rename = "6g-53-6215-20")]
    #[value(name = "6g-53-6215-20")]
    Sixg53621520,
    #[serde(rename = "6g-55-6225-80")]
    #[value(name = "6g-55-6225-80")]
    Sixg55622580,
    #[serde(rename = "6g-57-6235-20")]
    #[value(name = "6g-57-6235-20")]
    Sixg57623520,
    #[serde(rename = "6g-59-6245-40")]
    #[value(name = "6g-59-6245-40")]
    Sixg59624540,
    #[serde(rename = "6g-61-6255-20")]
    #[value(name = "6g-61-6255-20")]
    Sixg61625520,
    #[serde(rename = "6g-65-6275-20")]
    #[value(name = "6g-65-6275-20")]
    Sixg65627520,
    #[serde(rename = "6g-67-6285-40")]
    #[value(name = "6g-67-6285-40")]
    Sixg67628540,
    #[serde(rename = "6g-69-6295-20")]
    #[value(name = "6g-69-6295-20")]
    Sixg69629520,
    #[serde(rename = "6g-71-6305-80")]
    #[value(name = "6g-71-6305-80")]
    Sixg71630580,
    #[serde(rename = "6g-73-6315-20")]
    #[value(name = "6g-73-6315-20")]
    Sixg73631520,
    #[serde(rename = "6g-75-6325-40")]
    #[value(name = "6g-75-6325-40")]
    Sixg75632540,
    #[serde(rename = "6g-77-6335-20")]
    #[value(name = "6g-77-6335-20")]
    Sixg77633520,
    #[serde(rename = "6g-79-6345-160")]
    #[value(name = "6g-79-6345-160")]
    Sixg796345160,
    #[serde(rename = "6g-81-6355-20")]
    #[value(name = "6g-81-6355-20")]
    Sixg81635520,
    #[serde(rename = "6g-83-6365-40")]
    #[value(name = "6g-83-6365-40")]
    Sixg83636540,
    #[serde(rename = "6g-85-6375-20")]
    #[value(name = "6g-85-6375-20")]
    Sixg85637520,
    #[serde(rename = "6g-87-6385-80")]
    #[value(name = "6g-87-6385-80")]
    Sixg87638580,
    #[serde(rename = "6g-89-6395-20")]
    #[value(name = "6g-89-6395-20")]
    Sixg89639520,
    #[serde(rename = "6g-91-6405-40")]
    #[value(name = "6g-91-6405-40")]
    Sixg91640540,
    #[serde(rename = "6g-93-6415-20")]
    #[value(name = "6g-93-6415-20")]
    Sixg93641520,
    #[serde(rename = "6g-95-6425-320")]
    #[value(name = "6g-95-6425-320")]
    Sixg956425320,
    #[serde(rename = "6g-97-6435-20")]
    #[value(name = "6g-97-6435-20")]
    Sixg97643520,
    #[serde(rename = "6g-99-6445-40")]
    #[value(name = "6g-99-6445-40")]
    Sixg99644540,
    #[serde(rename = "6g-101-6455-20")]
    #[value(name = "6g-101-6455-20")]
    Sixg101645520,
    #[serde(rename = "6g-103-6465-80")]
    #[value(name = "6g-103-6465-80")]
    Sixg103646580,
    #[serde(rename = "6g-105-6475-20")]
    #[value(name = "6g-105-6475-20")]
    Sixg105647520,
    #[serde(rename = "6g-107-6485-40")]
    #[value(name = "6g-107-6485-40")]
    Sixg107648540,
    #[serde(rename = "6g-109-6495-20")]
    #[value(name = "6g-109-6495-20")]
    Sixg109649520,
    #[serde(rename = "6g-111-6505-160")]
    #[value(name = "6g-111-6505-160")]
    Sixg1116505160,
    #[serde(rename = "6g-113-6515-20")]
    #[value(name = "6g-113-6515-20")]
    Sixg113651520,
    #[serde(rename = "6g-115-6525-40")]
    #[value(name = "6g-115-6525-40")]
    Sixg115652540,
    #[serde(rename = "6g-117-6535-20")]
    #[value(name = "6g-117-6535-20")]
    Sixg117653520,
    #[serde(rename = "6g-119-6545-80")]
    #[value(name = "6g-119-6545-80")]
    Sixg119654580,
    #[serde(rename = "6g-121-6555-20")]
    #[value(name = "6g-121-6555-20")]
    Sixg121655520,
    #[serde(rename = "6g-123-6565-40")]
    #[value(name = "6g-123-6565-40")]
    Sixg123656540,
    #[serde(rename = "6g-125-6575-20")]
    #[value(name = "6g-125-6575-20")]
    Sixg125657520,
    #[serde(rename = "6g-129-6595-20")]
    #[value(name = "6g-129-6595-20")]
    Sixg129659520,
    #[serde(rename = "6g-131-6605-40")]
    #[value(name = "6g-131-6605-40")]
    Sixg131660540,
    #[serde(rename = "6g-133-6615-20")]
    #[value(name = "6g-133-6615-20")]
    Sixg133661520,
    #[serde(rename = "6g-135-6625-80")]
    #[value(name = "6g-135-6625-80")]
    Sixg135662580,
    #[serde(rename = "6g-137-6635-20")]
    #[value(name = "6g-137-6635-20")]
    Sixg137663520,
    #[serde(rename = "6g-139-6645-40")]
    #[value(name = "6g-139-6645-40")]
    Sixg139664540,
    #[serde(rename = "6g-141-6655-20")]
    #[value(name = "6g-141-6655-20")]
    Sixg141665520,
    #[serde(rename = "6g-143-6665-160")]
    #[value(name = "6g-143-6665-160")]
    Sixg1436665160,
    #[serde(rename = "6g-145-6675-20")]
    #[value(name = "6g-145-6675-20")]
    Sixg145667520,
    #[serde(rename = "6g-147-6685-40")]
    #[value(name = "6g-147-6685-40")]
    Sixg147668540,
    #[serde(rename = "6g-149-6695-20")]
    #[value(name = "6g-149-6695-20")]
    Sixg149669520,
    #[serde(rename = "6g-151-6705-80")]
    #[value(name = "6g-151-6705-80")]
    Sixg151670580,
    #[serde(rename = "6g-153-6715-20")]
    #[value(name = "6g-153-6715-20")]
    Sixg153671520,
    #[serde(rename = "6g-155-6725-40")]
    #[value(name = "6g-155-6725-40")]
    Sixg155672540,
    #[serde(rename = "6g-157-6735-20")]
    #[value(name = "6g-157-6735-20")]
    Sixg157673520,
    #[serde(rename = "6g-159-6745-320")]
    #[value(name = "6g-159-6745-320")]
    Sixg1596745320,
    #[serde(rename = "6g-161-6755-20")]
    #[value(name = "6g-161-6755-20")]
    Sixg161675520,
    #[serde(rename = "6g-163-6765-40")]
    #[value(name = "6g-163-6765-40")]
    Sixg163676540,
    #[serde(rename = "6g-165-6775-20")]
    #[value(name = "6g-165-6775-20")]
    Sixg165677520,
    #[serde(rename = "6g-167-6785-80")]
    #[value(name = "6g-167-6785-80")]
    Sixg167678580,
    #[serde(rename = "6g-169-6795-20")]
    #[value(name = "6g-169-6795-20")]
    Sixg169679520,
    #[serde(rename = "6g-171-6805-40")]
    #[value(name = "6g-171-6805-40")]
    Sixg171680540,
    #[serde(rename = "6g-173-6815-20")]
    #[value(name = "6g-173-6815-20")]
    Sixg173681520,
    #[serde(rename = "6g-175-6825-160")]
    #[value(name = "6g-175-6825-160")]
    Sixg1756825160,
    #[serde(rename = "6g-177-6835-20")]
    #[value(name = "6g-177-6835-20")]
    Sixg177683520,
    #[serde(rename = "6g-179-6845-40")]
    #[value(name = "6g-179-6845-40")]
    Sixg179684540,
    #[serde(rename = "6g-181-6855-20")]
    #[value(name = "6g-181-6855-20")]
    Sixg181685520,
    #[serde(rename = "6g-183-6865-80")]
    #[value(name = "6g-183-6865-80")]
    Sixg183686580,
    #[serde(rename = "6g-185-6875-20")]
    #[value(name = "6g-185-6875-20")]
    Sixg185687520,
    #[serde(rename = "6g-187-6885-40")]
    #[value(name = "6g-187-6885-40")]
    Sixg187688540,
    #[serde(rename = "6g-189-6895-20")]
    #[value(name = "6g-189-6895-20")]
    Sixg189689520,
    #[serde(rename = "6g-193-6915-20")]
    #[value(name = "6g-193-6915-20")]
    Sixg193691520,
    #[serde(rename = "6g-195-6925-40")]
    #[value(name = "6g-195-6925-40")]
    Sixg195692540,
    #[serde(rename = "6g-197-6935-20")]
    #[value(name = "6g-197-6935-20")]
    Sixg197693520,
    #[serde(rename = "6g-199-6945-80")]
    #[value(name = "6g-199-6945-80")]
    Sixg199694580,
    #[serde(rename = "6g-201-6955-20")]
    #[value(name = "6g-201-6955-20")]
    Sixg201695520,
    #[serde(rename = "6g-203-6965-40")]
    #[value(name = "6g-203-6965-40")]
    Sixg203696540,
    #[serde(rename = "6g-205-6975-20")]
    #[value(name = "6g-205-6975-20")]
    Sixg205697520,
    #[serde(rename = "6g-207-6985-160")]
    #[value(name = "6g-207-6985-160")]
    Sixg2076985160,
    #[serde(rename = "6g-209-6995-20")]
    #[value(name = "6g-209-6995-20")]
    Sixg209699520,
    #[serde(rename = "6g-211-7005-40")]
    #[value(name = "6g-211-7005-40")]
    Sixg211700540,
    #[serde(rename = "6g-213-7015-20")]
    #[value(name = "6g-213-7015-20")]
    Sixg213701520,
    #[serde(rename = "6g-215-7025-80")]
    #[value(name = "6g-215-7025-80")]
    Sixg215702580,
    #[serde(rename = "6g-217-7035-20")]
    #[value(name = "6g-217-7035-20")]
    Sixg217703520,
    #[serde(rename = "6g-219-7045-40")]
    #[value(name = "6g-219-7045-40")]
    Sixg219704540,
    #[serde(rename = "6g-221-7055-20")]
    #[value(name = "6g-221-7055-20")]
    Sixg221705520,
    #[serde(rename = "6g-225-7075-20")]
    #[value(name = "6g-225-7075-20")]
    Sixg225707520,
    #[serde(rename = "6g-227-7085-40")]
    #[value(name = "6g-227-7085-40")]
    Sixg227708540,
    #[serde(rename = "6g-229-7095-20")]
    #[value(name = "6g-229-7095-20")]
    Sixg229709520,
    #[serde(rename = "6g-233-7115-20")]
    #[value(name = "6g-233-7115-20")]
    Sixg233711520,
    #[serde(rename = "60g-1-58320-2160")]
    #[value(name = "60g-1-58320-2160")]
    Six0g1583202160,
    #[serde(rename = "60g-2-60480-2160")]
    #[value(name = "60g-2-60480-2160")]
    Six0g2604802160,
    #[serde(rename = "60g-3-62640-2160")]
    #[value(name = "60g-3-62640-2160")]
    Six0g3626402160,
    #[serde(rename = "60g-4-64800-2160")]
    #[value(name = "60g-4-64800-2160")]
    Six0g4648002160,
    #[serde(rename = "60g-5-66960-2160")]
    #[value(name = "60g-5-66960-2160")]
    Six0g5669602160,
    #[serde(rename = "60g-6-69120-2160")]
    #[value(name = "60g-6-69120-2160")]
    Six0g6691202160,
    #[serde(rename = "60g-9-59400-4320")]
    #[value(name = "60g-9-59400-4320")]
    Six0g9594004320,
    #[serde(rename = "60g-10-61560-4320")]
    #[value(name = "60g-10-61560-4320")]
    Six0g10615604320,
    #[serde(rename = "60g-11-63720-4320")]
    #[value(name = "60g-11-63720-4320")]
    Six0g11637204320,
    #[serde(rename = "60g-12-65880-4320")]
    #[value(name = "60g-12-65880-4320")]
    Six0g12658804320,
    #[serde(rename = "60g-13-68040-4320")]
    #[value(name = "60g-13-68040-4320")]
    Six0g13680404320,
    #[serde(rename = "60g-17-60480-6480")]
    #[value(name = "60g-17-60480-6480")]
    Six0g17604806480,
    #[serde(rename = "60g-18-62640-6480")]
    #[value(name = "60g-18-62640-6480")]
    Six0g18626406480,
    #[serde(rename = "60g-19-64800-6480")]
    #[value(name = "60g-19-64800-6480")]
    Six0g19648006480,
    #[serde(rename = "60g-20-66960-6480")]
    #[value(name = "60g-20-66960-6480")]
    Six0g20669606480,
    #[serde(rename = "60g-25-61560-6480")]
    #[value(name = "60g-25-61560-6480")]
    Six0g25615606480,
    #[serde(rename = "60g-26-63720-6480")]
    #[value(name = "60g-26-63720-6480")]
    Six0g26637206480,
    #[serde(rename = "60g-27-65880-6480")]
    #[value(name = "60g-27-65880-6480")]
    Six0g27658806480,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InterfaceRfRole {
    Ap,
    Station,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InterfaceType {
    Virtual,
    Bridge,
    Lag,
    #[serde(rename = "100base-fx")]
    #[value(name = "100base-fx")]
    One00baseFx,
    #[serde(rename = "100base-lfx")]
    #[value(name = "100base-lfx")]
    One00baseLfx,
    #[serde(rename = "100base-tx")]
    #[value(name = "100base-tx")]
    One00baseTx,
    #[serde(rename = "100base-t1")]
    #[value(name = "100base-t1")]
    One00baseT1,
    #[serde(rename = "1000base-bx10-d")]
    #[value(name = "1000base-bx10-d")]
    One000baseBx10D,
    #[serde(rename = "1000base-bx10-u")]
    #[value(name = "1000base-bx10-u")]
    One000baseBx10U,
    #[serde(rename = "1000base-cwdm")]
    #[value(name = "1000base-cwdm")]
    One000baseCwdm,
    #[serde(rename = "1000base-cx")]
    #[value(name = "1000base-cx")]
    One000baseCx,
    #[serde(rename = "1000base-dwdm")]
    #[value(name = "1000base-dwdm")]
    One000baseDwdm,
    #[serde(rename = "1000base-ex")]
    #[value(name = "1000base-ex")]
    One000baseEx,
    #[serde(rename = "1000base-lsx")]
    #[value(name = "1000base-lsx")]
    One000baseLsx,
    #[serde(rename = "1000base-lx")]
    #[value(name = "1000base-lx")]
    One000baseLx,
    #[serde(rename = "1000base-lx10")]
    #[value(name = "1000base-lx10")]
    One000baseLx10,
    #[serde(rename = "1000base-sx")]
    #[value(name = "1000base-sx")]
    One000baseSx,
    #[serde(rename = "1000base-t")]
    #[value(name = "1000base-t")]
    One000baseT,
    #[serde(rename = "1000base-tx")]
    #[value(name = "1000base-tx")]
    One000baseTx,
    #[serde(rename = "1000base-zx")]
    #[value(name = "1000base-zx")]
    One000baseZx,
    #[serde(rename = "2.5gbase-t")]
    #[value(name = "2.5gbase-t")]
    Two5gbaseT,
    #[serde(rename = "5gbase-t")]
    #[value(name = "5gbase-t")]
    FivegbaseT,
    #[serde(rename = "10gbase-br-d")]
    #[value(name = "10gbase-br-d")]
    One0gbaseBrD,
    #[serde(rename = "10gbase-br-u")]
    #[value(name = "10gbase-br-u")]
    One0gbaseBrU,
    #[serde(rename = "10gbase-cu")]
    #[value(name = "10gbase-cu")]
    One0gbaseCu,
    #[serde(rename = "10gbase-cx4")]
    #[value(name = "10gbase-cx4")]
    One0gbaseCx4,
    #[serde(rename = "10gbase-er")]
    #[value(name = "10gbase-er")]
    One0gbaseEr,
    #[serde(rename = "10gbase-lr")]
    #[value(name = "10gbase-lr")]
    One0gbaseLr,
    #[serde(rename = "10gbase-lrm")]
    #[value(name = "10gbase-lrm")]
    One0gbaseLrm,
    #[serde(rename = "10gbase-lx4")]
    #[value(name = "10gbase-lx4")]
    One0gbaseLx4,
    #[serde(rename = "10gbase-sr")]
    #[value(name = "10gbase-sr")]
    One0gbaseSr,
    #[serde(rename = "10gbase-t")]
    #[value(name = "10gbase-t")]
    One0gbaseT,
    #[serde(rename = "10gbase-zr")]
    #[value(name = "10gbase-zr")]
    One0gbaseZr,
    #[serde(rename = "25gbase-cr")]
    #[value(name = "25gbase-cr")]
    Two5gbaseCr,
    #[serde(rename = "25gbase-er")]
    #[value(name = "25gbase-er")]
    Two5gbaseEr,
    #[serde(rename = "25gbase-lr")]
    #[value(name = "25gbase-lr")]
    Two5gbaseLr,
    #[serde(rename = "25gbase-sr")]
    #[value(name = "25gbase-sr")]
    Two5gbaseSr,
    #[serde(rename = "25gbase-t")]
    #[value(name = "25gbase-t")]
    Two5gbaseT2,
    #[serde(rename = "40gbase-cr4")]
    #[value(name = "40gbase-cr4")]
    Four0gbaseCr4,
    #[serde(rename = "40gbase-er4")]
    #[value(name = "40gbase-er4")]
    Four0gbaseEr4,
    #[serde(rename = "40gbase-fr4")]
    #[value(name = "40gbase-fr4")]
    Four0gbaseFr4,
    #[serde(rename = "40gbase-lr4")]
    #[value(name = "40gbase-lr4")]
    Four0gbaseLr4,
    #[serde(rename = "40gbase-sr4")]
    #[value(name = "40gbase-sr4")]
    Four0gbaseSr4,
    #[serde(rename = "40gbase-sr4-bd")]
    #[value(name = "40gbase-sr4-bd")]
    Four0gbaseSr4Bd,
    #[serde(rename = "50gbase-cr")]
    #[value(name = "50gbase-cr")]
    Five0gbaseCr,
    #[serde(rename = "50gbase-er")]
    #[value(name = "50gbase-er")]
    Five0gbaseEr,
    #[serde(rename = "50gbase-fr")]
    #[value(name = "50gbase-fr")]
    Five0gbaseFr,
    #[serde(rename = "50gbase-lr")]
    #[value(name = "50gbase-lr")]
    Five0gbaseLr,
    #[serde(rename = "50gbase-sr")]
    #[value(name = "50gbase-sr")]
    Five0gbaseSr,
    #[serde(rename = "100gbase-cr1")]
    #[value(name = "100gbase-cr1")]
    One00gbaseCr1,
    #[serde(rename = "100gbase-cr2")]
    #[value(name = "100gbase-cr2")]
    One00gbaseCr2,
    #[serde(rename = "100gbase-cr4")]
    #[value(name = "100gbase-cr4")]
    One00gbaseCr4,
    #[serde(rename = "100gbase-cr10")]
    #[value(name = "100gbase-cr10")]
    One00gbaseCr10,
    #[serde(rename = "100gbase-cwdm4")]
    #[value(name = "100gbase-cwdm4")]
    One00gbaseCwdm4,
    #[serde(rename = "100gbase-dr")]
    #[value(name = "100gbase-dr")]
    One00gbaseDr,
    #[serde(rename = "100gbase-er4")]
    #[value(name = "100gbase-er4")]
    One00gbaseEr4,
    #[serde(rename = "100gbase-fr1")]
    #[value(name = "100gbase-fr1")]
    One00gbaseFr1,
    #[serde(rename = "100gbase-lr1")]
    #[value(name = "100gbase-lr1")]
    One00gbaseLr1,
    #[serde(rename = "100gbase-lr4")]
    #[value(name = "100gbase-lr4")]
    One00gbaseLr4,
    #[serde(rename = "100gbase-sr1")]
    #[value(name = "100gbase-sr1")]
    One00gbaseSr1,
    #[serde(rename = "100gbase-sr1.2")]
    #[value(name = "100gbase-sr1.2")]
    One00gbaseSr12,
    #[serde(rename = "100gbase-sr2")]
    #[value(name = "100gbase-sr2")]
    One00gbaseSr2,
    #[serde(rename = "100gbase-sr4")]
    #[value(name = "100gbase-sr4")]
    One00gbaseSr4,
    #[serde(rename = "100gbase-sr10")]
    #[value(name = "100gbase-sr10")]
    One00gbaseSr10,
    #[serde(rename = "100gbase-zr")]
    #[value(name = "100gbase-zr")]
    One00gbaseZr,
    #[serde(rename = "200gbase-cr2")]
    #[value(name = "200gbase-cr2")]
    Two00gbaseCr2,
    #[serde(rename = "200gbase-cr4")]
    #[value(name = "200gbase-cr4")]
    Two00gbaseCr4,
    #[serde(rename = "200gbase-dr4")]
    #[value(name = "200gbase-dr4")]
    Two00gbaseDr4,
    #[serde(rename = "200gbase-er4")]
    #[value(name = "200gbase-er4")]
    Two00gbaseEr4,
    #[serde(rename = "200gbase-fr4")]
    #[value(name = "200gbase-fr4")]
    Two00gbaseFr4,
    #[serde(rename = "200gbase-lr4")]
    #[value(name = "200gbase-lr4")]
    Two00gbaseLr4,
    #[serde(rename = "200gbase-sr2")]
    #[value(name = "200gbase-sr2")]
    Two00gbaseSr2,
    #[serde(rename = "200gbase-sr4")]
    #[value(name = "200gbase-sr4")]
    Two00gbaseSr4,
    #[serde(rename = "200gbase-vr2")]
    #[value(name = "200gbase-vr2")]
    Two00gbaseVr2,
    #[serde(rename = "400gbase-cr4")]
    #[value(name = "400gbase-cr4")]
    Four00gbaseCr4,
    #[serde(rename = "400gbase-dr4")]
    #[value(name = "400gbase-dr4")]
    Four00gbaseDr4,
    #[serde(rename = "400gbase-er8")]
    #[value(name = "400gbase-er8")]
    Four00gbaseEr8,
    #[serde(rename = "400gbase-fr4")]
    #[value(name = "400gbase-fr4")]
    Four00gbaseFr4,
    #[serde(rename = "400gbase-fr8")]
    #[value(name = "400gbase-fr8")]
    Four00gbaseFr8,
    #[serde(rename = "400gbase-lr4")]
    #[value(name = "400gbase-lr4")]
    Four00gbaseLr4,
    #[serde(rename = "400gbase-lr8")]
    #[value(name = "400gbase-lr8")]
    Four00gbaseLr8,
    #[serde(rename = "400gbase-sr4")]
    #[value(name = "400gbase-sr4")]
    Four00gbaseSr4,
    #[serde(rename = "400gbase-sr4_2")]
    #[value(name = "400gbase-sr4_2")]
    Four00gbaseSr42,
    #[serde(rename = "400gbase-sr8")]
    #[value(name = "400gbase-sr8")]
    Four00gbaseSr8,
    #[serde(rename = "400gbase-sr16")]
    #[value(name = "400gbase-sr16")]
    Four00gbaseSr16,
    #[serde(rename = "400gbase-vr4")]
    #[value(name = "400gbase-vr4")]
    Four00gbaseVr4,
    #[serde(rename = "400gbase-zr")]
    #[value(name = "400gbase-zr")]
    Four00gbaseZr,
    #[serde(rename = "800gbase-cr8")]
    #[value(name = "800gbase-cr8")]
    Eight00gbaseCr8,
    #[serde(rename = "800gbase-dr8")]
    #[value(name = "800gbase-dr8")]
    Eight00gbaseDr8,
    #[serde(rename = "800gbase-sr8")]
    #[value(name = "800gbase-sr8")]
    Eight00gbaseSr8,
    #[serde(rename = "800gbase-vr8")]
    #[value(name = "800gbase-vr8")]
    Eight00gbaseVr8,
    #[serde(rename = "1.6tbase-cr8")]
    #[value(name = "1.6tbase-cr8")]
    One6tbaseCr8,
    #[serde(rename = "1.6tbase-dr8")]
    #[value(name = "1.6tbase-dr8")]
    One6tbaseDr8,
    #[serde(rename = "1.6tbase-dr8-2")]
    #[value(name = "1.6tbase-dr8-2")]
    One6tbaseDr82,
    #[serde(rename = "100base-x-sfp")]
    #[value(name = "100base-x-sfp")]
    One00baseXSfp,
    #[serde(rename = "1000base-x-gbic")]
    #[value(name = "1000base-x-gbic")]
    One000baseXGbic,
    #[serde(rename = "1000base-x-sfp")]
    #[value(name = "1000base-x-sfp")]
    One000baseXSfp,
    #[serde(rename = "2.5gbase-x-sfp")]
    #[value(name = "2.5gbase-x-sfp")]
    Two5gbaseXSfp,
    #[serde(rename = "10gbase-x-sfpp")]
    #[value(name = "10gbase-x-sfpp")]
    One0gbaseXSfpp,
    #[serde(rename = "10gbase-x-xenpak")]
    #[value(name = "10gbase-x-xenpak")]
    One0gbaseXXenpak,
    #[serde(rename = "10gbase-x-xfp")]
    #[value(name = "10gbase-x-xfp")]
    One0gbaseXXfp,
    #[serde(rename = "10gbase-x-x2")]
    #[value(name = "10gbase-x-x2")]
    One0gbaseXX2,
    #[serde(rename = "25gbase-x-sfp28")]
    #[value(name = "25gbase-x-sfp28")]
    Two5gbaseXSfp28,
    #[serde(rename = "40gbase-x-qsfpp")]
    #[value(name = "40gbase-x-qsfpp")]
    Four0gbaseXQsfpp,
    #[serde(rename = "50gbase-x-sfp28")]
    #[value(name = "50gbase-x-sfp28")]
    Five0gbaseXSfp28,
    #[serde(rename = "50gbase-x-sfp56")]
    #[value(name = "50gbase-x-sfp56")]
    Five0gbaseXSfp56,
    #[serde(rename = "100gbase-x-cfp")]
    #[value(name = "100gbase-x-cfp")]
    One00gbaseXCfp,
    #[serde(rename = "100gbase-x-cfp2")]
    #[value(name = "100gbase-x-cfp2")]
    One00gbaseXCfp2,
    #[serde(rename = "100gbase-x-cfp4")]
    #[value(name = "100gbase-x-cfp4")]
    One00gbaseXCfp4,
    #[serde(rename = "100gbase-x-cxp")]
    #[value(name = "100gbase-x-cxp")]
    One00gbaseXCxp,
    #[serde(rename = "100gbase-x-cpak")]
    #[value(name = "100gbase-x-cpak")]
    One00gbaseXCpak,
    #[serde(rename = "100gbase-x-dsfp")]
    #[value(name = "100gbase-x-dsfp")]
    One00gbaseXDsfp,
    #[serde(rename = "100gbase-x-qsfp28")]
    #[value(name = "100gbase-x-qsfp28")]
    One00gbaseXQsfp28,
    #[serde(rename = "100gbase-x-qsfpdd")]
    #[value(name = "100gbase-x-qsfpdd")]
    One00gbaseXQsfpdd,
    #[serde(rename = "100gbase-x-sfpdd")]
    #[value(name = "100gbase-x-sfpdd")]
    One00gbaseXSfpdd,
    #[serde(rename = "200gbase-x-cfp2")]
    #[value(name = "200gbase-x-cfp2")]
    Two00gbaseXCfp2,
    #[serde(rename = "200gbase-x-qsfp56")]
    #[value(name = "200gbase-x-qsfp56")]
    Two00gbaseXQsfp56,
    #[serde(rename = "200gbase-x-qsfpdd")]
    #[value(name = "200gbase-x-qsfpdd")]
    Two00gbaseXQsfpdd,
    #[serde(rename = "400gbase-x-qsfp112")]
    #[value(name = "400gbase-x-qsfp112")]
    Four00gbaseXQsfp112,
    #[serde(rename = "400gbase-x-qsfpdd")]
    #[value(name = "400gbase-x-qsfpdd")]
    Four00gbaseXQsfpdd,
    #[serde(rename = "400gbase-x-cdfp")]
    #[value(name = "400gbase-x-cdfp")]
    Four00gbaseXCdfp,
    #[serde(rename = "400gbase-x-cfp2")]
    #[value(name = "400gbase-x-cfp2")]
    Four00gbaseXCfp2,
    #[serde(rename = "400gbase-x-cfp8")]
    #[value(name = "400gbase-x-cfp8")]
    Four00gbaseXCfp8,
    #[serde(rename = "400gbase-x-osfp")]
    #[value(name = "400gbase-x-osfp")]
    Four00gbaseXOsfp,
    #[serde(rename = "400gbase-x-osfp-rhs")]
    #[value(name = "400gbase-x-osfp-rhs")]
    Four00gbaseXOsfpRhs,
    #[serde(rename = "800gbase-x-osfp")]
    #[value(name = "800gbase-x-osfp")]
    Eight00gbaseXOsfp,
    #[serde(rename = "800gbase-x-qsfpdd")]
    #[value(name = "800gbase-x-qsfpdd")]
    Eight00gbaseXQsfpdd,
    #[serde(rename = "1.6tbase-x-osfp1600")]
    #[value(name = "1.6tbase-x-osfp1600")]
    One6tbaseXOsfp1600,
    #[serde(rename = "1.6tbase-x-osfp1600-rhs")]
    #[value(name = "1.6tbase-x-osfp1600-rhs")]
    One6tbaseXOsfp1600Rhs,
    #[serde(rename = "1.6tbase-x-qsfpdd1600")]
    #[value(name = "1.6tbase-x-qsfpdd1600")]
    One6tbaseXQsfpdd1600,
    #[serde(rename = "1000base-kx")]
    #[value(name = "1000base-kx")]
    One000baseKx,
    #[serde(rename = "2.5gbase-kx")]
    #[value(name = "2.5gbase-kx")]
    Two5gbaseKx,
    #[serde(rename = "5gbase-kr")]
    #[value(name = "5gbase-kr")]
    FivegbaseKr,
    #[serde(rename = "10gbase-kr")]
    #[value(name = "10gbase-kr")]
    One0gbaseKr,
    #[serde(rename = "10gbase-kx4")]
    #[value(name = "10gbase-kx4")]
    One0gbaseKx4,
    #[serde(rename = "25gbase-kr")]
    #[value(name = "25gbase-kr")]
    Two5gbaseKr,
    #[serde(rename = "40gbase-kr4")]
    #[value(name = "40gbase-kr4")]
    Four0gbaseKr4,
    #[serde(rename = "50gbase-kr")]
    #[value(name = "50gbase-kr")]
    Five0gbaseKr,
    #[serde(rename = "100gbase-kp4")]
    #[value(name = "100gbase-kp4")]
    One00gbaseKp4,
    #[serde(rename = "100gbase-kr2")]
    #[value(name = "100gbase-kr2")]
    One00gbaseKr2,
    #[serde(rename = "100gbase-kr4")]
    #[value(name = "100gbase-kr4")]
    One00gbaseKr4,
    #[serde(rename = "1.6tbase-kr8")]
    #[value(name = "1.6tbase-kr8")]
    One6tbaseKr8,
    #[serde(rename = "ieee802.11a")]
    #[value(name = "ieee802.11a")]
    Ieee80211a,
    #[serde(rename = "ieee802.11g")]
    #[value(name = "ieee802.11g")]
    Ieee80211g,
    #[serde(rename = "ieee802.11n")]
    #[value(name = "ieee802.11n")]
    Ieee80211n,
    #[serde(rename = "ieee802.11ac")]
    #[value(name = "ieee802.11ac")]
    Ieee80211ac,
    #[serde(rename = "ieee802.11ad")]
    #[value(name = "ieee802.11ad")]
    Ieee80211ad,
    #[serde(rename = "ieee802.11ax")]
    #[value(name = "ieee802.11ax")]
    Ieee80211ax,
    #[serde(rename = "ieee802.11ay")]
    #[value(name = "ieee802.11ay")]
    Ieee80211ay,
    #[serde(rename = "ieee802.11be")]
    #[value(name = "ieee802.11be")]
    Ieee80211be,
    #[serde(rename = "ieee802.15.1")]
    #[value(name = "ieee802.15.1")]
    Ieee802151,
    #[serde(rename = "ieee802.15.4")]
    #[value(name = "ieee802.15.4")]
    Ieee802154,
    #[serde(rename = "other-wireless")]
    #[value(name = "other-wireless")]
    OtherWireless,
    Gsm,
    Cdma,
    Lte,
    #[serde(rename = "4g")]
    #[value(name = "4g")]
    Fourg,
    #[serde(rename = "5g")]
    #[value(name = "5g")]
    Fiveg,
    #[serde(rename = "sonet-oc3")]
    #[value(name = "sonet-oc3")]
    SonetOc3,
    #[serde(rename = "sonet-oc12")]
    #[value(name = "sonet-oc12")]
    SonetOc12,
    #[serde(rename = "sonet-oc48")]
    #[value(name = "sonet-oc48")]
    SonetOc48,
    #[serde(rename = "sonet-oc192")]
    #[value(name = "sonet-oc192")]
    SonetOc192,
    #[serde(rename = "sonet-oc768")]
    #[value(name = "sonet-oc768")]
    SonetOc768,
    #[serde(rename = "sonet-oc1920")]
    #[value(name = "sonet-oc1920")]
    SonetOc1920,
    #[serde(rename = "sonet-oc3840")]
    #[value(name = "sonet-oc3840")]
    SonetOc3840,
    #[serde(rename = "1gfc-sfp")]
    #[value(name = "1gfc-sfp")]
    OnegfcSfp,
    #[serde(rename = "2gfc-sfp")]
    #[value(name = "2gfc-sfp")]
    TwogfcSfp,
    #[serde(rename = "4gfc-sfp")]
    #[value(name = "4gfc-sfp")]
    FourgfcSfp,
    #[serde(rename = "8gfc-sfpp")]
    #[value(name = "8gfc-sfpp")]
    EightgfcSfpp,
    #[serde(rename = "16gfc-sfpp")]
    #[value(name = "16gfc-sfpp")]
    One6gfcSfpp,
    #[serde(rename = "32gfc-sfp28")]
    #[value(name = "32gfc-sfp28")]
    Three2gfcSfp28,
    #[serde(rename = "32gfc-sfpp")]
    #[value(name = "32gfc-sfpp")]
    Three2gfcSfpp,
    #[serde(rename = "64gfc-qsfpp")]
    #[value(name = "64gfc-qsfpp")]
    Six4gfcQsfpp,
    #[serde(rename = "64gfc-sfpdd")]
    #[value(name = "64gfc-sfpdd")]
    Six4gfcSfpdd,
    #[serde(rename = "64gfc-sfpp")]
    #[value(name = "64gfc-sfpp")]
    Six4gfcSfpp,
    #[serde(rename = "128gfc-qsfp28")]
    #[value(name = "128gfc-qsfp28")]
    One28gfcQsfp28,
    #[serde(rename = "infiniband-sdr")]
    #[value(name = "infiniband-sdr")]
    InfinibandSdr,
    #[serde(rename = "infiniband-ddr")]
    #[value(name = "infiniband-ddr")]
    InfinibandDdr,
    #[serde(rename = "infiniband-qdr")]
    #[value(name = "infiniband-qdr")]
    InfinibandQdr,
    #[serde(rename = "infiniband-fdr10")]
    #[value(name = "infiniband-fdr10")]
    InfinibandFdr10,
    #[serde(rename = "infiniband-fdr")]
    #[value(name = "infiniband-fdr")]
    InfinibandFdr,
    #[serde(rename = "infiniband-edr")]
    #[value(name = "infiniband-edr")]
    InfinibandEdr,
    #[serde(rename = "infiniband-hdr")]
    #[value(name = "infiniband-hdr")]
    InfinibandHdr,
    #[serde(rename = "infiniband-ndr")]
    #[value(name = "infiniband-ndr")]
    InfinibandNdr,
    #[serde(rename = "infiniband-xdr")]
    #[value(name = "infiniband-xdr")]
    InfinibandXdr,
    T1,
    E1,
    T3,
    E3,
    Xdsl,
    Docsis,
    Moca,
    Bpon,
    Epon,
    #[serde(rename = "10g-epon")]
    #[value(name = "10g-epon")]
    One0gEpon,
    Gpon,
    #[serde(rename = "xg-pon")]
    #[value(name = "xg-pon")]
    XgPon,
    #[serde(rename = "xgs-pon")]
    #[value(name = "xgs-pon")]
    XgsPon,
    #[serde(rename = "ng-pon2")]
    #[value(name = "ng-pon2")]
    NgPon2,
    #[serde(rename = "25g-pon")]
    #[value(name = "25g-pon")]
    Two5gPon,
    #[serde(rename = "50g-pon")]
    #[value(name = "50g-pon")]
    Five0gPon,
    #[serde(rename = "cisco-stackwise")]
    #[value(name = "cisco-stackwise")]
    CiscoStackwise,
    #[serde(rename = "cisco-stackwise-plus")]
    #[value(name = "cisco-stackwise-plus")]
    CiscoStackwisePlus,
    #[serde(rename = "cisco-flexstack")]
    #[value(name = "cisco-flexstack")]
    CiscoFlexstack,
    #[serde(rename = "cisco-flexstack-plus")]
    #[value(name = "cisco-flexstack-plus")]
    CiscoFlexstackPlus,
    #[serde(rename = "cisco-stackwise-80")]
    #[value(name = "cisco-stackwise-80")]
    CiscoStackwise80,
    #[serde(rename = "cisco-stackwise-160")]
    #[value(name = "cisco-stackwise-160")]
    CiscoStackwise160,
    #[serde(rename = "cisco-stackwise-320")]
    #[value(name = "cisco-stackwise-320")]
    CiscoStackwise320,
    #[serde(rename = "cisco-stackwise-480")]
    #[value(name = "cisco-stackwise-480")]
    CiscoStackwise480,
    #[serde(rename = "cisco-stackwise-1t")]
    #[value(name = "cisco-stackwise-1t")]
    CiscoStackwise1t,
    #[serde(rename = "juniper-vcp")]
    #[value(name = "juniper-vcp")]
    JuniperVcp,
    #[serde(rename = "extreme-summitstack")]
    #[value(name = "extreme-summitstack")]
    ExtremeSummitstack,
    #[serde(rename = "extreme-summitstack-128")]
    #[value(name = "extreme-summitstack-128")]
    ExtremeSummitstack128,
    #[serde(rename = "extreme-summitstack-256")]
    #[value(name = "extreme-summitstack-256")]
    ExtremeSummitstack256,
    #[serde(rename = "extreme-summitstack-512")]
    #[value(name = "extreme-summitstack-512")]
    ExtremeSummitstack512,
    Other,
}

#[derive(Debug, Args)]
pub struct InterfaceCreateFields {
    #[arg(long)]
    pub bridge: Option<u64>,

    #[arg(long)]
    pub description: Option<String>,

    /// * `half` - Half
    /// * `full` - Full
    /// * `auto` - Auto
    #[arg(long)]
    pub duplex: Option<InterfaceDuplex>,

    #[arg(long)]
    pub enabled: Option<bool>,

    /// Physical label
    #[arg(long)]
    pub label: Option<String>,

    /// Treat as if a cable is connected
    #[arg(long)]
    pub mark_connected: Option<bool>,

    /// This interface is used only for out-of-band management
    #[arg(long)]
    pub mgmt_only: Option<bool>,

    /// IEEE 802.1Q tagging strategy
    ///
    /// * `access` - Access
    /// * `tagged` - Tagged
    /// * `tagged-all` - Tagged (All)
    /// * `q-in-q` - Q-in-Q (802.1ad)
    #[arg(long)]
    pub mode: Option<InterfaceMode>,

    #[arg(long)]
    pub module: Option<String>,

    #[arg(long)]
    pub mtu: Option<u64>,

    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub parent: Option<u64>,

    /// * `pd` - PD
    /// * `pse` - PSE
    #[arg(long)]
    pub poe_mode: Option<InterfacePoeMode>,

    /// * `type1-ieee802.3af` - 802.3af (Type 1)
    /// * `type2-ieee802.3at` - 802.3at (Type 2)
    /// * `type3-ieee802.3bt` - 802.3bt (Type 3)
    /// * `type4-ieee802.3bt` - 802.3bt (Type 4)
    /// * `passive-24v-2pair` - Passive 24V (2-pair)
    /// * `passive-24v-4pair` - Passive 24V (4-pair)
    /// * `passive-48v-2pair` - Passive 48V (2-pair)
    /// * `passive-48v-4pair` - Passive 48V (4-pair)
    #[arg(long)]
    pub poe_type: Option<InterfacePoeType>,

    #[arg(long)]
    pub primary_mac_address: Option<String>,

    #[arg(long)]
    pub qinq_svlan: Option<String>,

    /// * `2.4g-1-2412-22` - 1 (2412 MHz)
    /// * `2.4g-2-2417-22` - 2 (2417 MHz)
    /// * `2.4g-3-2422-22` - 3 (2422 MHz)
    /// * `2.4g-4-2427-22` - 4 (2427 MHz)
    /// * `2.4g-5-2432-22` - 5 (2432 MHz)
    /// * `2.4g-6-2437-22` - 6 (2437 MHz)
    /// * `2.4g-7-2442-22` - 7 (2442 MHz)
    /// * `2.4g-8-2447-22` - 8 (2447 MHz)
    /// * `2.4g-9-2452-22` - 9 (2452 MHz)
    /// * `2.4g-10-2457-22` - 10 (2457 MHz)
    /// * `2.4g-11-2462-22` - 11 (2462 MHz)
    /// * `2.4g-12-2467-22` - 12 (2467 MHz)
    /// * `2.4g-13-2472-22` - 13 (2472 MHz)
    /// * `5g-32-5160-20` - 32 (5160/20 MHz)
    /// * `5g-34-5170-40` - 34 (5170/40 MHz)
    /// * `5g-36-5180-20` - 36 (5180/20 MHz)
    /// * `5g-38-5190-40` - 38 (5190/40 MHz)
    /// * `5g-40-5200-20` - 40 (5200/20 MHz)
    /// * `5g-42-5210-80` - 42 (5210/80 MHz)
    /// * `5g-44-5220-20` - 44 (5220/20 MHz)
    /// * `5g-46-5230-40` - 46 (5230/40 MHz)
    /// * `5g-48-5240-20` - 48 (5240/20 MHz)
    /// * `5g-50-5250-160` - 50 (5250/160 MHz)
    /// * `5g-52-5260-20` - 52 (5260/20 MHz)
    /// * `5g-54-5270-40` - 54 (5270/40 MHz)
    /// * `5g-56-5280-20` - 56 (5280/20 MHz)
    /// * `5g-58-5290-80` - 58 (5290/80 MHz)
    /// * `5g-60-5300-20` - 60 (5300/20 MHz)
    /// * `5g-62-5310-40` - 62 (5310/40 MHz)
    /// * `5g-64-5320-20` - 64 (5320/20 MHz)
    /// * `5g-100-5500-20` - 100 (5500/20 MHz)
    /// * `5g-102-5510-40` - 102 (5510/40 MHz)
    /// * `5g-104-5520-20` - 104 (5520/20 MHz)
    /// * `5g-106-5530-80` - 106 (5530/80 MHz)
    /// * `5g-108-5540-20` - 108 (5540/20 MHz)
    /// * `5g-110-5550-40` - 110 (5550/40 MHz)
    /// * `5g-112-5560-20` - 112 (5560/20 MHz)
    /// * `5g-114-5570-160` - 114 (5570/160 MHz)
    /// * `5g-116-5580-20` - 116 (5580/20 MHz)
    /// * `5g-118-5590-40` - 118 (5590/40 MHz)
    /// * `5g-120-5600-20` - 120 (5600/20 MHz)
    /// * `5g-122-5610-80` - 122 (5610/80 MHz)
    /// * `5g-124-5620-20` - 124 (5620/20 MHz)
    /// * `5g-126-5630-40` - 126 (5630/40 MHz)
    /// * `5g-128-5640-20` - 128 (5640/20 MHz)
    /// * `5g-132-5660-20` - 132 (5660/20 MHz)
    /// * `5g-134-5670-40` - 134 (5670/40 MHz)
    /// * `5g-136-5680-20` - 136 (5680/20 MHz)
    /// * `5g-138-5690-80` - 138 (5690/80 MHz)
    /// * `5g-140-5700-20` - 140 (5700/20 MHz)
    /// * `5g-142-5710-40` - 142 (5710/40 MHz)
    /// * `5g-144-5720-20` - 144 (5720/20 MHz)
    /// * `5g-149-5745-20` - 149 (5745/20 MHz)
    /// * `5g-151-5755-40` - 151 (5755/40 MHz)
    /// * `5g-153-5765-20` - 153 (5765/20 MHz)
    /// * `5g-155-5775-80` - 155 (5775/80 MHz)
    /// * `5g-157-5785-20` - 157 (5785/20 MHz)
    /// * `5g-159-5795-40` - 159 (5795/40 MHz)
    /// * `5g-161-5805-20` - 161 (5805/20 MHz)
    /// * `5g-163-5815-160` - 163 (5815/160 MHz)
    /// * `5g-165-5825-20` - 165 (5825/20 MHz)
    /// * `5g-167-5835-40` - 167 (5835/40 MHz)
    /// * `5g-169-5845-20` - 169 (5845/20 MHz)
    /// * `5g-171-5855-80` - 171 (5855/80 MHz)
    /// * `5g-173-5865-20` - 173 (5865/20 MHz)
    /// * `5g-175-5875-40` - 175 (5875/40 MHz)
    /// * `5g-177-5885-20` - 177 (5885/20 MHz)
    /// * `6g-1-5955-20` - 1 (5955/20 MHz)
    /// * `6g-3-5965-40` - 3 (5965/40 MHz)
    /// * `6g-5-5975-20` - 5 (5975/20 MHz)
    /// * `6g-7-5985-80` - 7 (5985/80 MHz)
    /// * `6g-9-5995-20` - 9 (5995/20 MHz)
    /// * `6g-11-6005-40` - 11 (6005/40 MHz)
    /// * `6g-13-6015-20` - 13 (6015/20 MHz)
    /// * `6g-15-6025-160` - 15 (6025/160 MHz)
    /// * `6g-17-6035-20` - 17 (6035/20 MHz)
    /// * `6g-19-6045-40` - 19 (6045/40 MHz)
    /// * `6g-21-6055-20` - 21 (6055/20 MHz)
    /// * `6g-23-6065-80` - 23 (6065/80 MHz)
    /// * `6g-25-6075-20` - 25 (6075/20 MHz)
    /// * `6g-27-6085-40` - 27 (6085/40 MHz)
    /// * `6g-29-6095-20` - 29 (6095/20 MHz)
    /// * `6g-31-6105-320` - 31 (6105/320 MHz)
    /// * `6g-33-6115-20` - 33 (6115/20 MHz)
    /// * `6g-35-6125-40` - 35 (6125/40 MHz)
    /// * `6g-37-6135-20` - 37 (6135/20 MHz)
    /// * `6g-39-6145-80` - 39 (6145/80 MHz)
    /// * `6g-41-6155-20` - 41 (6155/20 MHz)
    /// * `6g-43-6165-40` - 43 (6165/40 MHz)
    /// * `6g-45-6175-20` - 45 (6175/20 MHz)
    /// * `6g-47-6185-160` - 47 (6185/160 MHz)
    /// * `6g-49-6195-20` - 49 (6195/20 MHz)
    /// * `6g-51-6205-40` - 51 (6205/40 MHz)
    /// * `6g-53-6215-20` - 53 (6215/20 MHz)
    /// * `6g-55-6225-80` - 55 (6225/80 MHz)
    /// * `6g-57-6235-20` - 57 (6235/20 MHz)
    /// * `6g-59-6245-40` - 59 (6245/40 MHz)
    /// * `6g-61-6255-20` - 61 (6255/20 MHz)
    /// * `6g-65-6275-20` - 65 (6275/20 MHz)
    /// * `6g-67-6285-40` - 67 (6285/40 MHz)
    /// * `6g-69-6295-20` - 69 (6295/20 MHz)
    /// * `6g-71-6305-80` - 71 (6305/80 MHz)
    /// * `6g-73-6315-20` - 73 (6315/20 MHz)
    /// * `6g-75-6325-40` - 75 (6325/40 MHz)
    /// * `6g-77-6335-20` - 77 (6335/20 MHz)
    /// * `6g-79-6345-160` - 79 (6345/160 MHz)
    /// * `6g-81-6355-20` - 81 (6355/20 MHz)
    /// * `6g-83-6365-40` - 83 (6365/40 MHz)
    /// * `6g-85-6375-20` - 85 (6375/20 MHz)
    /// * `6g-87-6385-80` - 87 (6385/80 MHz)
    /// * `6g-89-6395-20` - 89 (6395/20 MHz)
    /// * `6g-91-6405-40` - 91 (6405/40 MHz)
    /// * `6g-93-6415-20` - 93 (6415/20 MHz)
    /// * `6g-95-6425-320` - 95 (6425/320 MHz)
    /// * `6g-97-6435-20` - 97 (6435/20 MHz)
    /// * `6g-99-6445-40` - 99 (6445/40 MHz)
    /// * `6g-101-6455-20` - 101 (6455/20 MHz)
    /// * `6g-103-6465-80` - 103 (6465/80 MHz)
    /// * `6g-105-6475-20` - 105 (6475/20 MHz)
    /// * `6g-107-6485-40` - 107 (6485/40 MHz)
    /// * `6g-109-6495-20` - 109 (6495/20 MHz)
    /// * `6g-111-6505-160` - 111 (6505/160 MHz)
    /// * `6g-113-6515-20` - 113 (6515/20 MHz)
    /// * `6g-115-6525-40` - 115 (6525/40 MHz)
    /// * `6g-117-6535-20` - 117 (6535/20 MHz)
    /// * `6g-119-6545-80` - 119 (6545/80 MHz)
    /// * `6g-121-6555-20` - 121 (6555/20 MHz)
    /// * `6g-123-6565-40` - 123 (6565/40 MHz)
    /// * `6g-125-6575-20` - 125 (6575/20 MHz)
    /// * `6g-129-6595-20` - 129 (6595/20 MHz)
    /// * `6g-131-6605-40` - 131 (6605/40 MHz)
    /// * `6g-133-6615-20` - 133 (6615/20 MHz)
    /// * `6g-135-6625-80` - 135 (6625/80 MHz)
    /// * `6g-137-6635-20` - 137 (6635/20 MHz)
    /// * `6g-139-6645-40` - 139 (6645/40 MHz)
    /// * `6g-141-6655-20` - 141 (6655/20 MHz)
    /// * `6g-143-6665-160` - 143 (6665/160 MHz)
    /// * `6g-145-6675-20` - 145 (6675/20 MHz)
    /// * `6g-147-6685-40` - 147 (6685/40 MHz)
    /// * `6g-149-6695-20` - 149 (6695/20 MHz)
    /// * `6g-151-6705-80` - 151 (6705/80 MHz)
    /// * `6g-153-6715-20` - 153 (6715/20 MHz)
    /// * `6g-155-6725-40` - 155 (6725/40 MHz)
    /// * `6g-157-6735-20` - 157 (6735/20 MHz)
    /// * `6g-159-6745-320` - 159 (6745/320 MHz)
    /// * `6g-161-6755-20` - 161 (6755/20 MHz)
    /// * `6g-163-6765-40` - 163 (6765/40 MHz)
    /// * `6g-165-6775-20` - 165 (6775/20 MHz)
    /// * `6g-167-6785-80` - 167 (6785/80 MHz)
    /// * `6g-169-6795-20` - 169 (6795/20 MHz)
    /// * `6g-171-6805-40` - 171 (6805/40 MHz)
    /// * `6g-173-6815-20` - 173 (6815/20 MHz)
    /// * `6g-175-6825-160` - 175 (6825/160 MHz)
    /// * `6g-177-6835-20` - 177 (6835/20 MHz)
    /// * `6g-179-6845-40` - 179 (6845/40 MHz)
    /// * `6g-181-6855-20` - 181 (6855/20 MHz)
    /// * `6g-183-6865-80` - 183 (6865/80 MHz)
    /// * `6g-185-6875-20` - 185 (6875/20 MHz)
    /// * `6g-187-6885-40` - 187 (6885/40 MHz)
    /// * `6g-189-6895-20` - 189 (6895/20 MHz)
    /// * `6g-193-6915-20` - 193 (6915/20 MHz)
    /// * `6g-195-6925-40` - 195 (6925/40 MHz)
    /// * `6g-197-6935-20` - 197 (6935/20 MHz)
    /// * `6g-199-6945-80` - 199 (6945/80 MHz)
    /// * `6g-201-6955-20` - 201 (6955/20 MHz)
    /// * `6g-203-6965-40` - 203 (6965/40 MHz)
    /// * `6g-205-6975-20` - 205 (6975/20 MHz)
    /// * `6g-207-6985-160` - 207 (6985/160 MHz)
    /// * `6g-209-6995-20` - 209 (6995/20 MHz)
    /// * `6g-211-7005-40` - 211 (7005/40 MHz)
    /// * `6g-213-7015-20` - 213 (7015/20 MHz)
    /// * `6g-215-7025-80` - 215 (7025/80 MHz)
    /// * `6g-217-7035-20` - 217 (7035/20 MHz)
    /// * `6g-219-7045-40` - 219 (7045/40 MHz)
    /// * `6g-221-7055-20` - 221 (7055/20 MHz)
    /// * `6g-225-7075-20` - 225 (7075/20 MHz)
    /// * `6g-227-7085-40` - 227 (7085/40 MHz)
    /// * `6g-229-7095-20` - 229 (7095/20 MHz)
    /// * `6g-233-7115-20` - 233 (7115/20 MHz)
    /// * `60g-1-58320-2160` - 1 (58.32/2.16 GHz)
    /// * `60g-2-60480-2160` - 2 (60.48/2.16 GHz)
    /// * `60g-3-62640-2160` - 3 (62.64/2.16 GHz)
    /// * `60g-4-64800-2160` - 4 (64.80/2.16 GHz)
    /// * `60g-5-66960-2160` - 5 (66.96/2.16 GHz)
    /// * `60g-6-69120-2160` - 6 (69.12/2.16 GHz)
    /// * `60g-9-59400-4320` - 9 (59.40/4.32 GHz)
    /// * `60g-10-61560-4320` - 10 (61.56/4.32 GHz)
    /// * `60g-11-63720-4320` - 11 (63.72/4.32 GHz)
    /// * `60g-12-65880-4320` - 12 (65.88/4.32 GHz)
    /// * `60g-13-68040-4320` - 13 (68.04/4.32 GHz)
    /// * `60g-17-60480-6480` - 17 (60.48/6.48 GHz)
    /// * `60g-18-62640-6480` - 18 (62.64/6.48 GHz)
    /// * `60g-19-64800-6480` - 19 (64.80/6.48 GHz)
    /// * `60g-20-66960-6480` - 20 (66.96/6.48 GHz)
    /// * `60g-25-61560-6480` - 25 (61.56/8.64 GHz)
    /// * `60g-26-63720-6480` - 26 (63.72/8.64 GHz)
    /// * `60g-27-65880-6480` - 27 (65.88/8.64 GHz)
    #[arg(long)]
    pub rf_channel: Option<InterfaceRfChannel>,

    /// Populated by selected channel (if set)
    #[arg(long)]
    pub rf_channel_frequency: Option<f64>,

    /// Populated by selected channel (if set)
    #[arg(long)]
    pub rf_channel_width: Option<f64>,

    /// * `ap` - Access point
    /// * `station` - Station
    #[arg(long)]
    pub rf_role: Option<InterfaceRfRole>,

    #[arg(long)]
    pub speed: Option<u64>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tx_power: Option<u64>,

    /// * `virtual` - Virtual
    /// * `bridge` - Bridge
    /// * `lag` - Link Aggregation Group (LAG)
    /// * `100base-fx` - 100BASE-FX (10/100ME)
    /// * `100base-lfx` - 100BASE-LFX (10/100ME)
    /// * `100base-tx` - 100BASE-TX (10/100ME)
    /// * `100base-t1` - 100BASE-T1 (10/100ME)
    /// * `1000base-bx10-d` - 1000BASE-BX10-D (1GE BiDi Down)
    /// * `1000base-bx10-u` - 1000BASE-BX10-U (1GE BiDi Up)
    /// * `1000base-cwdm` - 1000BASE-CWDM (1GE)
    /// * `1000base-cx` - 1000BASE-CX (1GE DAC)
    /// * `1000base-dwdm` - 1000BASE-DWDM (1GE)
    /// * `1000base-ex` - 1000BASE-EX (1GE)
    /// * `1000base-lsx` - 1000BASE-LSX (1GE)
    /// * `1000base-lx` - 1000BASE-LX (1GE)
    /// * `1000base-lx10` - 1000BASE-LX10/LH (1GE)
    /// * `1000base-sx` - 1000BASE-SX (1GE)
    /// * `1000base-t` - 1000BASE-T (1GE)
    /// * `1000base-tx` - 1000BASE-TX (1GE)
    /// * `1000base-zx` - 1000BASE-ZX (1GE)
    /// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
    /// * `5gbase-t` - 5GBASE-T (5GE)
    /// * `10gbase-br-d` - 10GBASE-BR-D (10GE BiDi Down)
    /// * `10gbase-br-u` - 10GBASE-BR-U (10GE BiDi Up)
    /// * `10gbase-cu` - 10GBASE-CU (10GE DAC Passive Twinax)
    /// * `10gbase-cx4` - 10GBASE-CX4 (10GE DAC)
    /// * `10gbase-er` - 10GBASE-ER (10GE)
    /// * `10gbase-lr` - 10GBASE-LR (10GE)
    /// * `10gbase-lrm` - 10GBASE-LRM (10GE)
    /// * `10gbase-lx4` - 10GBASE-LX4 (10GE)
    /// * `10gbase-sr` - 10GBASE-SR (10GE)
    /// * `10gbase-t` - 10GBASE-T (10GE)
    /// * `10gbase-zr` - 10GBASE-ZR (10GE)
    /// * `25gbase-cr` - 25GBASE-CR (25GE DAC)
    /// * `25gbase-er` - 25GBASE-ER (25GE)
    /// * `25gbase-lr` - 25GBASE-LR (25GE)
    /// * `25gbase-sr` - 25GBASE-SR (25GE)
    /// * `25gbase-t` - 25GBASE-T (25GE)
    /// * `40gbase-cr4` - 40GBASE-CR4 (40GE DAC)
    /// * `40gbase-er4` - 40GBASE-ER4 (40GE)
    /// * `40gbase-fr4` - 40GBASE-FR4 (40GE)
    /// * `40gbase-lr4` - 40GBASE-LR4 (40GE)
    /// * `40gbase-sr4` - 40GBASE-SR4 (40GE)
    /// * `40gbase-sr4-bd` - 40GBASE-SR4 (40GE BiDi)
    /// * `50gbase-cr` - 50GBASE-CR (50GE DAC)
    /// * `50gbase-er` - 50GBASE-ER (50GE)
    /// * `50gbase-fr` - 50GBASE-FR (50GE)
    /// * `50gbase-lr` - 50GBASE-LR (50GE)
    /// * `50gbase-sr` - 50GBASE-SR (50GE)
    /// * `100gbase-cr1` - 100GBASE-CR1 (100GE DAC)
    /// * `100gbase-cr2` - 100GBASE-CR2 (100GE DAC)
    /// * `100gbase-cr4` - 100GBASE-CR4 (100GE DAC)
    /// * `100gbase-cr10` - 100GBASE-CR10 (100GE DAC)
    /// * `100gbase-cwdm4` - 100GBASE-CWDM4 (100GE)
    /// * `100gbase-dr` - 100GBASE-DR (100GE)
    /// * `100gbase-er4` - 100GBASE-ER4 (100GE)
    /// * `100gbase-fr1` - 100GBASE-FR1 (100GE)
    /// * `100gbase-lr1` - 100GBASE-LR1 (100GE)
    /// * `100gbase-lr4` - 100GBASE-LR4 (100GE)
    /// * `100gbase-sr1` - 100GBASE-SR1 (100GE)
    /// * `100gbase-sr1.2` - 100GBASE-SR1.2 (100GE BiDi)
    /// * `100gbase-sr2` - 100GBASE-SR2 (100GE)
    /// * `100gbase-sr4` - 100GBASE-SR4 (100GE)
    /// * `100gbase-sr10` - 100GBASE-SR10 (100GE)
    /// * `100gbase-zr` - 100GBASE-ZR (100GE)
    /// * `200gbase-cr2` - 200GBASE-CR2 (200GE)
    /// * `200gbase-cr4` - 200GBASE-CR4 (200GE)
    /// * `200gbase-dr4` - 200GBASE-DR4 (200GE)
    /// * `200gbase-er4` - 200GBASE-ER4 (200GE)
    /// * `200gbase-fr4` - 200GBASE-FR4 (200GE)
    /// * `200gbase-lr4` - 200GBASE-LR4 (200GE)
    /// * `200gbase-sr2` - 200GBASE-SR2 (200GE)
    /// * `200gbase-sr4` - 200GBASE-SR4 (200GE)
    /// * `200gbase-vr2` - 200GBASE-VR2 (200GE)
    /// * `400gbase-cr4` - 400GBASE-CR4 (400GE)
    /// * `400gbase-dr4` - 400GBASE-DR4 (400GE)
    /// * `400gbase-er8` - 400GBASE-ER8 (400GE)
    /// * `400gbase-fr4` - 400GBASE-FR4 (400GE)
    /// * `400gbase-fr8` - 400GBASE-FR8 (400GE)
    /// * `400gbase-lr4` - 400GBASE-LR4 (400GE)
    /// * `400gbase-lr8` - 400GBASE-LR8 (400GE)
    /// * `400gbase-sr4` - 400GBASE-SR4 (400GE)
    /// * `400gbase-sr4_2` - 400GBASE-SR4.2 (400GE BiDi)
    /// * `400gbase-sr8` - 400GBASE-SR8 (400GE)
    /// * `400gbase-sr16` - 400GBASE-SR16 (400GE)
    /// * `400gbase-vr4` - 400GBASE-VR4 (400GE)
    /// * `400gbase-zr` - 400GBASE-ZR (400GE)
    /// * `800gbase-cr8` - 800GBASE-CR8 (800GE)
    /// * `800gbase-dr8` - 800GBASE-DR8 (800GE)
    /// * `800gbase-sr8` - 800GBASE-SR8 (800GE)
    /// * `800gbase-vr8` - 800GBASE-VR8 (800GE)
    /// * `1.6tbase-cr8` - 1.6TBASE-CR8 (1.6TE)
    /// * `1.6tbase-dr8` - 1.6TBASE-DR8 (1.6TE)
    /// * `1.6tbase-dr8-2` - 1.6TBASE-DR8-2 (1.6TE)
    /// * `100base-x-sfp` - SFP (100ME)
    /// * `1000base-x-gbic` - GBIC (1GE)
    /// * `1000base-x-sfp` - SFP (1GE)
    /// * `2.5gbase-x-sfp` - SFP (2.5GE)
    /// * `10gbase-x-sfpp` - SFP+ (10GE)
    /// * `10gbase-x-xenpak` - XENPAK (10GE)
    /// * `10gbase-x-xfp` - XFP (10GE)
    /// * `10gbase-x-x2` - X2 (10GE)
    /// * `25gbase-x-sfp28` - SFP28 (25GE)
    /// * `40gbase-x-qsfpp` - QSFP+ (40GE)
    /// * `50gbase-x-sfp28` - QSFP28 (50GE)
    /// * `50gbase-x-sfp56` - SFP56 (50GE)
    /// * `100gbase-x-cfp` - CFP (100GE)
    /// * `100gbase-x-cfp2` - CFP2 (100GE)
    /// * `100gbase-x-cfp4` - CFP4 (100GE)
    /// * `100gbase-x-cxp` - CXP (100GE)
    /// * `100gbase-x-cpak` - Cisco CPAK (100GE)
    /// * `100gbase-x-dsfp` - DSFP (100GE)
    /// * `100gbase-x-qsfp28` - QSFP28 (100GE)
    /// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
    /// * `100gbase-x-sfpdd` - SFP-DD (100GE)
    /// * `200gbase-x-cfp2` - CFP2 (200GE)
    /// * `200gbase-x-qsfp56` - QSFP56 (200GE)
    /// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
    /// * `400gbase-x-qsfp112` - QSFP112 (400GE)
    /// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
    /// * `400gbase-x-cdfp` - CDFP (400GE)
    /// * `400gbase-x-cfp2` - CFP2 (400GE)
    /// * `400gbase-x-cfp8` - CPF8 (400GE)
    /// * `400gbase-x-osfp` - OSFP (400GE)
    /// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
    /// * `800gbase-x-osfp` - OSFP (800GE)
    /// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
    /// * `1.6tbase-x-osfp1600` - OSFP1600 (1.6TE)
    /// * `1.6tbase-x-osfp1600-rhs` - OSFP1600-RHS (1.6TE)
    /// * `1.6tbase-x-qsfpdd1600` - QSFP-DD1600 (1.6TE)
    /// * `1000base-kx` - 1000BASE-KX (1GE)
    /// * `2.5gbase-kx` - 2.5GBASE-KX (2.5GE)
    /// * `5gbase-kr` - 5GBASE-KR (5GE)
    /// * `10gbase-kr` - 10GBASE-KR (10GE)
    /// * `10gbase-kx4` - 10GBASE-KX4 (10GE)
    /// * `25gbase-kr` - 25GBASE-KR (25GE)
    /// * `40gbase-kr4` - 40GBASE-KR4 (40GE)
    /// * `50gbase-kr` - 50GBASE-KR (50GE)
    /// * `100gbase-kp4` - 100GBASE-KP4 (100GE)
    /// * `100gbase-kr2` - 100GBASE-KR2 (100GE)
    /// * `100gbase-kr4` - 100GBASE-KR4 (100GE)
    /// * `1.6tbase-kr8` - 1.6TBASE-KR8 (1.6TE)
    /// * `ieee802.11a` - IEEE 802.11a
    /// * `ieee802.11g` - IEEE 802.11b/g
    /// * `ieee802.11n` - IEEE 802.11n (Wi-Fi 4)
    /// * `ieee802.11ac` - IEEE 802.11ac (Wi-Fi 5)
    /// * `ieee802.11ad` - IEEE 802.11ad (WiGig)
    /// * `ieee802.11ax` - IEEE 802.11ax (Wi-Fi 6)
    /// * `ieee802.11ay` - IEEE 802.11ay (WiGig)
    /// * `ieee802.11be` - IEEE 802.11be (Wi-Fi 7)
    /// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
    /// * `ieee802.15.4` - IEEE 802.15.4 (LR-WPAN)
    /// * `other-wireless` - Other (Wireless)
    /// * `gsm` - GSM
    /// * `cdma` - CDMA
    /// * `lte` - LTE
    /// * `4g` - 4G
    /// * `5g` - 5G
    /// * `sonet-oc3` - OC-3/STM-1
    /// * `sonet-oc12` - OC-12/STM-4
    /// * `sonet-oc48` - OC-48/STM-16
    /// * `sonet-oc192` - OC-192/STM-64
    /// * `sonet-oc768` - OC-768/STM-256
    /// * `sonet-oc1920` - OC-1920/STM-640
    /// * `sonet-oc3840` - OC-3840/STM-1234
    /// * `1gfc-sfp` - SFP (1GFC)
    /// * `2gfc-sfp` - SFP (2GFC)
    /// * `4gfc-sfp` - SFP (4GFC)
    /// * `8gfc-sfpp` - SFP+ (8GFC)
    /// * `16gfc-sfpp` - SFP+ (16GFC)
    /// * `32gfc-sfp28` - SFP28 (32GFC)
    /// * `32gfc-sfpp` - SFP+ (32GFC)
    /// * `64gfc-qsfpp` - QSFP+ (64GFC)
    /// * `64gfc-sfpdd` - SFP-DD (64GFC)
    /// * `64gfc-sfpp` - SFP+ (64GFC)
    /// * `128gfc-qsfp28` - QSFP28 (128GFC)
    /// * `infiniband-sdr` - SDR (2 Gbps)
    /// * `infiniband-ddr` - DDR (4 Gbps)
    /// * `infiniband-qdr` - QDR (8 Gbps)
    /// * `infiniband-fdr10` - FDR10 (10 Gbps)
    /// * `infiniband-fdr` - FDR (13.5 Gbps)
    /// * `infiniband-edr` - EDR (25 Gbps)
    /// * `infiniband-hdr` - HDR (50 Gbps)
    /// * `infiniband-ndr` - NDR (100 Gbps)
    /// * `infiniband-xdr` - XDR (250 Gbps)
    /// * `t1` - T1 (1.544 Mbps)
    /// * `e1` - E1 (2.048 Mbps)
    /// * `t3` - T3 (45 Mbps)
    /// * `e3` - E3 (34 Mbps)
    /// * `xdsl` - xDSL
    /// * `docsis` - DOCSIS
    /// * `moca` - MoCA
    /// * `bpon` - BPON (622 Mbps / 155 Mbps)
    /// * `epon` - EPON (1 Gbps)
    /// * `10g-epon` - 10G-EPON (10 Gbps)
    /// * `gpon` - GPON (2.5 Gbps / 1.25 Gbps)
    /// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
    /// * `xgs-pon` - XGS-PON (10 Gbps)
    /// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
    /// * `25g-pon` - 25G-PON (25 Gbps)
    /// * `50g-pon` - 50G-PON (50 Gbps)
    /// * `cisco-stackwise` - Cisco StackWise
    /// * `cisco-stackwise-plus` - Cisco StackWise Plus
    /// * `cisco-flexstack` - Cisco FlexStack
    /// * `cisco-flexstack-plus` - Cisco FlexStack Plus
    /// * `cisco-stackwise-80` - Cisco StackWise-80
    /// * `cisco-stackwise-160` - Cisco StackWise-160
    /// * `cisco-stackwise-320` - Cisco StackWise-320
    /// * `cisco-stackwise-480` - Cisco StackWise-480
    /// * `cisco-stackwise-1t` - Cisco StackWise-1T
    /// * `juniper-vcp` - Juniper VCP
    /// * `extreme-summitstack` - Extreme SummitStack
    /// * `extreme-summitstack-128` - Extreme SummitStack-128
    /// * `extreme-summitstack-256` - Extreme SummitStack-256
    /// * `extreme-summitstack-512` - Extreme SummitStack-512
    /// * `other` - Other
    #[arg(long = "type")]
    pub r#type: InterfaceType,

    #[arg(long)]
    pub vlan_translation_policy: Option<String>,

    #[arg(long)]
    pub vrf: Option<String>,

    #[arg(long)]
    pub wwn: Option<String>,

}

#[derive(Debug, Args)]
pub struct InterfaceUpdateFields {
    #[arg(long)]
    pub bridge: Option<u64>,

    #[arg(long)]
    pub description: Option<String>,

    /// * `half` - Half
    /// * `full` - Full
    /// * `auto` - Auto
    #[arg(long)]
    pub duplex: Option<InterfaceDuplex>,

    #[arg(long)]
    pub enabled: Option<bool>,

    /// Physical label
    #[arg(long)]
    pub label: Option<String>,

    /// Treat as if a cable is connected
    #[arg(long)]
    pub mark_connected: Option<bool>,

    /// This interface is used only for out-of-band management
    #[arg(long)]
    pub mgmt_only: Option<bool>,

    /// IEEE 802.1Q tagging strategy
    ///
    /// * `access` - Access
    /// * `tagged` - Tagged
    /// * `tagged-all` - Tagged (All)
    /// * `q-in-q` - Q-in-Q (802.1ad)
    #[arg(long)]
    pub mode: Option<InterfaceMode>,

    #[arg(long)]
    pub module: Option<String>,

    #[arg(long)]
    pub mtu: Option<u64>,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub parent: Option<u64>,

    /// * `pd` - PD
    /// * `pse` - PSE
    #[arg(long)]
    pub poe_mode: Option<InterfacePoeMode>,

    /// * `type1-ieee802.3af` - 802.3af (Type 1)
    /// * `type2-ieee802.3at` - 802.3at (Type 2)
    /// * `type3-ieee802.3bt` - 802.3bt (Type 3)
    /// * `type4-ieee802.3bt` - 802.3bt (Type 4)
    /// * `passive-24v-2pair` - Passive 24V (2-pair)
    /// * `passive-24v-4pair` - Passive 24V (4-pair)
    /// * `passive-48v-2pair` - Passive 48V (2-pair)
    /// * `passive-48v-4pair` - Passive 48V (4-pair)
    #[arg(long)]
    pub poe_type: Option<InterfacePoeType>,

    #[arg(long)]
    pub primary_mac_address: Option<String>,

    #[arg(long)]
    pub qinq_svlan: Option<String>,

    /// * `2.4g-1-2412-22` - 1 (2412 MHz)
    /// * `2.4g-2-2417-22` - 2 (2417 MHz)
    /// * `2.4g-3-2422-22` - 3 (2422 MHz)
    /// * `2.4g-4-2427-22` - 4 (2427 MHz)
    /// * `2.4g-5-2432-22` - 5 (2432 MHz)
    /// * `2.4g-6-2437-22` - 6 (2437 MHz)
    /// * `2.4g-7-2442-22` - 7 (2442 MHz)
    /// * `2.4g-8-2447-22` - 8 (2447 MHz)
    /// * `2.4g-9-2452-22` - 9 (2452 MHz)
    /// * `2.4g-10-2457-22` - 10 (2457 MHz)
    /// * `2.4g-11-2462-22` - 11 (2462 MHz)
    /// * `2.4g-12-2467-22` - 12 (2467 MHz)
    /// * `2.4g-13-2472-22` - 13 (2472 MHz)
    /// * `5g-32-5160-20` - 32 (5160/20 MHz)
    /// * `5g-34-5170-40` - 34 (5170/40 MHz)
    /// * `5g-36-5180-20` - 36 (5180/20 MHz)
    /// * `5g-38-5190-40` - 38 (5190/40 MHz)
    /// * `5g-40-5200-20` - 40 (5200/20 MHz)
    /// * `5g-42-5210-80` - 42 (5210/80 MHz)
    /// * `5g-44-5220-20` - 44 (5220/20 MHz)
    /// * `5g-46-5230-40` - 46 (5230/40 MHz)
    /// * `5g-48-5240-20` - 48 (5240/20 MHz)
    /// * `5g-50-5250-160` - 50 (5250/160 MHz)
    /// * `5g-52-5260-20` - 52 (5260/20 MHz)
    /// * `5g-54-5270-40` - 54 (5270/40 MHz)
    /// * `5g-56-5280-20` - 56 (5280/20 MHz)
    /// * `5g-58-5290-80` - 58 (5290/80 MHz)
    /// * `5g-60-5300-20` - 60 (5300/20 MHz)
    /// * `5g-62-5310-40` - 62 (5310/40 MHz)
    /// * `5g-64-5320-20` - 64 (5320/20 MHz)
    /// * `5g-100-5500-20` - 100 (5500/20 MHz)
    /// * `5g-102-5510-40` - 102 (5510/40 MHz)
    /// * `5g-104-5520-20` - 104 (5520/20 MHz)
    /// * `5g-106-5530-80` - 106 (5530/80 MHz)
    /// * `5g-108-5540-20` - 108 (5540/20 MHz)
    /// * `5g-110-5550-40` - 110 (5550/40 MHz)
    /// * `5g-112-5560-20` - 112 (5560/20 MHz)
    /// * `5g-114-5570-160` - 114 (5570/160 MHz)
    /// * `5g-116-5580-20` - 116 (5580/20 MHz)
    /// * `5g-118-5590-40` - 118 (5590/40 MHz)
    /// * `5g-120-5600-20` - 120 (5600/20 MHz)
    /// * `5g-122-5610-80` - 122 (5610/80 MHz)
    /// * `5g-124-5620-20` - 124 (5620/20 MHz)
    /// * `5g-126-5630-40` - 126 (5630/40 MHz)
    /// * `5g-128-5640-20` - 128 (5640/20 MHz)
    /// * `5g-132-5660-20` - 132 (5660/20 MHz)
    /// * `5g-134-5670-40` - 134 (5670/40 MHz)
    /// * `5g-136-5680-20` - 136 (5680/20 MHz)
    /// * `5g-138-5690-80` - 138 (5690/80 MHz)
    /// * `5g-140-5700-20` - 140 (5700/20 MHz)
    /// * `5g-142-5710-40` - 142 (5710/40 MHz)
    /// * `5g-144-5720-20` - 144 (5720/20 MHz)
    /// * `5g-149-5745-20` - 149 (5745/20 MHz)
    /// * `5g-151-5755-40` - 151 (5755/40 MHz)
    /// * `5g-153-5765-20` - 153 (5765/20 MHz)
    /// * `5g-155-5775-80` - 155 (5775/80 MHz)
    /// * `5g-157-5785-20` - 157 (5785/20 MHz)
    /// * `5g-159-5795-40` - 159 (5795/40 MHz)
    /// * `5g-161-5805-20` - 161 (5805/20 MHz)
    /// * `5g-163-5815-160` - 163 (5815/160 MHz)
    /// * `5g-165-5825-20` - 165 (5825/20 MHz)
    /// * `5g-167-5835-40` - 167 (5835/40 MHz)
    /// * `5g-169-5845-20` - 169 (5845/20 MHz)
    /// * `5g-171-5855-80` - 171 (5855/80 MHz)
    /// * `5g-173-5865-20` - 173 (5865/20 MHz)
    /// * `5g-175-5875-40` - 175 (5875/40 MHz)
    /// * `5g-177-5885-20` - 177 (5885/20 MHz)
    /// * `6g-1-5955-20` - 1 (5955/20 MHz)
    /// * `6g-3-5965-40` - 3 (5965/40 MHz)
    /// * `6g-5-5975-20` - 5 (5975/20 MHz)
    /// * `6g-7-5985-80` - 7 (5985/80 MHz)
    /// * `6g-9-5995-20` - 9 (5995/20 MHz)
    /// * `6g-11-6005-40` - 11 (6005/40 MHz)
    /// * `6g-13-6015-20` - 13 (6015/20 MHz)
    /// * `6g-15-6025-160` - 15 (6025/160 MHz)
    /// * `6g-17-6035-20` - 17 (6035/20 MHz)
    /// * `6g-19-6045-40` - 19 (6045/40 MHz)
    /// * `6g-21-6055-20` - 21 (6055/20 MHz)
    /// * `6g-23-6065-80` - 23 (6065/80 MHz)
    /// * `6g-25-6075-20` - 25 (6075/20 MHz)
    /// * `6g-27-6085-40` - 27 (6085/40 MHz)
    /// * `6g-29-6095-20` - 29 (6095/20 MHz)
    /// * `6g-31-6105-320` - 31 (6105/320 MHz)
    /// * `6g-33-6115-20` - 33 (6115/20 MHz)
    /// * `6g-35-6125-40` - 35 (6125/40 MHz)
    /// * `6g-37-6135-20` - 37 (6135/20 MHz)
    /// * `6g-39-6145-80` - 39 (6145/80 MHz)
    /// * `6g-41-6155-20` - 41 (6155/20 MHz)
    /// * `6g-43-6165-40` - 43 (6165/40 MHz)
    /// * `6g-45-6175-20` - 45 (6175/20 MHz)
    /// * `6g-47-6185-160` - 47 (6185/160 MHz)
    /// * `6g-49-6195-20` - 49 (6195/20 MHz)
    /// * `6g-51-6205-40` - 51 (6205/40 MHz)
    /// * `6g-53-6215-20` - 53 (6215/20 MHz)
    /// * `6g-55-6225-80` - 55 (6225/80 MHz)
    /// * `6g-57-6235-20` - 57 (6235/20 MHz)
    /// * `6g-59-6245-40` - 59 (6245/40 MHz)
    /// * `6g-61-6255-20` - 61 (6255/20 MHz)
    /// * `6g-65-6275-20` - 65 (6275/20 MHz)
    /// * `6g-67-6285-40` - 67 (6285/40 MHz)
    /// * `6g-69-6295-20` - 69 (6295/20 MHz)
    /// * `6g-71-6305-80` - 71 (6305/80 MHz)
    /// * `6g-73-6315-20` - 73 (6315/20 MHz)
    /// * `6g-75-6325-40` - 75 (6325/40 MHz)
    /// * `6g-77-6335-20` - 77 (6335/20 MHz)
    /// * `6g-79-6345-160` - 79 (6345/160 MHz)
    /// * `6g-81-6355-20` - 81 (6355/20 MHz)
    /// * `6g-83-6365-40` - 83 (6365/40 MHz)
    /// * `6g-85-6375-20` - 85 (6375/20 MHz)
    /// * `6g-87-6385-80` - 87 (6385/80 MHz)
    /// * `6g-89-6395-20` - 89 (6395/20 MHz)
    /// * `6g-91-6405-40` - 91 (6405/40 MHz)
    /// * `6g-93-6415-20` - 93 (6415/20 MHz)
    /// * `6g-95-6425-320` - 95 (6425/320 MHz)
    /// * `6g-97-6435-20` - 97 (6435/20 MHz)
    /// * `6g-99-6445-40` - 99 (6445/40 MHz)
    /// * `6g-101-6455-20` - 101 (6455/20 MHz)
    /// * `6g-103-6465-80` - 103 (6465/80 MHz)
    /// * `6g-105-6475-20` - 105 (6475/20 MHz)
    /// * `6g-107-6485-40` - 107 (6485/40 MHz)
    /// * `6g-109-6495-20` - 109 (6495/20 MHz)
    /// * `6g-111-6505-160` - 111 (6505/160 MHz)
    /// * `6g-113-6515-20` - 113 (6515/20 MHz)
    /// * `6g-115-6525-40` - 115 (6525/40 MHz)
    /// * `6g-117-6535-20` - 117 (6535/20 MHz)
    /// * `6g-119-6545-80` - 119 (6545/80 MHz)
    /// * `6g-121-6555-20` - 121 (6555/20 MHz)
    /// * `6g-123-6565-40` - 123 (6565/40 MHz)
    /// * `6g-125-6575-20` - 125 (6575/20 MHz)
    /// * `6g-129-6595-20` - 129 (6595/20 MHz)
    /// * `6g-131-6605-40` - 131 (6605/40 MHz)
    /// * `6g-133-6615-20` - 133 (6615/20 MHz)
    /// * `6g-135-6625-80` - 135 (6625/80 MHz)
    /// * `6g-137-6635-20` - 137 (6635/20 MHz)
    /// * `6g-139-6645-40` - 139 (6645/40 MHz)
    /// * `6g-141-6655-20` - 141 (6655/20 MHz)
    /// * `6g-143-6665-160` - 143 (6665/160 MHz)
    /// * `6g-145-6675-20` - 145 (6675/20 MHz)
    /// * `6g-147-6685-40` - 147 (6685/40 MHz)
    /// * `6g-149-6695-20` - 149 (6695/20 MHz)
    /// * `6g-151-6705-80` - 151 (6705/80 MHz)
    /// * `6g-153-6715-20` - 153 (6715/20 MHz)
    /// * `6g-155-6725-40` - 155 (6725/40 MHz)
    /// * `6g-157-6735-20` - 157 (6735/20 MHz)
    /// * `6g-159-6745-320` - 159 (6745/320 MHz)
    /// * `6g-161-6755-20` - 161 (6755/20 MHz)
    /// * `6g-163-6765-40` - 163 (6765/40 MHz)
    /// * `6g-165-6775-20` - 165 (6775/20 MHz)
    /// * `6g-167-6785-80` - 167 (6785/80 MHz)
    /// * `6g-169-6795-20` - 169 (6795/20 MHz)
    /// * `6g-171-6805-40` - 171 (6805/40 MHz)
    /// * `6g-173-6815-20` - 173 (6815/20 MHz)
    /// * `6g-175-6825-160` - 175 (6825/160 MHz)
    /// * `6g-177-6835-20` - 177 (6835/20 MHz)
    /// * `6g-179-6845-40` - 179 (6845/40 MHz)
    /// * `6g-181-6855-20` - 181 (6855/20 MHz)
    /// * `6g-183-6865-80` - 183 (6865/80 MHz)
    /// * `6g-185-6875-20` - 185 (6875/20 MHz)
    /// * `6g-187-6885-40` - 187 (6885/40 MHz)
    /// * `6g-189-6895-20` - 189 (6895/20 MHz)
    /// * `6g-193-6915-20` - 193 (6915/20 MHz)
    /// * `6g-195-6925-40` - 195 (6925/40 MHz)
    /// * `6g-197-6935-20` - 197 (6935/20 MHz)
    /// * `6g-199-6945-80` - 199 (6945/80 MHz)
    /// * `6g-201-6955-20` - 201 (6955/20 MHz)
    /// * `6g-203-6965-40` - 203 (6965/40 MHz)
    /// * `6g-205-6975-20` - 205 (6975/20 MHz)
    /// * `6g-207-6985-160` - 207 (6985/160 MHz)
    /// * `6g-209-6995-20` - 209 (6995/20 MHz)
    /// * `6g-211-7005-40` - 211 (7005/40 MHz)
    /// * `6g-213-7015-20` - 213 (7015/20 MHz)
    /// * `6g-215-7025-80` - 215 (7025/80 MHz)
    /// * `6g-217-7035-20` - 217 (7035/20 MHz)
    /// * `6g-219-7045-40` - 219 (7045/40 MHz)
    /// * `6g-221-7055-20` - 221 (7055/20 MHz)
    /// * `6g-225-7075-20` - 225 (7075/20 MHz)
    /// * `6g-227-7085-40` - 227 (7085/40 MHz)
    /// * `6g-229-7095-20` - 229 (7095/20 MHz)
    /// * `6g-233-7115-20` - 233 (7115/20 MHz)
    /// * `60g-1-58320-2160` - 1 (58.32/2.16 GHz)
    /// * `60g-2-60480-2160` - 2 (60.48/2.16 GHz)
    /// * `60g-3-62640-2160` - 3 (62.64/2.16 GHz)
    /// * `60g-4-64800-2160` - 4 (64.80/2.16 GHz)
    /// * `60g-5-66960-2160` - 5 (66.96/2.16 GHz)
    /// * `60g-6-69120-2160` - 6 (69.12/2.16 GHz)
    /// * `60g-9-59400-4320` - 9 (59.40/4.32 GHz)
    /// * `60g-10-61560-4320` - 10 (61.56/4.32 GHz)
    /// * `60g-11-63720-4320` - 11 (63.72/4.32 GHz)
    /// * `60g-12-65880-4320` - 12 (65.88/4.32 GHz)
    /// * `60g-13-68040-4320` - 13 (68.04/4.32 GHz)
    /// * `60g-17-60480-6480` - 17 (60.48/6.48 GHz)
    /// * `60g-18-62640-6480` - 18 (62.64/6.48 GHz)
    /// * `60g-19-64800-6480` - 19 (64.80/6.48 GHz)
    /// * `60g-20-66960-6480` - 20 (66.96/6.48 GHz)
    /// * `60g-25-61560-6480` - 25 (61.56/8.64 GHz)
    /// * `60g-26-63720-6480` - 26 (63.72/8.64 GHz)
    /// * `60g-27-65880-6480` - 27 (65.88/8.64 GHz)
    #[arg(long)]
    pub rf_channel: Option<InterfaceRfChannel>,

    /// Populated by selected channel (if set)
    #[arg(long)]
    pub rf_channel_frequency: Option<f64>,

    /// Populated by selected channel (if set)
    #[arg(long)]
    pub rf_channel_width: Option<f64>,

    /// * `ap` - Access point
    /// * `station` - Station
    #[arg(long)]
    pub rf_role: Option<InterfaceRfRole>,

    #[arg(long)]
    pub speed: Option<u64>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tx_power: Option<u64>,

    /// * `virtual` - Virtual
    /// * `bridge` - Bridge
    /// * `lag` - Link Aggregation Group (LAG)
    /// * `100base-fx` - 100BASE-FX (10/100ME)
    /// * `100base-lfx` - 100BASE-LFX (10/100ME)
    /// * `100base-tx` - 100BASE-TX (10/100ME)
    /// * `100base-t1` - 100BASE-T1 (10/100ME)
    /// * `1000base-bx10-d` - 1000BASE-BX10-D (1GE BiDi Down)
    /// * `1000base-bx10-u` - 1000BASE-BX10-U (1GE BiDi Up)
    /// * `1000base-cwdm` - 1000BASE-CWDM (1GE)
    /// * `1000base-cx` - 1000BASE-CX (1GE DAC)
    /// * `1000base-dwdm` - 1000BASE-DWDM (1GE)
    /// * `1000base-ex` - 1000BASE-EX (1GE)
    /// * `1000base-lsx` - 1000BASE-LSX (1GE)
    /// * `1000base-lx` - 1000BASE-LX (1GE)
    /// * `1000base-lx10` - 1000BASE-LX10/LH (1GE)
    /// * `1000base-sx` - 1000BASE-SX (1GE)
    /// * `1000base-t` - 1000BASE-T (1GE)
    /// * `1000base-tx` - 1000BASE-TX (1GE)
    /// * `1000base-zx` - 1000BASE-ZX (1GE)
    /// * `2.5gbase-t` - 2.5GBASE-T (2.5GE)
    /// * `5gbase-t` - 5GBASE-T (5GE)
    /// * `10gbase-br-d` - 10GBASE-BR-D (10GE BiDi Down)
    /// * `10gbase-br-u` - 10GBASE-BR-U (10GE BiDi Up)
    /// * `10gbase-cu` - 10GBASE-CU (10GE DAC Passive Twinax)
    /// * `10gbase-cx4` - 10GBASE-CX4 (10GE DAC)
    /// * `10gbase-er` - 10GBASE-ER (10GE)
    /// * `10gbase-lr` - 10GBASE-LR (10GE)
    /// * `10gbase-lrm` - 10GBASE-LRM (10GE)
    /// * `10gbase-lx4` - 10GBASE-LX4 (10GE)
    /// * `10gbase-sr` - 10GBASE-SR (10GE)
    /// * `10gbase-t` - 10GBASE-T (10GE)
    /// * `10gbase-zr` - 10GBASE-ZR (10GE)
    /// * `25gbase-cr` - 25GBASE-CR (25GE DAC)
    /// * `25gbase-er` - 25GBASE-ER (25GE)
    /// * `25gbase-lr` - 25GBASE-LR (25GE)
    /// * `25gbase-sr` - 25GBASE-SR (25GE)
    /// * `25gbase-t` - 25GBASE-T (25GE)
    /// * `40gbase-cr4` - 40GBASE-CR4 (40GE DAC)
    /// * `40gbase-er4` - 40GBASE-ER4 (40GE)
    /// * `40gbase-fr4` - 40GBASE-FR4 (40GE)
    /// * `40gbase-lr4` - 40GBASE-LR4 (40GE)
    /// * `40gbase-sr4` - 40GBASE-SR4 (40GE)
    /// * `40gbase-sr4-bd` - 40GBASE-SR4 (40GE BiDi)
    /// * `50gbase-cr` - 50GBASE-CR (50GE DAC)
    /// * `50gbase-er` - 50GBASE-ER (50GE)
    /// * `50gbase-fr` - 50GBASE-FR (50GE)
    /// * `50gbase-lr` - 50GBASE-LR (50GE)
    /// * `50gbase-sr` - 50GBASE-SR (50GE)
    /// * `100gbase-cr1` - 100GBASE-CR1 (100GE DAC)
    /// * `100gbase-cr2` - 100GBASE-CR2 (100GE DAC)
    /// * `100gbase-cr4` - 100GBASE-CR4 (100GE DAC)
    /// * `100gbase-cr10` - 100GBASE-CR10 (100GE DAC)
    /// * `100gbase-cwdm4` - 100GBASE-CWDM4 (100GE)
    /// * `100gbase-dr` - 100GBASE-DR (100GE)
    /// * `100gbase-er4` - 100GBASE-ER4 (100GE)
    /// * `100gbase-fr1` - 100GBASE-FR1 (100GE)
    /// * `100gbase-lr1` - 100GBASE-LR1 (100GE)
    /// * `100gbase-lr4` - 100GBASE-LR4 (100GE)
    /// * `100gbase-sr1` - 100GBASE-SR1 (100GE)
    /// * `100gbase-sr1.2` - 100GBASE-SR1.2 (100GE BiDi)
    /// * `100gbase-sr2` - 100GBASE-SR2 (100GE)
    /// * `100gbase-sr4` - 100GBASE-SR4 (100GE)
    /// * `100gbase-sr10` - 100GBASE-SR10 (100GE)
    /// * `100gbase-zr` - 100GBASE-ZR (100GE)
    /// * `200gbase-cr2` - 200GBASE-CR2 (200GE)
    /// * `200gbase-cr4` - 200GBASE-CR4 (200GE)
    /// * `200gbase-dr4` - 200GBASE-DR4 (200GE)
    /// * `200gbase-er4` - 200GBASE-ER4 (200GE)
    /// * `200gbase-fr4` - 200GBASE-FR4 (200GE)
    /// * `200gbase-lr4` - 200GBASE-LR4 (200GE)
    /// * `200gbase-sr2` - 200GBASE-SR2 (200GE)
    /// * `200gbase-sr4` - 200GBASE-SR4 (200GE)
    /// * `200gbase-vr2` - 200GBASE-VR2 (200GE)
    /// * `400gbase-cr4` - 400GBASE-CR4 (400GE)
    /// * `400gbase-dr4` - 400GBASE-DR4 (400GE)
    /// * `400gbase-er8` - 400GBASE-ER8 (400GE)
    /// * `400gbase-fr4` - 400GBASE-FR4 (400GE)
    /// * `400gbase-fr8` - 400GBASE-FR8 (400GE)
    /// * `400gbase-lr4` - 400GBASE-LR4 (400GE)
    /// * `400gbase-lr8` - 400GBASE-LR8 (400GE)
    /// * `400gbase-sr4` - 400GBASE-SR4 (400GE)
    /// * `400gbase-sr4_2` - 400GBASE-SR4.2 (400GE BiDi)
    /// * `400gbase-sr8` - 400GBASE-SR8 (400GE)
    /// * `400gbase-sr16` - 400GBASE-SR16 (400GE)
    /// * `400gbase-vr4` - 400GBASE-VR4 (400GE)
    /// * `400gbase-zr` - 400GBASE-ZR (400GE)
    /// * `800gbase-cr8` - 800GBASE-CR8 (800GE)
    /// * `800gbase-dr8` - 800GBASE-DR8 (800GE)
    /// * `800gbase-sr8` - 800GBASE-SR8 (800GE)
    /// * `800gbase-vr8` - 800GBASE-VR8 (800GE)
    /// * `1.6tbase-cr8` - 1.6TBASE-CR8 (1.6TE)
    /// * `1.6tbase-dr8` - 1.6TBASE-DR8 (1.6TE)
    /// * `1.6tbase-dr8-2` - 1.6TBASE-DR8-2 (1.6TE)
    /// * `100base-x-sfp` - SFP (100ME)
    /// * `1000base-x-gbic` - GBIC (1GE)
    /// * `1000base-x-sfp` - SFP (1GE)
    /// * `2.5gbase-x-sfp` - SFP (2.5GE)
    /// * `10gbase-x-sfpp` - SFP+ (10GE)
    /// * `10gbase-x-xenpak` - XENPAK (10GE)
    /// * `10gbase-x-xfp` - XFP (10GE)
    /// * `10gbase-x-x2` - X2 (10GE)
    /// * `25gbase-x-sfp28` - SFP28 (25GE)
    /// * `40gbase-x-qsfpp` - QSFP+ (40GE)
    /// * `50gbase-x-sfp28` - QSFP28 (50GE)
    /// * `50gbase-x-sfp56` - SFP56 (50GE)
    /// * `100gbase-x-cfp` - CFP (100GE)
    /// * `100gbase-x-cfp2` - CFP2 (100GE)
    /// * `100gbase-x-cfp4` - CFP4 (100GE)
    /// * `100gbase-x-cxp` - CXP (100GE)
    /// * `100gbase-x-cpak` - Cisco CPAK (100GE)
    /// * `100gbase-x-dsfp` - DSFP (100GE)
    /// * `100gbase-x-qsfp28` - QSFP28 (100GE)
    /// * `100gbase-x-qsfpdd` - QSFP-DD (100GE)
    /// * `100gbase-x-sfpdd` - SFP-DD (100GE)
    /// * `200gbase-x-cfp2` - CFP2 (200GE)
    /// * `200gbase-x-qsfp56` - QSFP56 (200GE)
    /// * `200gbase-x-qsfpdd` - QSFP-DD (200GE)
    /// * `400gbase-x-qsfp112` - QSFP112 (400GE)
    /// * `400gbase-x-qsfpdd` - QSFP-DD (400GE)
    /// * `400gbase-x-cdfp` - CDFP (400GE)
    /// * `400gbase-x-cfp2` - CFP2 (400GE)
    /// * `400gbase-x-cfp8` - CPF8 (400GE)
    /// * `400gbase-x-osfp` - OSFP (400GE)
    /// * `400gbase-x-osfp-rhs` - OSFP-RHS (400GE)
    /// * `800gbase-x-osfp` - OSFP (800GE)
    /// * `800gbase-x-qsfpdd` - QSFP-DD (800GE)
    /// * `1.6tbase-x-osfp1600` - OSFP1600 (1.6TE)
    /// * `1.6tbase-x-osfp1600-rhs` - OSFP1600-RHS (1.6TE)
    /// * `1.6tbase-x-qsfpdd1600` - QSFP-DD1600 (1.6TE)
    /// * `1000base-kx` - 1000BASE-KX (1GE)
    /// * `2.5gbase-kx` - 2.5GBASE-KX (2.5GE)
    /// * `5gbase-kr` - 5GBASE-KR (5GE)
    /// * `10gbase-kr` - 10GBASE-KR (10GE)
    /// * `10gbase-kx4` - 10GBASE-KX4 (10GE)
    /// * `25gbase-kr` - 25GBASE-KR (25GE)
    /// * `40gbase-kr4` - 40GBASE-KR4 (40GE)
    /// * `50gbase-kr` - 50GBASE-KR (50GE)
    /// * `100gbase-kp4` - 100GBASE-KP4 (100GE)
    /// * `100gbase-kr2` - 100GBASE-KR2 (100GE)
    /// * `100gbase-kr4` - 100GBASE-KR4 (100GE)
    /// * `1.6tbase-kr8` - 1.6TBASE-KR8 (1.6TE)
    /// * `ieee802.11a` - IEEE 802.11a
    /// * `ieee802.11g` - IEEE 802.11b/g
    /// * `ieee802.11n` - IEEE 802.11n (Wi-Fi 4)
    /// * `ieee802.11ac` - IEEE 802.11ac (Wi-Fi 5)
    /// * `ieee802.11ad` - IEEE 802.11ad (WiGig)
    /// * `ieee802.11ax` - IEEE 802.11ax (Wi-Fi 6)
    /// * `ieee802.11ay` - IEEE 802.11ay (WiGig)
    /// * `ieee802.11be` - IEEE 802.11be (Wi-Fi 7)
    /// * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth)
    /// * `ieee802.15.4` - IEEE 802.15.4 (LR-WPAN)
    /// * `other-wireless` - Other (Wireless)
    /// * `gsm` - GSM
    /// * `cdma` - CDMA
    /// * `lte` - LTE
    /// * `4g` - 4G
    /// * `5g` - 5G
    /// * `sonet-oc3` - OC-3/STM-1
    /// * `sonet-oc12` - OC-12/STM-4
    /// * `sonet-oc48` - OC-48/STM-16
    /// * `sonet-oc192` - OC-192/STM-64
    /// * `sonet-oc768` - OC-768/STM-256
    /// * `sonet-oc1920` - OC-1920/STM-640
    /// * `sonet-oc3840` - OC-3840/STM-1234
    /// * `1gfc-sfp` - SFP (1GFC)
    /// * `2gfc-sfp` - SFP (2GFC)
    /// * `4gfc-sfp` - SFP (4GFC)
    /// * `8gfc-sfpp` - SFP+ (8GFC)
    /// * `16gfc-sfpp` - SFP+ (16GFC)
    /// * `32gfc-sfp28` - SFP28 (32GFC)
    /// * `32gfc-sfpp` - SFP+ (32GFC)
    /// * `64gfc-qsfpp` - QSFP+ (64GFC)
    /// * `64gfc-sfpdd` - SFP-DD (64GFC)
    /// * `64gfc-sfpp` - SFP+ (64GFC)
    /// * `128gfc-qsfp28` - QSFP28 (128GFC)
    /// * `infiniband-sdr` - SDR (2 Gbps)
    /// * `infiniband-ddr` - DDR (4 Gbps)
    /// * `infiniband-qdr` - QDR (8 Gbps)
    /// * `infiniband-fdr10` - FDR10 (10 Gbps)
    /// * `infiniband-fdr` - FDR (13.5 Gbps)
    /// * `infiniband-edr` - EDR (25 Gbps)
    /// * `infiniband-hdr` - HDR (50 Gbps)
    /// * `infiniband-ndr` - NDR (100 Gbps)
    /// * `infiniband-xdr` - XDR (250 Gbps)
    /// * `t1` - T1 (1.544 Mbps)
    /// * `e1` - E1 (2.048 Mbps)
    /// * `t3` - T3 (45 Mbps)
    /// * `e3` - E3 (34 Mbps)
    /// * `xdsl` - xDSL
    /// * `docsis` - DOCSIS
    /// * `moca` - MoCA
    /// * `bpon` - BPON (622 Mbps / 155 Mbps)
    /// * `epon` - EPON (1 Gbps)
    /// * `10g-epon` - 10G-EPON (10 Gbps)
    /// * `gpon` - GPON (2.5 Gbps / 1.25 Gbps)
    /// * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps)
    /// * `xgs-pon` - XGS-PON (10 Gbps)
    /// * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps)
    /// * `25g-pon` - 25G-PON (25 Gbps)
    /// * `50g-pon` - 50G-PON (50 Gbps)
    /// * `cisco-stackwise` - Cisco StackWise
    /// * `cisco-stackwise-plus` - Cisco StackWise Plus
    /// * `cisco-flexstack` - Cisco FlexStack
    /// * `cisco-flexstack-plus` - Cisco FlexStack Plus
    /// * `cisco-stackwise-80` - Cisco StackWise-80
    /// * `cisco-stackwise-160` - Cisco StackWise-160
    /// * `cisco-stackwise-320` - Cisco StackWise-320
    /// * `cisco-stackwise-480` - Cisco StackWise-480
    /// * `cisco-stackwise-1t` - Cisco StackWise-1T
    /// * `juniper-vcp` - Juniper VCP
    /// * `extreme-summitstack` - Extreme SummitStack
    /// * `extreme-summitstack-128` - Extreme SummitStack-128
    /// * `extreme-summitstack-256` - Extreme SummitStack-256
    /// * `extreme-summitstack-512` - Extreme SummitStack-512
    /// * `other` - Other
    #[arg(long = "type")]
    pub r#type: Option<InterfaceType>,

    #[arg(long)]
    pub vlan_translation_policy: Option<String>,

    #[arg(long)]
    pub vrf: Option<String>,

    #[arg(long)]
    pub wwn: Option<String>,

}

pub async fn create_body(args: InterfaceCreateFields, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = json!({});
    let object: &mut Map<String, Value> = body.as_object_mut()
        .expect("json!({}) always returns an object");
    if let Some(v) = args.bridge {
        object.insert("bridge".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "description", args.description);
    if let Some(v) = args.duplex {
        object.insert("duplex".to_owned(), json!(v));
    }
    insert_optional_bool_field(object, "enabled", args.enabled);
    insert_optional_string_field(object, "label", args.label);
    insert_optional_bool_field(object, "mark_connected", args.mark_connected);
    insert_optional_bool_field(object, "mgmt_only", args.mgmt_only);
    if let Some(v) = args.mode {
        object.insert("mode".to_owned(), json!(v));
    }
    if let Some(v) = args.module {
        object.insert("module".to_owned(), crate::commands::resolve_reference_id("/api/dcim/modules/", "serial", &v, opts).await?);
    }
    if let Some(v) = args.mtu {
        object.insert("mtu".to_owned(), json!(v));
    }
    object.insert("name".to_owned(), Value::String(args.name));
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.parent {
        object.insert("parent".to_owned(), json!(v));
    }
    if let Some(v) = args.poe_mode {
        object.insert("poe_mode".to_owned(), json!(v));
    }
    if let Some(v) = args.poe_type {
        object.insert("poe_type".to_owned(), json!(v));
    }
    if let Some(v) = args.primary_mac_address {
        object.insert("primary_mac_address".to_owned(), crate::commands::resolve_reference_id("/api/dcim/mac-addresses/", "mac_address", &v, opts).await?);
    }
    if let Some(v) = args.qinq_svlan {
        object.insert("qinq_svlan".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vlans/", "name", &v, opts).await?);
    }
    if let Some(v) = args.rf_channel {
        object.insert("rf_channel".to_owned(), json!(v));
    }
    if let Some(v) = args.rf_channel_frequency {
        object.insert("rf_channel_frequency".to_owned(), json!(v));
    }
    if let Some(v) = args.rf_channel_width {
        object.insert("rf_channel_width".to_owned(), json!(v));
    }
    if let Some(v) = args.rf_role {
        object.insert("rf_role".to_owned(), json!(v));
    }
    if let Some(v) = args.speed {
        object.insert("speed".to_owned(), json!(v));
    }
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    if let Some(v) = args.tx_power {
        object.insert("tx_power".to_owned(), json!(v));
    }
    object.insert("type".to_owned(), json!(args.r#type));
    if let Some(v) = args.vlan_translation_policy {
        object.insert("vlan_translation_policy".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vlan-translation-policies/", "name", &v, opts).await?);
    }
    if let Some(v) = args.vrf {
        object.insert("vrf".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vrfs/", "name", &v, opts).await?);
    }
    insert_optional_string_field(object, "wwn", args.wwn);
    Ok(body)
}

pub async fn update_body(args: InterfaceUpdateFields, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = json!({});
    let object: &mut Map<String, Value> = body.as_object_mut()
        .expect("json!({}) always returns an object");
    if let Some(v) = args.bridge {
        object.insert("bridge".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "description", args.description);
    if let Some(v) = args.duplex {
        object.insert("duplex".to_owned(), json!(v));
    }
    insert_optional_bool_field(object, "enabled", args.enabled);
    insert_optional_string_field(object, "label", args.label);
    insert_optional_bool_field(object, "mark_connected", args.mark_connected);
    insert_optional_bool_field(object, "mgmt_only", args.mgmt_only);
    if let Some(v) = args.mode {
        object.insert("mode".to_owned(), json!(v));
    }
    if let Some(v) = args.module {
        object.insert("module".to_owned(), crate::commands::resolve_reference_id("/api/dcim/modules/", "serial", &v, opts).await?);
    }
    if let Some(v) = args.mtu {
        object.insert("mtu".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "name", args.name);
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.parent {
        object.insert("parent".to_owned(), json!(v));
    }
    if let Some(v) = args.poe_mode {
        object.insert("poe_mode".to_owned(), json!(v));
    }
    if let Some(v) = args.poe_type {
        object.insert("poe_type".to_owned(), json!(v));
    }
    if let Some(v) = args.primary_mac_address {
        object.insert("primary_mac_address".to_owned(), crate::commands::resolve_reference_id("/api/dcim/mac-addresses/", "mac_address", &v, opts).await?);
    }
    if let Some(v) = args.qinq_svlan {
        object.insert("qinq_svlan".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vlans/", "name", &v, opts).await?);
    }
    if let Some(v) = args.rf_channel {
        object.insert("rf_channel".to_owned(), json!(v));
    }
    if let Some(v) = args.rf_channel_frequency {
        object.insert("rf_channel_frequency".to_owned(), json!(v));
    }
    if let Some(v) = args.rf_channel_width {
        object.insert("rf_channel_width".to_owned(), json!(v));
    }
    if let Some(v) = args.rf_role {
        object.insert("rf_role".to_owned(), json!(v));
    }
    if let Some(v) = args.speed {
        object.insert("speed".to_owned(), json!(v));
    }
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    if let Some(v) = args.tx_power {
        object.insert("tx_power".to_owned(), json!(v));
    }
    if let Some(v) = args.r#type {
        object.insert("type".to_owned(), json!(v));
    }
    if let Some(v) = args.vlan_translation_policy {
        object.insert("vlan_translation_policy".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vlan-translation-policies/", "name", &v, opts).await?);
    }
    if let Some(v) = args.vrf {
        object.insert("vrf".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vrfs/", "name", &v, opts).await?);
    }
    insert_optional_string_field(object, "wwn", args.wwn);
    Ok(body)
}

/// Try to deserialize a successful NetBox response into the typify-generated
/// response struct. Used by dispatchers to detect schema drift at runtime;
/// on parse failure, nbx emits a one-line stderr warning. Set the env var
/// `NBX_SKIP_RESPONSE_VALIDATION=1` to silence the check.
pub fn validate_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<crate::generated::types::Interface>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::Interface>>(value.clone()).map(|_| ())
}

