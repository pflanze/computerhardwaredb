use anyhow::Result;
use computerhardwaredb::{
    types::{Bytes, CPU, Watt, GHz, CoolerType, SoldAt, CPUSocket, ExtensionBus,
            MemoryType, MTperSec, Shop, GraphicsModel, Price, MemorySubtype, MarketSegment,
            Architecture},
    value::Value,
    index::{pindex_by, mindex_by_foreign},
    collectsorted::{unsafe_cmp, on, CollectSorted}
};


/// A performance estimate score, higher is better, that works for
/// comparison across CPU families and is inversely proportional to
/// time required to compile a single Rust or C++ package on that CPU
/// (using all of its threads as much as compilation allows, assuming
/// use of the mold linker), i.e. simulating rebuilds of a project
/// where a random file is changed (header file in C++ -> rebuild of
/// many of the object files; rebuild of the lib crate and maybe also
/// the app crate in Rust, relying on the parallel stage in LLVM for
/// concurrency, as well as hoping somewhat for future parallelization
/// of the frontend).
fn anticipated_compilation_performance(cpu: &CPU) -> Result<f32> {
    let coresthreads = {
        let cores = *cpu.cores.value()? as f32;
        let threads = *cpu.threads.value()? as f32;
        let additional_threads = threads - cores;
        
        cores + additional_threads * 0.3
    };
    // XX memory channels  compared to threads!

    let base_clock = cpu.base_clock.value()?;
    // let max_boost_clock = cpu.max_boost_clock.value()?;
    // when highly parallel, only base clock usable, right?
    // which is most of my compiler workload.

    let factor = {
        let launch_date_sec = cpu.launch_date.value()?.unixtime();
        // Twice as fast per core every 5 years? In the last 5y anyway? Or
        // should rather look at the architecture, right?
        let launch_date_years: f32 = (launch_date_sec as f32) / (365.*24.*3600.);
        let periods: f32 = launch_date_years / 5.;
        f32::powf(2.0, periods)
    };

    // XX PCIe

    Ok(coresthreads * base_clock.0 * factor)
}


fn main() -> Result<()> {
    let t = true;
    let f = false;
    use Bytes::*;
    use Value::Missing;
    use ExtensionBus::PCIe;

    let cpus: &[CPU] = &[
        CPU {
            name: "AMD Ryzen™ 9 5950X".into(),
            // https://www.amd.com/en/products/cpu/amd-ryzen-9-5950x
            url: "https://www.amd.com/en/product/10456".into(),
            product_line: "AMD Ryzen™ 9 Desktop Processors".try_into()?,
            desc: "".into(),
            cores: 16.into(),
            threads: 32.into(),
            l1cache: Missing,
            l2cache: MB(8).into(),
            l3cache: MB(64).into(),
            tdp: Watt(105).into(),
            base_clock: GHz(3.4).into(),
            max_boost_clock: Missing,
            cooler: CoolerType::LiquidRecommended.into(),
            launch_date: "11/5/2020".try_into()?,
            cpu_socket: CPUSocket::AM4.into(),
            memory_channels: Missing,
            pci_express_version: PCIe(4.0, Missing).into(),
            system_memory_type: MemoryType::DDR4.into(),
            system_memory_specification: MTperSec(3200).into(),
            graphics_model: GraphicsModel::None.into(),
            system_memory_subtype: Missing,
            ecc_support: Missing,
            market_segment: Missing,
            architecture: "AMD \"Zen 3\" Core Architecture".try_into()?,
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },
        CPU {
            name: "AMD Ryzen™ 9 PRO 7945".into(),
            url: "https://www.amd.com/en/product/13496".into(),
            product_line: "AMD Ryzen™ 9 Desktop Processors".try_into()?, // guess
            desc: "".into(),
            cores: 12.into(),
            threads: 24.into(),
            l1cache: KB(768).into(),
            l2cache: MB(12).into(),
            l3cache: MB(64).into(),
            tdp: Watt(65).into(),
            base_clock: GHz(3.7).into(),
            max_boost_clock: GHz(5.4).into(),
            cooler: Missing,
            launch_date: "6/13/2023".try_into()?,
            cpu_socket: CPUSocket::AM5.into(),
            memory_channels: 2.into(),
            pci_express_version: PCIe(
                5.0,
                Value::SomeWithDoubts(24, "Native PCIe® Lanes (Total/Usable)  28 / 24
Additional Usable PCIe Lanes from Motherboard  # huh ?
AMD X670E  12x Gen4
AMD X670   12x Gen4
AMD B650E  8x Gen4
AMD B650   8x Gen4".into())
            ).into(),
            system_memory_type: MemoryType::DDR5.into(),
            system_memory_subtype: MemorySubtype::UDIMM.into(),
            system_memory_specification: Missing,
            ecc_support: t.into(),
            graphics_model: GraphicsModel::Radeon.into(),
            market_segment: Missing,
            architecture: Architecture::Zen4.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },
        CPU {
            name: "AMD Ryzen™ Threadripper™ PRO 5955WX".into(),
            url: "https://www.amd.com/en/products/cpu/amd-ryzen-threadripper-pro-5955wx".into(),
            product_line: "AMD Ryzen™ Threadripper™ PRO 5000 WX-Series".try_into()?,
            desc: "".into(),
            cores: 16.into(),
            threads: 32.into(),
            l1cache: MB(1).into(),
            l2cache: MB(8).into(),
            l3cache: MB(64).into(),
            tdp: Watt(280).into(),
            base_clock: GHz(4.0).into(),
            max_boost_clock: GHz(4.5).into(),
            cooler: Missing,
            launch_date: "3/8/2022".try_into()?,
            cpu_socket: "sWRX8".try_into()?,
            memory_channels: 8.into(),
            pci_express_version: PCIe(4.0, Missing).into(),
            system_memory_type: MemoryType::DDR4.into(),
            system_memory_subtype: Missing,
            system_memory_specification: MTperSec(3200).into(),
            ecc_support: Missing,
            graphics_model: GraphicsModel::None.into(),
            market_segment: Missing,
            architecture: "AMD \"Zen 3\" Core Architecture".try_into()?,
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },
        CPU {
            name: "AMD Ryzen 9 7950X3D".into(),
            url: "https://www.amd.com/en/products/apu/amd-ryzen-9-7950x3d".into(),
            market_segment: MarketSegment::EnthusiastDesktop.into(),
            product_line: "AMD Ryzen™ 9 Processors".try_into()?,
            desc: "".into(),
            cores: 16.into(),
            threads: 32.into(),
            l1cache: MB(1).into(),
            l2cache: MB(16).into(),
            l3cache: MB(128).into(),
            tdp: Watt(120).into(),
            base_clock: GHz(4.2).into(),
            max_boost_clock: GHz(5.7).into(),
            cooler: CoolerType::LiquidRecommended.into(),
            launch_date: "2/28/2023".try_into()?,
            cpu_socket: "AM5".try_into()?,
            memory_channels: 2.into(),
            pci_express_version: PCIe(5.0, Missing).into(),
            system_memory_type: MemoryType::DDR5.into(),
            system_memory_subtype: MemorySubtype::UDIMM.into(),
            system_memory_specification: Missing,
            ecc_support: t.into(),
            graphics_model: GraphicsModel::Radeon.into(),
            architecture: Architecture::Zen4.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },
        CPU {
            // a review (14 November 2019):
            // https://www.tomshardware.com/reviews/amd-ryzen-9-3950x-review
            name: "AMD Ryzen 9 3950X".into(),
            // Can't find this CPU in DDG nor in AMD's main
            // search. Here's another AMD search, finally:
            // https://www.amd.com/en/products/specifications/processors
            url: "https://www.amd.com/en/product/8486".into(),
            market_segment: MarketSegment::EnthusiastDesktop.into(),
            product_line: "AMD Ryzen™ 9 Desktop Processors".try_into()?,
            desc: "".into(),
            cores: 16.into(),
            threads: 32.into(),
            l1cache: MB(1).into(),
            l2cache: MB(8).into(),
            l3cache: MB(64).into(),
            tdp: Watt(105).into(), //  XX AMD Ryzen™ Master Eco-Mode  65W
            base_clock: GHz(3.5).into(),
            max_boost_clock: GHz(4.7).into(),
            cooler: CoolerType::LiquidRecommended .into(),
            launch_date: "7/7/2019".try_into()?,
            cpu_socket: "AM4".try_into()?,
            memory_channels: 2.into(),
            pci_express_version: PCIe(4.0, Missing).into(),
            system_memory_type: MemoryType::DDR4.into(),
            system_memory_subtype: MemorySubtype::UDIMM.into(),
            system_memory_specification: Missing,
            ecc_support: t.into(),
            graphics_model: GraphicsModel::None.into(),
            pcie_dma_security: t.into(),
            usb_dma_security: t.into(),
            amd_enhanced_virus_protection_nx_bit: t.into(),
            architecture: "Zen 2".try_into()?,
        },

        CPU {
            name: "AMD EPYC 7502P".into(),
            url: "https://www.amd.com/en/products/cpu/amd-epyc-7502p".into(),
            market_segment: Missing,
            product_line: "AMD EPYC™ 7002 Series".try_into()?,
            architecture: "AMD Infinity Architecture".try_into()?,
            desc: "".into(),
            cores: 32.into(),
            threads: 64.into(),
            l1cache: Missing,
            l2cache: Missing,
            l3cache: MB(128).into(),
            tdp: Watt(180).into(),
            base_clock: GHz(2.5).into(),
            max_boost_clock: GHz(3.35).into(),
            cooler: Missing,
            launch_date: Value::SomeWithDoubts(
                "August 7, 2019".try_into()?,
                "from https://en.wikipedia.org/wiki/Zen_2".into()),
            cpu_socket: "SP3".try_into()?,
            memory_channels: 8.into(),
            pci_express_version: PCIe(4.0, 128.into()).into(),
            system_memory_type: MemoryType::DDR4.into(),
            system_memory_subtype: Missing,
            system_memory_specification: MTperSec(3200).into(), //  Per Socket Mem BW  204.8 GB/s
            ecc_support: Missing,
            graphics_model: GraphicsModel::None.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },

        CPU {
            name: "AMD EPYC™ 8224P".into(),
            url: "https://www.amd.com/en/products/cpu/amd-epyc-8224p".into(),
            market_segment: MarketSegment::Server.into(), //  Platform  Server
            product_line: "AMD EPYC™ 8004 Series".try_into()?,
            architecture: "AMD Infinity Architecture".try_into()?,
            desc: "".into(),
            cores: 24.into(),
            threads: 48.into(),
            l1cache: Missing,
            l2cache: Missing,
            l3cache: MB(64).into(),
            tdp: Watt(160).into(),
            base_clock: GHz(2.55).into(),
            max_boost_clock: GHz(3.0).into(),
            cooler: Missing,
            launch_date: "9/18/2023".try_into()?,
            cpu_socket: "AM4".try_into()?,
            memory_channels: 6.into(),
            pci_express_version: PCIe(5.0, 96.into()).into(),
            system_memory_type: MemoryType::DDR5.into(),
            system_memory_subtype: Missing,
            system_memory_specification: MTperSec(4800).into(),
            // ^ Per Socket Mem BW  230.4 GB/s (4.800*6 * 8)
            ecc_support: Missing,
            graphics_model: GraphicsModel::None.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },
        
        CPU {
            name: "AMD EPYC 7443".into(),
            url: "https://www.amd.com/en/products/cpu/amd-epyc-7443".into(),
            market_segment: MarketSegment::Server.into(),
            product_line: "AMD EPYC™ 7003 Series".try_into()?,
            architecture: "AMD Infinity Architecture".try_into()?,
            desc: "".into(),
            cores: 24.into(),
            threads: 48.into(),
            l1cache: Missing,
            l2cache: Missing,
            l3cache: MB(128).into(),
            tdp: Watt(200).into(), // AMD Configurable TDP (cTDP)  165-200W
            base_clock: GHz(2.85).into(),
            max_boost_clock: GHz(4.0).into(),
            cooler: Missing,
            launch_date: "3/15/2021".try_into()?,
            cpu_socket: "SP3".try_into()?,
            memory_channels: 8.into(),
            pci_express_version: PCIe(4.0, 128.into()).into(),
            system_memory_type: MemoryType::DDR4.into(),
            system_memory_subtype: Missing,
            system_memory_specification: MTperSec(3200).into(),
            ecc_support: Missing,
            graphics_model: GraphicsModel::None.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },
        
        CPU {
            name: "AMD EPYC 73711P".into(),
            url: "https://www.amd.com/en/products/cpu/amd-epyc-7313p".into(),
            market_segment: MarketSegment::Server.into(),
            product_line: "AMD EPYC™ 7003 Series".try_into()?,
            architecture: "AMD Infinity Architecture".try_into()?,
            desc: "".into(),
            cores: 16.into(),
            threads: 32.into(),
            l1cache: Missing,
            l2cache: Missing,
            l3cache: MB(128).into(),
            tdp: Watt(155).into(), // AMD Configurable TDP (cTDP)  155-180W
            base_clock: GHz(3.0).into(),
            max_boost_clock: GHz(3.7).into(),
            cooler: Missing,
            launch_date: "3/15/2021".try_into()?,
            cpu_socket: "SP3".try_into()?,
            memory_channels: 8.into(),
            pci_express_version: PCIe(4.0, 128.into()).into(),
            system_memory_type: MemoryType::DDR4.into(),
            system_memory_subtype: Missing,
            system_memory_specification: MTperSec(3200).into(),
            ecc_support: Missing,
            graphics_model: GraphicsModel::None.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },

        CPU {
            name: "AMD EPYC™ 7352".into(),
            url: "https://www.amd.com/en/products/cpu/amd-epyc-7352".into(),
            market_segment: MarketSegment::Server.into(),
            product_line: "AMD EPYC™ 7002 Series".try_into()?,
            architecture: "AMD Infinity Architecture".try_into()?,
            desc: "".into(),
            cores: 24.into(),
            threads: 48.into(),
            l1cache: Missing,
            l2cache: Missing,
            l3cache: MB(128).into(),
            tdp: Watt(155).into(),
            base_clock: GHz(2.3).into(),
            max_boost_clock: GHz(3.2).into(),
            cooler: Missing,
            launch_date: Value::SomeWithDoubts(
                "August 7, 2019".try_into()?,
                "from https://en.wikipedia.org/wiki/Zen_2".into()
            ),
            cpu_socket: "SP3".try_into()?,
            memory_channels: 8.into(),
            pci_express_version: PCIe(4.0, 128.into()).into(),
            system_memory_type: MemoryType::DDR4.into(),
            system_memory_subtype: Missing,
            system_memory_specification: MTperSec(3200).into(),
            ecc_support: Missing,
            graphics_model: GraphicsModel::None.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },
        
        CPU {
            name: "AMD EPYC 9224".into(),
            url: "https://www.amd.com/en/products/cpu/amd-epyc-9224".into(),
            market_segment: MarketSegment::Server.into(),
            product_line: "AMD EPYC™ 9004 Series".try_into()?,
            architecture: "AMD Infinity Architecture".try_into()?,
            desc: "".into(),
            cores: 24.into(),
            threads: 48.into(),
            l1cache: Missing,
            l2cache: Missing,
            l3cache: MB(64).into(),
            tdp: Watt(200).into(),
            base_clock: GHz(2.5).into(),
            max_boost_clock: GHz(3.65).into(),
            cooler: Missing,
            launch_date: "11/10/2022".try_into()?,
            cpu_socket: "SP5".try_into()?,
            memory_channels: 12.into(),
            pci_express_version: PCIe(5.0, 128.into()).into(),
            system_memory_type: MemoryType::DDR5.into(),
            system_memory_subtype: Missing,
            system_memory_specification: MTperSec(4800).into(),
            ecc_support: Missing,
            graphics_model: GraphicsModel::None.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },

        CPU {
            name: "AMD EPYC™ 7443P".into(),
            url: "https://www.amd.com/en/products/cpu/amd-epyc-7443P".into(),
            market_segment: MarketSegment::Server.into(),
            product_line: "AMD EPYC™ 7003 Series".try_into()?,
            architecture: "AMD Infinity Architecture".try_into()?,
            desc: "".into(),
            cores: 24.into(),
            threads: 48.into(),
            l1cache: Missing,
            l2cache: Missing,
            l3cache: MB(128).into(),
            tdp: Watt(200).into(), // AMD Configurable TDP (cTDP) 165-200W
            base_clock: GHz(2.85).into(),
            max_boost_clock: GHz(4.0).into(),
            cooler: Missing,
            launch_date: "3/15/2021".try_into()?,
            cpu_socket: "SP3".try_into()?,
            memory_channels: 8.into(),
            pci_express_version: PCIe(4.0, 128.into()).into(),
            system_memory_type: MemoryType::DDR4.into(),
            system_memory_subtype: Missing,
            system_memory_specification: MTperSec(3200).into(),
            ecc_support: Missing,
            graphics_model: GraphicsModel::None.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },

        CPU {
            name: "AMD EPYC™ 7513".into(),
            url: "https://www.amd.com/en/products/cpu/amd-epyc-7513".into(),
            market_segment: MarketSegment::Server.into(),
            product_line: "AMD EPYC™ 7003 Series".try_into()?,
            architecture: "AMD Infinity Architecture".try_into()?,
            desc: "".into(),
            cores: 32.into(),
            threads: 64.into(),
            l1cache: Missing,
            l2cache: Missing,
            l3cache: MB(128).into(),
            tdp: Watt(200).into(),
            base_clock: GHz(2.6).into(),
            max_boost_clock: GHz(3.65).into(),
            cooler: Missing,
            launch_date: "3/15/2021".try_into()?,
            cpu_socket: "SP3".try_into()?,
            memory_channels: 8.into(),
            pci_express_version: PCIe(4.0, 128.into()).into(),
            system_memory_type: MemoryType::DDR4.into(),
            system_memory_subtype: Missing,
            system_memory_specification: MTperSec(3200).into(),
            ecc_support: Missing,
            graphics_model: GraphicsModel::None.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },

        CPU {
            name: "AMD Ryzen™ 9 7950X".into(),
            url: "https://www.amd.com/en/products/cpu/amd-ryzen-9-7950x".into(),
            market_segment: MarketSegment::EnthusiastDesktop.into(),
            product_line: "AMD Ryzen™ 9 Processors".try_into()?,
            architecture: "Zen 4".try_into()?,
            desc: "".into(),
            cores: 16.into(),
            threads: 32.into(),
            l1cache: MB(1).into(),
            l2cache: MB(16).into(),
            l3cache: MB(64).into(),
            tdp: Watt(170).into(),
            base_clock: GHz(4.5).into(),
            max_boost_clock: GHz(5.7).into(),
            cooler: CoolerType::LiquidRecommended.into(),
            launch_date: "9/27/2022".try_into()?,
            cpu_socket: "AM5".try_into()?,
            memory_channels: 2.into(),
            pci_express_version: PCIe(5.0, Missing).into(),
            system_memory_type: MemoryType::DDR5.into(),
            system_memory_subtype: MemorySubtype::UDIMM.into(),
            system_memory_specification: Missing,
            ecc_support: t.into(),
            graphics_model: GraphicsModel::Radeon.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },

        CPU {
            // AMD Ryzen TR 7960X
            name: "AMD Ryzen™ Threadripper™ 7960X".into(),
            url: "https://www.amd.com/en/products/cpu/amd-ryzen-threadripper-7960x".into(),
            market_segment: MarketSegment::EnthusiastDesktop.into(),
            product_line: "AMD Ryzen™ Threadripper™ Processors".try_into()?,
            architecture: "Zen 4".try_into()?,
            desc: "".into(),
            cores: 24.into(),
            threads: 48.into(),
            l1cache: KB(1536).into(),
            l2cache: MB(24).into(),
            l3cache: MB(128).into(),
            tdp: Watt(350).into(), // !
            base_clock: GHz(4.2).into(),
            max_boost_clock: GHz(5.3).into(),
            cooler: Missing,
            launch_date: "10/19/2023".try_into()?,
            cpu_socket: "sTR5".try_into()?,
            memory_channels: 4.into(),
            pci_express_version: PCIe(5.0, Missing).into(),
            system_memory_type: MemoryType::DDR5.into(),
            system_memory_subtype: MemorySubtype::RDIMM.into(),
            system_memory_specification: MTperSec(5200).into(),
            ecc_support: t.into(), // "Default Enabled"
            graphics_model: GraphicsModel::None.into(),
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: t.into(),
        },

        CPU {
            // "Intel® Xeon® Gold 6248R Processor", Processor Number: 6248R
            // so, invent our own naming, sigh
            name: "Intel 6248R".into(),
            url: "https://ark.intel.com/content/www/us/en/ark/products/199351/intel-xeon-gold-6248r-processor-35-75m-cache-3-00-ghz.html".into(),
            market_segment: MarketSegment::Server.into(), //  "Vertical Segment"
            product_line: Missing, // What would that be for Intel?
            architecture: Missing, // What would that be for Intel?
            desc: "".into(),
            cores: 24.into(), // XX how many performance vs efficiency scores ?
            threads: 48.into(),
            l1cache: Missing,
            l2cache: Missing,
            l3cache: KB(36608).into(), // "Cache" "35.75 MB", but is probably *total*
            tdp: Watt(205).into(),
            base_clock: GHz(3.0).into(),
            max_boost_clock: GHz(4.0).into(),
            cooler: Missing,
            launch_date: "Q1'20".try_into()?,
            cpu_socket: "FCLGA3647".try_into()?,
            memory_channels: 6.into(), // "Max # of Memory Channels", max? ah if all slots used?
            pci_express_version: PCIe(3.5, 48.into()).into(),
            system_memory_type: MemoryType::DDR4.into(), // DDR4-2933
            system_memory_subtype: Missing,
            //  Max Memory Size (dependent on memory type) 1 TB
            system_memory_specification: Missing,
            ecc_support: t.into(),
            graphics_model: Missing,
            pcie_dma_security: Missing,
            usb_dma_security: Missing,
            amd_enhanced_virus_protection_nx_bit: Missing,
        },

        // CPU {
        //     name: "".into(),
        //     url: "".into(),
        //     market_segment: MarketSegment::.into(),
        //     product_line: "".try_into()?,
        //     architecture: "".try_into()?,
        //     desc: "".into(),
        //     cores: .into(),
        //     threads: .into(),
        //     l1cache: .into(),
        //     l2cache: .into(),
        //     l3cache: .into(),
        //     tdp: Watt().into(),
        //     base_clock: GHz().into(),
        //     max_boost_clock: GHz().into(),
        //     cooler: .into(),
        //     launch_date: "".try_into()?,
        //     cpu_socket: "".try_into()?,
        //     memory_channels: .into(),
        //     pci_express_version: PCIe().into(),
        //     system_memory_type: MemoryType::.into(),
        //     system_memory_subtype: MemorySubtype::.into(),
        //     system_memory_specification: MTperSec().into(),
        //     ecc_support: .into(),
        //     graphics_model: GraphicsModel::.into(),
        //     pcie_dma_security: .into(),
        //     usb_dma_security: .into(),
        //     amd_enhanced_virus_protection_nx_bit: .into(),
        // },
        
    ];

    use Price::*;
    let sold_at: &[SoldAt] = &[
        SoldAt {
            article_name: "AMD Ryzen™ 9 5950X".into(),
            shop: Shop::Digitec,
            desc: "AMD Ryzen 9 5950X
AM4, 3.40 GHz, 16 -Core".into(),
            url: "https://www.digitec.ch/en/s1/product/amd-ryzen-9-5950x-am4-340-ghz-16-core-processors-13987916".into(),
            price: CHF(366).into(),
            is_tray_version: f.into(),
            is_used: f.into(),
            delivered: "Delivered the day after tomorrow
10 items in stock".into(),
        },

        SoldAt {
            article_name: "AMD Ryzen™ 9 5950X".into(),
            shop: Shop::Digitec,
            desc: "AMD Ryzen 9 5950X
AM4, 3.40 GHz, 16 -Core".into(),
            url: "https://www.digitec.ch/en/s1/product/amd-ryzen-9-5950x-am4-340-ghz-16-core-processors-13987916?shid=1399419".into(),
            price: CHF(329).into(),
            is_tray_version: f.into(),
            is_used: t.into(),
            delivered: "Delivered the day after tomorrow
Only 1 item in stock".into(),
        },

        SoldAt {
            article_name: "AMD Ryzen™ 9 5950X".into(),
            shop: Shop::Digitec,
            desc: "AMD Ryzen 9 5950X
AM4, 3.40 GHz, 16 -Core".into(),
            url: "https://www.digitec.ch/en/s1/product/amd-ryzen-9-5950x-34-ghz-am4-340-ghz-16-core-processors-31176126".into(),
            price: CHF(614).into(),
            is_tray_version: f.into(),
            is_used: f.into(),
            delivered: "Delivered between Fri 5.4. and Thu 11.4.
7 items in stock at third-party supplier
Supplied byJACOB DE".into(), // so bad, be careful about 3rd party, so bad?
        },

        SoldAt {
            article_name: "AMD Ryzen™ 9 5950X".into(),
            shop: Shop::Digitec,
            desc: "AMD CPU Ryzen 9 5950X 3.4GHz 16-core AM4
AM4, 3.40 GHz, 16 -Core".into(),
            url: "https://www.digitec.ch/en/s1/product/amd-cpu-ryzen-9-5950x-34ghz-16-core-am4-am4-340-ghz-16-core-processors-36137541".into(),
            price: CHF(860).into(),
            is_tray_version: f.into(),
            is_used: f.into(),
            delivered: "Delivered between Thu 18.4. and Wed 1.5.
More than 10 items in stock at supplier".into(), // even worse.  be careful ??
        },

        SoldAt {
            article_name: "AMD Ryzen 9 PRO 7945".into(),
            desc: "AMD Ryzen 9 Pro 7945 Tray Version AM5, 3.70 GHz, 12 -Core".into(),
            shop: Shop::Digitec,
            url: "https://www.digitec.ch/en/s1/product/amd-ryzen-9-pro-7945-tray-version-am5-370-ghz-12-core-processors-37097588".into(),
            price: CHF(481),
            is_tray_version: true,
            is_used: false,
            delivered: "Delivered Wed 3.4. Only 1 item in stock".into(),
        },
        SoldAt {
            article_name: "AMD Ryzen Threadripper PRO 5955WX".into(),
            desc: "AMD Ryzen ThreadRipper PRO 5955WX - 4 GHz - 16 cores - 32 threads - 64 MB cache memory - Socket sWRX8 - OEM.".into(),
            shop: Shop::Digitec,
            url: "https://www.digitec.ch/en/s1/product/amd-ryzen-threadripper-pro-5955wx-4-gh-swrx8-4-ghz-16-core-processors-23263816".into(),
            price: CHF(966),
            is_tray_version: false,
            is_used: false,
            delivered: "Delivered between Wed 10.4. and Sat 13.4.
Only 1 item in stock at supplier".into(),
        },
        SoldAt {
            article_name: "AMD Ryzen 9 7950X3D".into(),
            desc: "AMD AM5 Ryzen 9 7950X3D
Tray 5.7GHz 16xCore 144MB 120W
AM5, 4.20 GHz, 16 -Core".into(),
            shop: Shop::Digitec,
            url: "https://www.digitec.ch/en/s1/product/amd-am5-ryzen-9-7950x3d-tray-57ghz-16xcore-144mb-120w-am5-420-ghz-16-core-processors-36941584".into(),
            price: CHF(791),
            is_tray_version: true,
            is_used: false,
            delivered: "Delivered between Thu 4.4. and Thu 11.4.
More than 10 items in stock at third-party supplier
Supplied byorderflow.ch CH".into(),
        },
        SoldAt {
            article_name: "AMD Ryzen 9 3950X".into(),
            desc: "last new 1269.–

AM4, 3.50 GHz, 16 -Core".into(),
            shop: Shop::Digitec,
            url: "https://www.digitec.ch/en/s1/product/amd-ryzen-9-3950x-am4-350-ghz-16-core-processors-11239808?shid=1383800".into(),
            price: CHF(1070),
            is_tray_version: false,
            is_used: true,
            delivered: "Delivered between Tue 2.4. and Mon 8.4.
loicbujard9
Buy used from
loicbujard9 · Member since 2014".into(), // Oh careful
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-ryzen-threadripper-pro-5955wx-swrx8-4-ghz-16-core-processors-22516524".into(),
            article_name: "AMD Ryzen Threadripper PRO 5955WX".into(),
            desc: "sWRX8, 4 GHz, 16 -Core".into(),
            shop: Shop::Digitec,
            price: CHF(997),
            is_tray_version: f,
            is_used: f,
            delivered: "Delivered between Fri 12.4. and Wed 24.4.
5 items in stock at supplier".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-epyc-7502p-sp3-250-ghz-32-core-processors-12279505".into(),
            article_name: "AMD EPYC 7502P".into(),
            desc: "SP3, 2.50 GHz, 32 -Core".into(),
            shop: Shop::Digitec,
            price: CHF(1121),
            is_tray_version: f,
            is_used: f,
            delivered: "Delivered between Thu 4.4. and Thu 11.4.
5 items in stock at third-party supplier".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-epyc-7502p-25ghz-tray-sp3-250-ghz-32-core-processors-20922660".into(),
            article_name: "AMD EPYC 7502P".into(),
            desc: " - 2.5GHz (Tray)
SP3, 2.50 GHz, 32 -Core
Socket SP3 / 32 core / 64 threads / 128MB cache / 180W TDP.".into(),
            shop: Shop::Digitec,
            price: CHF(1045),
            is_tray_version: t,
            is_used: f,
            delivered: "Delivered between Tue 2.4. and Thu 4.4.
5 items in stock at supplier".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-epyc-8224p-255-ghz-24-cores-4-sp6-255-ghz-24-core-processors-40944724".into(),
            desc: " - 2.55 GHz - 24 cores - 4
SP6, 2.55 GHz, 24 -Core".into(),
            article_name: "AMD EPYC 8224P".into(),
            shop: Shop::Digitec,
            price: CHF(1023),
            is_tray_version: f,
            is_used: f,
            delivered: "Delivered between Fri 5.4. and Fri 12.4.
6 items in stock at third-party supplier
Supplied by
JACOB DE".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-ryzen-9-7950x3d-42-ghz-16-cores-am5-420-ghz-16-core-processors-32888396".into(),
            desc: "
 - 4.2 GHz - 16 cores
AM5, 4.20 GHz, 16 -Core
".into(),
            article_name: "AMD Ryzen 9 7950X3D".into(),
            shop: Shop::Digitec,
            price: CHF(815),
            is_tray_version: f,
            is_used: f,
            delivered: "Delivered between Fri 5.4. and Fri 12.4.
More than 10 items in stock at third-party supplier
Supplied by
JACOB DE".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-epyc-7443-tray-4-units-only-sp3-285-ghz-24-core-processors-15655850".into(),
            desc: "AMD Epyc 7443 Tray 4 units only  <-- XX
SP3, 2.85 GHz, 24 -Core".into(),
            article_name: "AMD EPYC 7443".into(),
            shop: Shop::Digitec,
            price: CHF(1224),
            is_tray_version: t,
            is_used: f,
            delivered: "Delivered between Tue 2.4. and Thu 4.4.
Only 1 item in stock at supplie".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-epyc-16-core-73711p-36ghz-sp3-240-ghz-16-core-processors-10892979".into(),
            desc: "AMD EPYC 16-CORE 73711P
3.6GHZ
SP3, 2.40 GHz, 16 -Core
EPYC 7351, 16C/32T, 2.4GHz (2.9GHz Max), 64MB L3 Cache, 170W.".into(),
            article_name: "AMD EPYC 73711P".into(),
            shop: Shop::Digitec,
            price: CHF(509),
            is_tray_version: f,
            is_used: f,
            delivered: "
Delivered between Wed 10.4. and Tue 16.4.
6 items in stock at supplier".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-epyc-7352-sp3-230-ghz-24-core-processors-12279514".into(),
            desc: "AMD Epyc 7352
SP3, 2.30 GHz, 24 -Core".into(),
            article_name: "AMD EPYC 7352".into(),
            shop: Shop::Digitec,
            price: CHF(753),
            is_tray_version: f,
            is_used: f,
            delivered: "Delivered between Wed 10.4. and Tue 16.4.
Only 4 items in stock at supplier".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-epyc-9224-25-ghz-24-cores-48-sp5-250-ghz-48-core-processors-32425504".into(),
            desc: " - 2.5 GHz - 24 cores - 48
SP5, 2.50 GHz, 48 -Core
AMD EPYC 9224 - 2.5 GHz - 24 cores - 48 threads - 64 MB cache memory - Socket SP5 - OEM.".into(),
            article_name: "AMD EPYC 9224".into(),
            shop: Shop::Digitec,
            price: CHF(1755),
            is_tray_version: f,
            is_used: f,
            delivered: "Delivered between Sat 13.4. and Wed 1.5.
Only 4 items in stock at supplier".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-epyc-24core-model-7443p-sp3-box-sp3-285-ghz-24-core-processors-37552911".into(),
            desc: "AMD EPYC 24Core Model 7443P
SP3 BOX
SP3, 2.85 GHz, 24 -Core".into(),
            article_name: "AMD EPYC 7443P".into(),
            shop: Shop::Digitec,
            price: CHF(1336),
            is_tray_version: f,
            is_used: f,
            delivered: "Delivered between Wed 22.5. and Thu 4.7. XX".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-epyc-7513-26-ghz-sp3-260-ghz-32-core-processors-21973612".into(),
            desc: " 2.6 GHz
SP3, 2.60 GHz, 32 -Core
".into(),
            article_name: "AMD EPYC 7513".into(),
            shop: Shop::Digitec,
            price: CHF(839),
            is_tray_version: f,
            is_used: f,
            delivered: "Delivered between Tue 14.5. and Wed 29.5.".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-ryzen-9-7950x3d-am5-420-ghz-16-core-processors-24107476".into(),
            desc: "AM5, 4.20 GHz, 16 -Core".into(),
            article_name: "AMD Ryzen 9 7950X3D".into(),
            shop: Shop::Digitec,
            price: CHF(570),
            is_tray_version: f,
            is_used: f,
            delivered: "Delivered Wed 3.4.
More than 10 items in stock".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-ryzen-9-7950x-am5-450-ghz-16-core-processors-21918730".into(),
            desc: "AM5, 4.50 GHz, 16 -Core
16-Core / 32-Threads / 4.5Hz / Socket AM5.
".into(),
            article_name: "AMD Ryzen 9 7950X".into(),
            shop: Shop::Digitec,
            price: CHF(511),
            is_tray_version: f,
            is_used: f,
            delivered: "Delivered Wed 3.4.
More than 10 items in stock".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-ryzen-tr-7960x-tray-8-units-str5-420-ghz-24-core-processors-40183078".into(),
            desc: " Tray 8 units
sTR5, 4.20 GHz, 24 -Core
AMD Ryzen TR 7960X Tray 8 units.".into(),
            article_name: "AMD Ryzen Threadripper 7960X".into(),
            shop: Shop::Digitec,
            price: CHF(1520),
            is_tray_version: t,
            is_used: f,
            delivered: "Delivered between Wed 10.4. and Tue 16.4.
5 items in stock at supplier".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/amd-threadripper-7960x-str5-str5-420-ghz-24-core-processors-39441097".into(),
            desc: "AMD THREADRIPPER 7960X STR5
sTR5, 4.20 GHz, 24 -Core
24C 5.3GHZ 152MB 350W WOF.".into(),
            article_name: "AMD Ryzen Threadripper 7960X".into(),
            shop: Shop::Digitec,
            price: CHF(1400),
            is_tray_version: f,
            is_used: f,
            delivered: "Delivered Wed 3.4.
Only 2 items in stock".into(),
        },
        SoldAt {
            url: "https://www.digitec.ch/en/s1/product/intel-intel-xeon-6248r-lga-3647-3-ghz-24-core-processors-14053584".into(),
            desc: "–18%
only 1 item on saleonly 1 piece on sale
.–
was 1415.05
Intel Intel Xeon 6248R
LGA 3647, 3 GHz, 24 -Core
INTEL Xeon Gold 6248R 3.0GHz FC-LGA3647 35.75M Cache Tray CPU.".into(),
            article_name: "Intel 6248R".into(),
            shop: Shop::Digitec,
            price: CHF(1159),
            is_tray_version: f, // XX well, t but OK
            is_used: f,
            delivered: "Delivered Wed 3.4.
Only 1 item in stock".into(),
        },

    ];
    let cpus_by_name = pindex_by(cpus, |s| &s.name)?;
    let sold_at_by_article_name = mindex_by_foreign(
        sold_at, |s| &s.article_name, &cpus_by_name, "SoldAt.article_name -> Article.name")?;
    for cpu in cpus {
        if ! sold_at_by_article_name.contains_key(&cpu.name) {
            println!("WARNING: cpu {:?} is not being sold", &cpu.name);
        }
    }

    let offers = sold_at.iter().map(|offer| -> Result<_> {
        let cpu = cpus_by_name.get(&offer.article_name).expect("checked already");
        let perf = anticipated_compilation_performance(cpu)?;
        let value = perf / (offer.price.in_chf() as f32);
        Ok((offer, perf, value))
    });
    let offers = Box::new(offers).try_collect_sorted_by(on(|(_, _perf, _value)| _value,
                                                           unsafe_cmp))?;

    println!("{offers:#?}");
    Ok(())
}
