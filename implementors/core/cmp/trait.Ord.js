(function() {var implementors = {};
implementors["raw_cpuid"] = [];implementors["libc"] = [];implementors["x86"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='x86/irq/struct.PageFaultError.html' title='x86::irq::PageFaultError'>PageFaultError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='x86/rflags/struct.RFlags.html' title='x86::rflags::RFlags'>RFlags</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='x86/paging/struct.PAddr.html' title='x86::paging::PAddr'>PAddr</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='x86/paging/struct.VAddr.html' title='x86::paging::VAddr'>VAddr</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='x86/paging/struct.PML4Entry.html' title='x86::paging::PML4Entry'>PML4Entry</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='x86/paging/struct.PDPTEntry.html' title='x86::paging::PDPTEntry'>PDPTEntry</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='x86/paging/struct.PDEntry.html' title='x86::paging::PDEntry'>PDEntry</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='x86/paging/struct.PTEntry.html' title='x86::paging::PTEntry'>PTEntry</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='x86/segmentation/struct.SegmentSelector.html' title='x86::segmentation::SegmentSelector'>SegmentSelector</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='x86/segmentation/struct.SegmentDescriptor.html' title='x86::segmentation::SegmentDescriptor'>SegmentDescriptor</a>",];implementors["perfcnt"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.ReadFormatFlags.html' title='perfcnt::linux::perf_format::ReadFormatFlags'>ReadFormatFlags</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.SampleFormatFlags.html' title='perfcnt::linux::perf_format::SampleFormatFlags'>SampleFormatFlags</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.EventAttrFlags.html' title='perfcnt::linux::perf_format::EventAttrFlags'>EventAttrFlags</a>",];implementors["perfcnt"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.ReadFormatFlags.html' title='perfcnt::linux::perf_format::ReadFormatFlags'>ReadFormatFlags</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.SampleFormatFlags.html' title='perfcnt::linux::perf_format::SampleFormatFlags'>SampleFormatFlags</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.EventAttrFlags.html' title='perfcnt::linux::perf_format::EventAttrFlags'>EventAttrFlags</a>",];implementors["perfcnt"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.ReadFormatFlags.html' title='perfcnt::linux::perf_format::ReadFormatFlags'>ReadFormatFlags</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.SampleFormatFlags.html' title='perfcnt::linux::perf_format::SampleFormatFlags'>SampleFormatFlags</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.EventAttrFlags.html' title='perfcnt::linux::perf_format::EventAttrFlags'>EventAttrFlags</a>",];implementors["perfcnt"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.ReadFormatFlags.html' title='perfcnt::linux::perf_format::ReadFormatFlags'>ReadFormatFlags</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.SampleFormatFlags.html' title='perfcnt::linux::perf_format::SampleFormatFlags'>SampleFormatFlags</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html' title='core::cmp::Ord'>Ord</a> for <a class='struct' href='perfcnt/linux/perf_format/struct.EventAttrFlags.html' title='perfcnt::linux::perf_format::EventAttrFlags'>EventAttrFlags</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
