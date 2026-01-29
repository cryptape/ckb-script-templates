ckb_std::entry_simulator!({{project-name | append: "@@SIMULATOR_PLACEHOLDER@@" | remove: "-sim@@SIMULATOR_PLACEHOLDER@@" | replace: "-", "_"}}::program_entry);

// Flush LLVM coverage data when the shared library is unloaded.
// This is necessary because coverage data from dynamically loaded libraries
// is not automatically written when using cargo-llvm-cov.
#[cfg(coverage)]
mod coverage_flush {
    unsafe extern "C" {
        fn __llvm_profile_write_file() -> i32;
    }

    #[ctor::dtor]
    fn flush_coverage() {
        unsafe {
            __llvm_profile_write_file();
        }
    }
}
