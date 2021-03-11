
const LIB_CLANG_DEV_VERSION: &str = "3.9";
const LIB_IBVERBS_DEV_VERSION: &str = "1.5.22";
const LIB_RDMACM_DEV_VERSION: &str = "1.1.22";

fn main() {
    // assert!(pkg_config::find_library("libclang-dev").is_ok(), "libclang-dev NOT found");
    pkg_config::Config::new()
        .atleast_version(LIB_CLANG_DEV_VERSION)
        .statik(true)
        .probe("libclang")
        .expect(&format!("please install libclang-dev {}", LIB_CLANG_DEV_VERSION));

    pkg_config::Config::new()
        .atleast_version(LIB_IBVERBS_DEV_VERSION)
        .statik(true)
        .probe("libibverbs")
        .expect(&format!("please install libibverbs-dev {}", LIB_IBVERBS_DEV_VERSION));
    
    pkg_config::Config::new()
        .atleast_version(LIB_RDMACM_DEV_VERSION)
        .statik(true)
        .probe("librdmacm")
        .expect(&format!("please install librdmacm-dev {}", LIB_RDMACM_DEV_VERSION));
        

    let bindings = bindgen::Builder::default()
        .header("/usr/include/infiniband/verbs.h")
        //.clang_arg("-Ivendor/rdma-core/build/include/")
        // https://github.com/servo/rust-bindgen/issues/550
        .blacklist_type("max_align_t")
        .whitelist_function("ibv_.*")
        .whitelist_type("ibv_.*")
        .bitfield_enum("ibv_access_flags")
        .bitfield_enum("ibv_qp_attr_mask")
        .bitfield_enum("ibv_wc_flags")
        .bitfield_enum("ibv_send_flags")
        .bitfield_enum("ibv_port_cap_flags")
        .constified_enum_module("ibv_qp_type")
        .constified_enum_module("ibv_qp_state")
        .constified_enum_module("ibv_port_state")
        .constified_enum_module("ibv_wc_opcode")
        .constified_enum_module("ibv_wr_opcode")
        .constified_enum_module("ibv_wc_status")
        //.constified_enum_module("IBV_WC_.*")
        //.constified_enum_module("IBV_WR_.*")
        //.constified_enum_module("IBV_QPS_.*")
        //.constified_enum_module("IBV_PORT_.*")
        .derive_default(true)
        .derive_debug(true)
        .prepend_enum_name(false)
        .blacklist_type("ibv_wc")
        .generate()
        .expect("Unable to generate bindings");

    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        //.write_to_file(out_path.join("bindings.rs"))
        .write_to_file("./src/bindings.rs")
        .expect("Could not write bindings");
}
