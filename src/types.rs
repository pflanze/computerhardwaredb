use anyhow::{bail, Context, anyhow};
use serde::{Deserialize, Serialize};

use crate::set::Set;
use crate::{value::Value, date::Date, index::PrimaryKey};

use crate::{def_name_type, set};

def_name_type!{ArticleName}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    DDR4,
    DDR5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemorySubtype {
    UDIMM,
    RDIMM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtensionBus {
    /// version, lanes
    PCIe(f32, Value<u16>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoolerType {
    LiquidRecommended, // "Liquid cooler recommended for optimal performance"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bytes {
    KB(u32),
    MB(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watt(pub u16);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GHz(pub f32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTperSec(pub u32);


#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum CPUSocket {
    // AMD:
    AM4,
    AM5,
    SWRX8, // really sWRX8
    SP3,
    SP5, // LGA-6096
    SP6,
    STR5,

    // Intel:
    FCLGA3647,
}

impl TryFrom<&str> for CPUSocket {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use CPUSocket::*;
        match value {
            "AM4" => Ok(AM4),
            "AM5" => Ok(AM5),
            "sWRX8" => Ok(SWRX8),
            "SP3" => Ok(SP3),
            "SP5" => Ok(SP5),
            "SP6" => Ok(SP6),
            "sTR5" => Ok(STR5),
            "FCLGA3647" => Ok(FCLGA3647),
            _ => bail!("invalid CPU Socket name {value:?}")
        }
    }
}

// Why is this now needed, thought it went automatically?
impl TryFrom<&str> for Value<CPUSocket> {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Value::Some(value.try_into()?))
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphicsModel {
    None, // Discrete Graphics Card Required
    Radeon,
}


#[derive(Hash, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum Architecture {
    Zen1, // or just "Zen"? But that would confuse us users now.
    Zen2,
    Zen3,
    Zen4,
    Zen4c,
    Zen5,
    Infinity,
}

impl TryFrom<&str> for Architecture {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Architecture::*;
        match value {
            "Zen 2" => Ok(Zen2),
            "Zen 3" | "AMD \"Zen 3\" Core Architecture" => Ok(Zen3),
            "Zen 4" => Ok(Zen4),
            "Zen 4c" => Ok(Zen4c),
            "Zen 5" => Ok(Zen5),
            "AMD Infinity Architecture" => Ok(Infinity),
            _ => bail!("invalid Architecture {value:?}")
        }
    }
}

// Why is this now needed, thought it went automatically?
impl TryFrom<&str> for Value<Architecture> {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Value::Some(value.try_into()?))
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPU {
    pub name: ArticleName, // AMD Ryzen™ 9 5950X
    pub url: String, // https://www.amd.com/en/product/10456
    pub market_segment: Value<MarketSegment>,
    pub product_line: Value<ProductLine>, // "AMD Ryzen™ 9 Desktop Processors"
    pub architecture: Value<Architecture>,
    pub desc: String, // optional
    pub cores: Value<u8>,
    pub threads: Value<u8>,
    pub l1cache: Value<Bytes>,
    pub l2cache: Value<Bytes>,
    pub l3cache: Value<Bytes>,
    pub tdp: Value<Watt>,
    pub base_clock: Value<GHz>,
    pub max_boost_clock: Value<GHz>,
    pub cooler: Value<CoolerType>,
    pub launch_date: Value<Date>,
    pub cpu_socket: Value<CPUSocket>,
    pub memory_channels: Value<u8>,
    pub pci_express_version: Value<ExtensionBus>,
    pub system_memory_type: Value<MemoryType>,
    pub system_memory_subtype: Value<MemorySubtype>,
    pub system_memory_specification: Value<MTperSec>,
    pub ecc_support: Value<bool>,
    pub graphics_model: Value<GraphicsModel>,
    // "Security, Virtualization, and Manageability"
    pub pcie_dma_security: Value<bool>,
    pub usb_dma_security: Value<bool>,
    pub amd_enhanced_virus_protection_nx_bit: Value<bool>, // just nx_bit  ? 
}

impl PrimaryKey<ArticleName> for CPU {
    fn primary_key(&self) -> &ArticleName {
        &self.name
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shop {
    Digitec,
    Brack,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Price {
    CHF(u16), // leave out the fractional part
}

impl Price {
    pub fn chf(self) -> u16 {
        match self {
            Price::CHF(v) => v
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoldAt {
    pub article_name: ArticleName, // foreign key!
    pub desc: String, // for double-checking, "AMD Ryzen 9 Pro 7945 Tray Version AM5, 3.70 GHz, 12 -Core"
    pub shop: Shop,
    pub url: String, // optional
    pub price: Price,
    pub is_tray_version: bool,
    pub is_used: bool,
    pub delivered: String, // "Delivered Wed 3.4. Only 1 item in stock"
}

impl PrimaryKey<String> for SoldAt {
    fn primary_key(&self) -> &String {
        &self.url
    }
}

impl TryFrom<&str> for Value<Date> {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Value::Some(value.try_into()?))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Usage {
    Mobile,
    Desktop,
    ServerOrEmbedded,
    // unused?:
    Server,
    Embedded,
}

impl TryFrom<&str> for Usage {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Usage::*;
        match value {
            "Mobile" => Ok(Mobile),
            "Desktop" => Ok(Desktop),
            "ServerOrEmbeddedr" => Ok(ServerOrEmbedded),
            "Server" => Ok(Server),
            "Embedded" => Ok(Embedded),
            _ => bail!("invalid Usage string {value:?}")
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Brand {
    Ryzen9,
    RyzenThreadripperPRO5000WX, // "AMD Ryzen™ Threadripper™ PRO 5000 WX-Series"
    RyzenThreadripper, // is brand useless anyway? Product line

    // https://en.wikipedia.org/wiki/Epyc (Predecessor(s) Opteron)
    // "Epyc CPUs use a multi-chip module design to enable higher
    // yields for a CPU than traditional monolithic dies. First
    // generation Epyc CPUs are composed of four 14 nm compute dies,
    // each with up to 8 cores.[20][21] Cores are symmetrically
    // disabled on dies to create lower binned products with fewer
    // cores but the same I/O and memory footprint. Second and Third
    // gen Epyc CPUs are composed of eight compute dies ...  Third gen
    // Milan-X CPUs use advanced through-silicon-vias to stack an
    // additional die on top of each of the 8 compute dies, adding 64
    // MB of L3 cache per die.  Epyc CPUs supports both single socket
    // and dual socket operation. In a dual socket configuration, 64
    // PCIe lanes from each CPU are allocated to AMD's proprietary
    // Infinity Fabric interconnect to allow for full bandwidth
    // between both CPUs.[24] Thus, a dual socket configuration has
    // the same number of usable PCIe lanes as a single socket
    // configuration."
    EPYC7001,
    // ^ "March 2017, AMD announced plans to re-enter the server
    // market ... ; June 2017 .. up to 32 cores per socket"
    // https://en.wikipedia.org/wiki/Epyc
    EPYC7002,
    // ^ "in August 2019, the Epyc 7002 "Rome" series
    // processors, based on the Zen 2 microarchitecture, launched,
    // doubling the core count per socket to 64, and increasing
    // per-core performance dramatically over the last generation
    // architecture." https://en.wikipedia.org/wiki/Epyc
    EPYC7003,
    // ^ "March 2021 .. "Milan" series, based on the Zen 3
    // microarchitecture.[4] Epyc Milan brought the same 64 cores as
    // Epyc Rome, but with much higher per-core performance, with the
    // Epyc 7763 beating the Epyc 7702 by up to 22 percent despite
    // having the same number of cores and threads."
    // https://en.wikipedia.org/wiki/Epyc

    EPYC8004, // Genoa, Bergamo and Siena

    // ^ "November 10, 2022, AMD launched the fourth generation of
    // Epyc server and data center processors based on the Zen 4
    // microarchitecture, codenamed Genoa. .. Genoa features between
    // 16 and 96 cores with support for PCIe 5.0 and DDR5. There was
    // also an emphasis by AMD on Genoa's energy efficiency, which
    // according to AMD CEO Lisa Su, means \"lower total cost of
    // ownership\" for enterprise and cloud datacenter clients.[50]
    // Genoa uses AMD's new SP5 (LGA 6096) socket.[51]"

    // "On June 13, 2023, AMD introduced Genoa-X with 3D V-Cache
    // technology for technical computing performance and Bergamo
    // (9734, 9754 and 9754S) for cloud native computing.[52]"

    // "On September 18, 2023, AMD introduced the low power Siena
    // lineup of processors, based on the Zen 4c
    // microarchitecture. Siena supports up to 64 cores on the new SP6
    // socket, which is currently only used by Siena processors. Siena
    // uses the same I/O die as Bergamo, however certain features,
    // such as dual socket support, are removed, and other features
    // are reduced, such as the change from 12 channel memory support
    // to 6 channel memory support.[53]"

    // "In September 2023, AMD launched their low power and embedded
    // 8004 series of CPUs, codenamed Siena. Siena utilizes a new
    // socket, called SP6, which has a smaller footprint and pin count
    // than the SP5 socket of its contemporary Genoa processors. Siena
    // utilizes the same Zen 4c core architecture as Bergamo cloud
    // native processors, allowing up to 64 cores per processor"

    EPYC9004,

    EPYC9005,
}

pub struct BrandInfo {
    pub first_release_year: u16,
    pub usage: Usage,
    pub codenames: Set<&'static str>,
    pub socket: Set<CPUSocket>,
    /// relative speed of 1 core compared to one of an Epyc 7702 at
    /// the same frequency
    pub epyc_speed: Value<f32>,
    pub microarchitectures: Set<Architecture>,
    
}

impl Brand {
    pub fn info(self) -> BrandInfo {
        use Brand::*;
        use Value::Missing;
        match self {
            Ryzen9 => todo!(),
            RyzenThreadripperPRO5000WX => todo!(),
            RyzenThreadripper => todo!(),

            EPYC7001 => BrandInfo {
                first_release_year: 2017,
                usage: Usage::ServerOrEmbedded,
                codenames: set!["Naples"],
                socket: set![CPUSocket::SP3],
                epyc_speed: Missing,
                microarchitectures: set![Architecture::Zen1 /*?*/], 
            },
            EPYC7002 => BrandInfo {
                first_release_year: 2019,
                usage: Usage::ServerOrEmbedded,
                codenames: set!["Rome"],
                socket: set![CPUSocket::SP3],
                epyc_speed: 1.0.into(),
                microarchitectures: set![Architecture::Zen2],
                
            },
            EPYC7003 => BrandInfo {
                first_release_year: 2021,
                usage: Usage::Server,
                codenames: set!["Milan", "Milan-X"],
                socket: set![CPUSocket::SP3], 
                epyc_speed: 1.2.into(),
                microarchitectures: set![Architecture::Zen3],
            },
            EPYC8004 => BrandInfo {
                first_release_year: 2023,
                usage: Usage::Server,
                codenames: set!["Siena"],
                socket: set![CPUSocket::SP6],
                epyc_speed: Missing,
                microarchitectures: set![Architecture::Zen4c],
            },
            EPYC9004 => BrandInfo {
                first_release_year: 2022,
                usage: Usage::Embedded,
                codenames: set!["Genoa", "Genoa-X", "Bergamo"],
                socket: set![CPUSocket::SP5],
                epyc_speed: Missing,
                microarchitectures: set![Architecture::Zen4], 
            },
            EPYC9005 => BrandInfo {
                first_release_year: 2025,
                usage: Usage::Server,
                codenames: set!["Turin"],
                socket: set![CPUSocket::SP5],
                epyc_speed: Missing,
                microarchitectures: set![Architecture::Zen5],
            },
        }
    }
}


// ("Ryzen", 9) => Brand::Ryzen9
impl TryFrom<(&str, u8)> for Brand {
    type Error = anyhow::Error;

    fn try_from(value: (&str, u8)) -> Result<Self, Self::Error> {
        use Brand::*;
        match value {
            ("Ryzen", 9) => Ok(Ryzen9),
            // ... 
            _ => bail!("unknown Brand string {value:?}")
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketSegment {
    EnthusiastDesktop,
    Server, // well, specified as "Platform": "Server"
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductLine(pub Brand, pub Value<Usage>);
// "AMD Ryzen™ 9 Desktop Processors" == ProductLine(Brand::Ryzen9, Usage::Desktop)

impl TryFrom<&str> for Value<ProductLine> {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vals: Vec<_> = value.split(' ').collect();
        (|| -> Result<Self, Self::Error> {
            match value {
                "AMD Ryzen™ Threadripper™ PRO 5000 WX-Series" =>
                // https://en.wikipedia.org/wiki/Ryzen#Threadripper_series
                // "Threadripper, which is geared for high-end
                // desktops (HEDT) and professional workstations,
                // was not developed as part of a business plan or
                // a specific roadmap. Instead, a small team
                // inside AMD saw an opportunity to develop the
                // benefits of Ryzen and EPYC CPU roadmaps, so as
                // to give AMD the lead in desktop CPU
                // performance. After some progress was made in
                // their spare time, the project was greenlit and
                // put in an official roadmap by 2016.[20] "
                    Ok(Value::Some(
                        ProductLine(Brand::RyzenThreadripperPRO5000WX,
                                    Usage::Desktop.into()))),
                "AMD Ryzen™ 9 Processors" =>
                    Ok(Value::Some(
                        ProductLine(Brand::Ryzen9,
                                    Value::Missing))),

                "AMD EPYC™ 7001 Series" =>
                    Ok(Value::Some(
                        ProductLine(Brand::EPYC7001,
                                    Usage::ServerOrEmbedded.into()))),

                "AMD EPYC™ 7002 Series" =>
                    Ok(Value::Some(
                        ProductLine(Brand::EPYC7002,
                                    Usage::ServerOrEmbedded.into()))),

                "AMD EPYC™ 7003 Series" =>
                    Ok(Value::Some(
                        ProductLine(Brand::EPYC7003,
                                    Usage::Server.into()))),

                "AMD EPYC™ 8004 Series" =>
                    Ok(Value::Some(
                        ProductLine(Brand::EPYC8004,
                                    Usage::ServerOrEmbedded.into()))),

                "AMD EPYC™ 9004 Series" =>
                    Ok(Value::Some(
                        ProductLine(Brand::EPYC9004,
                                    Usage::ServerOrEmbedded.into()))),

                "AMD Ryzen™ Threadripper™ Processors" =>
                    Ok(Value::Some(
                        ProductLine(Brand::RyzenThreadripper,
                                    Value::Missing))),

                    

                _ => {
                    // possibly working more general solution, but I've
                    // given up on parsing, AMD's naming/website is just
                    // too messy.
                    if vals.len() == 5 {
                        if let [amd, brand, generation, usage, processors] = &*vals {
                            if *amd != "AMD" { bail!("expecting string AMD, not {amd:?}") }
                            if *processors != "Processors" {
                                bail!("expecting string Processors, not {processors:?}")
                            }
                            let generation = generation.parse()?;
                            let brand =
                                if let Some((i, c)) = brand.char_indices().last() {
                                    if c == '™' {
                                        &brand[0..i]
                                    } else {
                                        brand
                                    }
                                } else {
                                    brand
                                };
                            Ok(Value::Some(ProductLine(
                                (brand, generation).try_into()?,
                                Value::Some((*usage).try_into()?)
                            )))
                        } else {
                            panic!()
                        }
                    } else {
                        bail!("need exactly 4 ' '")
                    }
                }
            }
        })().with_context(|| anyhow!("invalid product line string {value:?}"))
    }
}
