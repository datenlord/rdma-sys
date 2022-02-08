use bindgen::callbacks::ParseCallbacks;
use std::env;
use std::path::Path;
// TODO: check libclang-dev version, since Rust bindgen depends on it
// const LIB_CLANG_DEV_VERSION: &str = "3.9";
const LIB_IBVERBS_DEV_VERSION: &str = "1.8.28";
const LIB_RDMACM_DEV_VERSION: &str = "1.2.28";

#[derive(Debug)]
struct BindGenCallback;

impl ParseCallbacks for BindGenCallback {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        if original_item_name == "in6_addr" {
            Some(
                original_item_name
                    .replace("in6_addr", "libc::in6_addr")
                    .to_string(),
            )
        } else if original_item_name.starts_with("pthread_") {
            Some(
                original_item_name
                    .replace("pthread_", "libc::pthread_")
                    .to_string(),
            )
        } else if original_item_name.starts_with("sockaddr") {
            Some(
                original_item_name
                    .replace("sockaddr", "libc::sockaddr")
                    .to_string(),
            )
        } else if original_item_name == "timespec" {
            Some(
                original_item_name
                    .replace("timespec", "libc::timespec")
                    .to_string(),
            )
        } else {
            None
        }
    }
}

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
            LIB_RDMACM_DEV_VERSION,
        ));

    let bindings = bindgen::Builder::default()
        .header("/usr/include/infiniband/verbs.h")
        .header("/usr/include/rdma/rdma_cma.h")
        .header("/usr/include/rdma/rdma_verbs.h")
        .allowlist_function("ibv_.*")
        .allowlist_type("ibv_.*")
        .allowlist_function("rdma_.*")
        .allowlist_type("rdma_.*")
        .allowlist_type("verbs_.*")
        .allowlist_type("ib_uverbs_access_flags")
        //.allowlist_type("verbs_devices_ops")
        //.allowlist_var("verbs_provider_.*")
        .blocklist_type("in6_addr")
        .blocklist_type("pthread_.*")
        .blocklist_type("sockaddr.*")
        .blocklist_type("timespec")
        .blocklist_type("ibv_ah_attr")
        .blocklist_type("ibv_async_event")
        .blocklist_type("ibv_flow_spec")
        .blocklist_type("ibv_gid")
        .blocklist_type("ibv_global_route")
        .blocklist_type("ibv_mw_bind_info")
        .blocklist_type("ibv_ops_wr")
        .blocklist_type("ibv_send_wr")
        .blocklist_type("ibv_wc")
        .blocklist_type("rdma_addr")
        .blocklist_type("rdma_cm_event")
        .blocklist_type("rdma_ib_addr")
        .blocklist_type("rdma_ud_param")
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
        .bitfield_enum("ibv_srq_init_attr_mask") // TODO: need to be bitfield?
        .bitfield_enum("ibv_wq_init_attr_mask")
        .bitfield_enum("ibv_wq_flags")
        .bitfield_enum("ibv_wq_attr_mask")
        .bitfield_enum("ibv_ind_table_init_attr_mask")
        .bitfield_enum("ibv_qp_init_attr_mask") // TODO: need to be bitfield?
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
        .bitfield_enum("ib_uverbs_access_flags")
        .bitfield_enum("rdma_cm_join_mc_attr_mask")
        .bitfield_enum("rdma_cm_mc_join_flags")
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
        .constified_enum_module("ib_uverbs_advise_mr_advice")
        .constified_enum_module("rdma_cm_event_type")
        .constified_enum_module("rdma_driver_id")
        .constified_enum_module("rdma_port_space")
        .derive_copy(true)
        .derive_debug(false)
        .derive_default(false)
        .generate_comments(false)
        //.generate_inline_functions(true)
        //.default_macro_constant_type(bindgen::MacroTypeVariation::Unsigned)
        .prepend_enum_name(false)
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .disable_untagged_union()
        .parse_callbacks(Box::new(BindGenCallback))
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("bindings.rs");

    bindings
        .write_to_file(dest_path)
        .expect("Could not write bindings");
}
