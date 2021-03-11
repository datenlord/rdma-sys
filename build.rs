// TODO: check libclang-dev version, since Rust bindgen depends on it
// const LIB_CLANG_DEV_VERSION: &str = "3.9";
const LIB_IBVERBS_DEV_VERSION: &str = "1.8.28";
const LIB_RDMACM_DEV_VERSION: &str = "1.2.28";

fn main() {
    // assert!(pkg_config::find_library("libclang-dev").is_ok(), "libclang-dev NOT found");
    // pkg_config::Config::new()
    //     .atleast_version(LIB_CLANG_DEV_VERSION)
    //     //.statik(true)
    //     .probe("libclang")
    //     .expect(&format!("please install libclang-dev {}", LIB_CLANG_DEV_VERSION));

    pkg_config::Config::new()
        .atleast_version(LIB_IBVERBS_DEV_VERSION)
        .statik(true)
        .probe("libibverbs")
        .expect(&format!(
            "please install libibverbs-dev {}",
            LIB_IBVERBS_DEV_VERSION
        ));

    pkg_config::Config::new()
        .atleast_version(LIB_RDMACM_DEV_VERSION)
        .statik(true)
        .probe("librdmacm")
        .expect(&format!(
            "please install librdmacm-dev {}",
            LIB_RDMACM_DEV_VERSION
        ));

    let bindings = bindgen::Builder::default()
        .header("/usr/include/infiniband/verbs.h")
        .header("/usr/include/rdma/rdma_cma.h")
        .whitelist_function("ibv_.*")
        .whitelist_type("ibv_.*")
        .whitelist_function("rdma_.*")
        .whitelist_type("rdma_.*")
        // Following ENUM will used with bitwise-or
        // including flags, mask, caps, bits, fields, size
        .bitfield_enum("ibv_device_cap_flags")
        .bitfield_enum("ibv_odp_transport_cap_bits")
        .bitfield_enum("ibv_odp_general_caps")
        .bitfield_enum("ibv_rx_hash_function_flags")
        .bitfield_enum("ibv_rx_hash_fields")
        .bitfield_enum("ibv_raw_packet_caps")
        .bitfield_enum("ibv_tm_cap_flags")
        .bitfield_enum("ibv_pci_atomic_op_size")
        .bitfield_enum("ibv_port_cap_flags")
        .bitfield_enum("ibv_port_cap_flags2")
        .bitfield_enum("ibv_create_cq_wc_flags")
        .bitfield_enum("ibv_wc_flags")
        .bitfield_enum("ibv_access_flags")
        .bitfield_enum("ibv_xrcd_init_attr_mask")
        .bitfield_enum("ibv_rereg_mr_flags")
        .bitfield_enum("ibv_srq_attr_mask")
        .bitfield_enum("ibv_srq_init_attr_mask")
        .bitfield_enum("ibv_wq_init_attr_mask")
        .bitfield_enum("ibv_wq_flags")
        .bitfield_enum("ibv_wq_attr_mask")
        .bitfield_enum("ibv_ind_table_init_attr_mask")
        .bitfield_enum("ibv_qp_init_attr_mask")
        .bitfield_enum("ibv_qp_create_flags")
        .bitfield_enum("ibv_qp_create_send_ops_flags")
        .bitfield_enum("ibv_qp_open_attr_mask")
        .bitfield_enum("ibv_qp_attr_mask")
        .bitfield_enum("ibv_send_flags")
        .bitfield_enum("ibv_ops_flags")
        .bitfield_enum("ibv_cq_attr_mask")
        .bitfield_enum("ibv_flow_flags")
        .bitfield_enum("ibv_flow_action_esp_mask")
        .bitfield_enum("ibv_cq_init_attr_mask")
        .bitfield_enum("ibv_create_cq_attr_flags")
        .bitfield_enum("ibv_parent_domain_init_attr_mask")
        .bitfield_enum("ibv_read_counters_flags")
        .bitfield_enum("ibv_values_mask")
        // Following ENUM will be const in a sub-mod
        .constified_enum_module("ibv_node_type")
        .constified_enum_module("ibv_transport_type")
        .constified_enum_module("ibv_atomic_cap")
        .constified_enum_module("ibv_mtu")
        .constified_enum_module("ibv_port_state")
        .constified_enum_module("ibv_event_type")
        .constified_enum_module("ibv_wc_status")
        .constified_enum_module("ibv_wc_opcode")
        .constified_enum_module("ibv_mw_type")
        .constified_enum_module("ibv_rate")
        .constified_enum_module("ibv_srq_type")
        .constified_enum_module("ibv_wq_type")
        .constified_enum_module("ibv_wq_state")
        .constified_enum_module("ibv_qp_type")
        .constified_enum_module("ibv_qp_state")
        .constified_enum_module("ibv_mig_state")
        .constified_enum_module("ibv_wr_opcode")
        .constified_enum_module("ibv_ops_wr_opcode")
        .constified_enum_module("ibv_flow_attr_type")
        .constified_enum_module("ibv_flow_spec_type")
        .constified_enum_module("ibv_counter_description")
        .constified_enum_module("ibv_rereg_mr_err_code")
        .derive_default(true)
        .derive_debug(true)
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("./src/rdma.rs")
        .expect("Could not write bindings");
}
