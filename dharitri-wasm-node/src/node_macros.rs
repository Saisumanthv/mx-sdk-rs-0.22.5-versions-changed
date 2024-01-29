#[macro_export]
macro_rules! wasm_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)+ ) ) => {
        pub use dharitri_wasm_output;

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name(dharitri_wasm_node::vm_api());
            }
        )+
    };
}

#[macro_export]
macro_rules! wasm_empty_callback {
    () => {
        #[allow(non_snake_case)]
        #[no_mangle]
        fn callBack() {}
    };
}
