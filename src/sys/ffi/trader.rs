// 由 ctp-generator 根据官方 SPI 与事件审核表生成，请勿手工编辑。

use crate::sys::events::trader::*;

#[allow(dead_code, clippy::missing_safety_doc)]
#[cxx::bridge(namespace = "ctp_base::trader")]
mod bridge {
    unsafe extern "C++" {
        include!("cpp/bridge.h");

        type TraderApiBridge;
        type TraderSpiBridge;

        fn create_trader_api(
            sink: Box<TraderSpiSink>,
            flow_path: &str,
            using_udp: bool,
            multicast: bool,
            production_mode: bool,
        ) -> UniquePtr<TraderApiBridge>;
        fn create_trader_spi(sink: Box<TraderSpiSink>) -> UniquePtr<TraderSpiBridge>;
        fn version(self: &TraderApiBridge) -> String;
        fn init(self: &TraderApiBridge);
        fn join(self: &TraderApiBridge) -> i32;
        fn trading_day(self: &TraderApiBridge) -> String;
        fn register_front(self: &TraderApiBridge, address: &str);
        fn register_name_server(self: &TraderApiBridge, address: &str);
        unsafe fn front_info(self: &TraderApiBridge, info: *mut u8);
        unsafe fn register_fens_user_info(self: &TraderApiBridge, request: *mut u8);
        fn subscribe_private(self: &TraderApiBridge, resume: i32);
        fn subscribe_public(self: &TraderApiBridge, resume: i32);
        unsafe fn authenticate(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn login(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn logout(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn confirm_settlement(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn settlement_confirmation(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_settlement_info(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_investor_position_detail(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_product(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn query_exchange(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn query_trading_code(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_instrument_margin_rate(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_instrument_commission_rate(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_instrument_order_comm_rate(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_max_order_volume(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_depth_market_data(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_order(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn query_trade(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn query_investor_position(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_trading_account(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_instrument(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn insert_order(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn cancel_order(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn register_user_system_info(self: &TraderApiBridge, request: *mut u8);
        unsafe fn submit_user_system_info(self: &TraderApiBridge, request: *mut u8);
        unsafe fn register_wechat_user_system_info(self: &TraderApiBridge, request: *mut u8);
        unsafe fn submit_wechat_user_system_info(self: &TraderApiBridge, request: *mut u8);
        unsafe fn user_password_update(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn trading_account_password_update(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn user_auth_method(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn gen_user_captcha(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn gen_user_text(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn user_login_with_captcha(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn user_login_with_text(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn user_login_with_otp(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn parked_order_insert(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn cancel_parked_order(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn remove_parked_order(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn remove_parked_order_action(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn exec_order_insert(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn exec_order_action(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn for_quote_insert(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn quote_insert(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn quote_action(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn batch_order_action(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn option_self_close_insert(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn option_self_close_action(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn comb_action_insert(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_investor(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn query_user_session(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_trader_offer(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_transfer_bank(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_notice(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn query_investor_position_combine_detail(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_cfmmc_trading_account_key(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_ewarrant_offset(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_exchange_rate(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_product_exch_rate(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_product_group(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_exec_order(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_for_quote(self: &TraderApiBridge, request: *mut u8, request_id: i32)
        -> i32;
        unsafe fn query_quote(self: &TraderApiBridge, request: *mut u8, request_id: i32) -> i32;
        unsafe fn query_option_self_close(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_invest_unit(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_comb_instrument_guard(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_comb_action(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_transfer_serial(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_account_register(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_contract_bank(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_parked_order(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_parked_actions(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn trading_notices(self: &TraderApiBridge, request: *mut u8, request_id: i32)
        -> i32;
        unsafe fn query_cfmmc_trading_account_token(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_classified_instrument(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_comb_promotion_param(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_investor_product_group_margin(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_exchange_margin_rate(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_exchange_margin_rate_adjust(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_mm_instrument_commission_rate(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_mm_option_instr_comm_rate(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_option_instr_trade_cost(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_option_instr_comm_rate(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_risk_settle_invest_position(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_risk_settle_product_status(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_spbm_future_parameter(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_spbm_option_parameter(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_spbm_intra_parameter(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_spbm_inter_parameter(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_spbm_portf_definition(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_spbm_investor_portf_def(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_investor_portf_margin_ratio(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_investor_prod_spbm_detail(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_investor_commodity_spmm_margin(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_investor_commodity_group_spmm_margin(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_spmm_inst_param(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_spmm_product_param(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_spbm_add_on_inter_parameter(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_rcams_comb_product_info(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_rcams_instr_parameter(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_rcams_intra_parameter(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_rcams_inter_parameter(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_rcams_short_opt_adjust_param(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_rcams_investor_comb_position(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_investor_prod_rcams_margin(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_rule_instr_parameter(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_rule_intra_parameter(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_rule_inter_parameter(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_investor_prod_rule_margin(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn query_investor_portf_setting(
            self: &TraderApiBridge,
            request: *mut u8,
            request_id: i32,
        ) -> i32;
        unsafe fn emit_trader_order(spi: Pin<&mut TraderSpiBridge>, data: *mut u8);
        unsafe fn emit_trader_callback(
            spi: Pin<&mut TraderSpiBridge>,
            callback_index: u32,
            first: *mut u8,
            second: *mut u8,
            request_id: i32,
            last: bool,
        );
    }

    extern "Rust" {
        type TraderSpiSink;

        fn connected(sink: &mut TraderSpiSink);
        fn disconnected(sink: &mut TraderSpiSink, reason: i32);
        fn heartbeat(sink: &mut TraderSpiSink, time_lapse: i32);
        unsafe fn authenticate_response(
            sink: &mut TraderSpiSink,
            rsp_authenticate: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        fn private_seq_no_report(sink: &mut TraderSpiSink, seq_no: i32);
        unsafe fn login_response(
            sink: &mut TraderSpiSink,
            rsp_user_login: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn logout_response(
            sink: &mut TraderSpiSink,
            user_logout: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn user_password_update_response(
            sink: &mut TraderSpiSink,
            user_password_update: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn trading_account_password_update_response(
            sink: &mut TraderSpiSink,
            trading_account_password_update: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn user_auth_method_response(
            sink: &mut TraderSpiSink,
            rsp_user_auth_method: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn gen_user_captcha_response(
            sink: &mut TraderSpiSink,
            rsp_gen_user_captcha: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn gen_user_text_response(
            sink: &mut TraderSpiSink,
            rsp_gen_user_text: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn order_insert_response(
            sink: &mut TraderSpiSink,
            input_order: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn parked_order_insert_response(
            sink: &mut TraderSpiSink,
            parked_order: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn parked_action_response(
            sink: &mut TraderSpiSink,
            parked_order_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn order_action_response(
            sink: &mut TraderSpiSink,
            input_order_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn max_order_volume_response(
            sink: &mut TraderSpiSink,
            qry_max_order_volume: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn settlement_confirm_response(
            sink: &mut TraderSpiSink,
            settlement_info_confirm: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn remove_parked_order_response(
            sink: &mut TraderSpiSink,
            remove_parked_order: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn remove_parked_order_action_response(
            sink: &mut TraderSpiSink,
            remove_parked_order_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn exec_order_insert_response(
            sink: &mut TraderSpiSink,
            input_exec_order: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn exec_order_action_response(
            sink: &mut TraderSpiSink,
            input_exec_order_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn for_quote_insert_response(
            sink: &mut TraderSpiSink,
            input_for_quote: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn quote_insert_response(
            sink: &mut TraderSpiSink,
            input_quote: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn quote_action_response(
            sink: &mut TraderSpiSink,
            input_quote_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn batch_order_action_response(
            sink: &mut TraderSpiSink,
            input_batch_order_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn option_self_close_insert_response(
            sink: &mut TraderSpiSink,
            input_option_self_close: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn option_self_close_action_response(
            sink: &mut TraderSpiSink,
            input_option_self_close_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn comb_action_insert_response(
            sink: &mut TraderSpiSink,
            input_comb_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn order_response(
            sink: &mut TraderSpiSink,
            order: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn trade_response(
            sink: &mut TraderSpiSink,
            trade: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_position_response(
            sink: &mut TraderSpiSink,
            investor_position: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn trading_account_response(
            sink: &mut TraderSpiSink,
            trading_account: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_response(
            sink: &mut TraderSpiSink,
            investor: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn trading_code_response(
            sink: &mut TraderSpiSink,
            trading_code: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn instrument_margin_rate_response(
            sink: &mut TraderSpiSink,
            instrument_margin_rate: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn instrument_commission_rate_response(
            sink: &mut TraderSpiSink,
            instrument_commission_rate: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn user_session_response(
            sink: &mut TraderSpiSink,
            user_session: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn exchange_response(
            sink: &mut TraderSpiSink,
            exchange: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn product_response(
            sink: &mut TraderSpiSink,
            product: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn instrument_response(
            sink: &mut TraderSpiSink,
            instrument: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn depth_market_data_response(
            sink: &mut TraderSpiSink,
            depth_market_data: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn trader_offer_response(
            sink: &mut TraderSpiSink,
            trader_offer: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn settlement_info_response(
            sink: &mut TraderSpiSink,
            settlement_info: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn transfer_bank_response(
            sink: &mut TraderSpiSink,
            transfer_bank: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_position_detail_response(
            sink: &mut TraderSpiSink,
            investor_position_detail: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn notice_response(
            sink: &mut TraderSpiSink,
            notice: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn settlement_status(
            sink: &mut TraderSpiSink,
            settlement_info_confirm: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_position_combine_detail_response(
            sink: &mut TraderSpiSink,
            investor_position_combine_detail: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn cfmmc_trading_account_key_response(
            sink: &mut TraderSpiSink,
            cfmmc_trading_account_key: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn e_warrant_offset_response(
            sink: &mut TraderSpiSink,
            e_warrant_offset: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_product_group_margin_response(
            sink: &mut TraderSpiSink,
            investor_product_group_margin: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn exchange_margin_rate_response(
            sink: &mut TraderSpiSink,
            exchange_margin_rate: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn exchange_margin_rate_adjust_response(
            sink: &mut TraderSpiSink,
            exchange_margin_rate_adjust: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn exchange_rate_response(
            sink: &mut TraderSpiSink,
            exchange_rate: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn sec_agent_acid_map_response(
            sink: &mut TraderSpiSink,
            sec_agent_acid_map: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn product_exchange_rate_response(
            sink: &mut TraderSpiSink,
            product_exch_rate: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn product_group_response(
            sink: &mut TraderSpiSink,
            product_group: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn mm_instrument_commission_rate_response(
            sink: &mut TraderSpiSink,
            mm_instrument_commission_rate: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn mm_option_instrument_commission_rate_response(
            sink: &mut TraderSpiSink,
            mm_option_instr_comm_rate: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn instrument_order_commission_rate_response(
            sink: &mut TraderSpiSink,
            instrument_order_comm_rate: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn sec_agent_trading_account_response(
            sink: &mut TraderSpiSink,
            trading_account: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn sec_agent_check_mode_response(
            sink: &mut TraderSpiSink,
            sec_agent_check_mode: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn sec_agent_trade_info_response(
            sink: &mut TraderSpiSink,
            sec_agent_trade_info: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn option_instrument_trade_cost_response(
            sink: &mut TraderSpiSink,
            option_instr_trade_cost: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn option_instrument_commission_rate_response(
            sink: &mut TraderSpiSink,
            option_instr_comm_rate: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn exec_order_response(
            sink: &mut TraderSpiSink,
            exec_order: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn for_quote_query_response(
            sink: &mut TraderSpiSink,
            for_quote: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn quote_response(
            sink: &mut TraderSpiSink,
            quote: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn option_self_close_response(
            sink: &mut TraderSpiSink,
            option_self_close: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn invest_unit_response(
            sink: &mut TraderSpiSink,
            invest_unit: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn comb_instrument_guard_response(
            sink: &mut TraderSpiSink,
            comb_instrument_guard: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn comb_action_response(
            sink: &mut TraderSpiSink,
            comb_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn transfer_serial_response(
            sink: &mut TraderSpiSink,
            transfer_serial: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn account_register_response(
            sink: &mut TraderSpiSink,
            account_register: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn error(
            sink: &mut TraderSpiSink,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn order_update(sink: &mut TraderSpiSink, order: *const u8);
        unsafe fn execution(sink: &mut TraderSpiSink, trade: *const u8);
        unsafe fn order_insert_error(
            sink: &mut TraderSpiSink,
            input_order: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn order_action_error(
            sink: &mut TraderSpiSink,
            order_action: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn instrument_status(sink: &mut TraderSpiSink, instrument_status: *const u8);
        unsafe fn bulletin(sink: &mut TraderSpiSink, bulletin: *const u8);
        unsafe fn notice_received(sink: &mut TraderSpiSink, trading_notice_info: *const u8);
        unsafe fn conditional_order_error(
            sink: &mut TraderSpiSink,
            error_conditional_order: *const u8,
        );
        unsafe fn exec_order_report(sink: &mut TraderSpiSink, exec_order: *const u8);
        unsafe fn exec_order_insert_error(
            sink: &mut TraderSpiSink,
            input_exec_order: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn exec_order_action_error(
            sink: &mut TraderSpiSink,
            exec_order_action: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn for_quote_insert_error(
            sink: &mut TraderSpiSink,
            input_for_quote: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn quote_report(sink: &mut TraderSpiSink, quote: *const u8);
        unsafe fn quote_insert_error(
            sink: &mut TraderSpiSink,
            input_quote: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn quote_action_error(
            sink: &mut TraderSpiSink,
            quote_action: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn for_quote_response(sink: &mut TraderSpiSink, for_quote_rsp: *const u8);
        unsafe fn cfmmc_account_token(
            sink: &mut TraderSpiSink,
            cfmmc_trading_account_token: *const u8,
        );
        unsafe fn batch_order_action_error(
            sink: &mut TraderSpiSink,
            batch_order_action: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn option_self_close_report(sink: &mut TraderSpiSink, option_self_close: *const u8);
        unsafe fn option_self_close_insert_error(
            sink: &mut TraderSpiSink,
            input_option_self_close: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn option_self_close_action_error(
            sink: &mut TraderSpiSink,
            option_self_close_action: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn comb_action_report(sink: &mut TraderSpiSink, comb_action: *const u8);
        unsafe fn comb_action_insert_error(
            sink: &mut TraderSpiSink,
            input_comb_action: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn contract_bank_response(
            sink: &mut TraderSpiSink,
            contract_bank: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn parked_order_response(
            sink: &mut TraderSpiSink,
            parked_order: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn parked_order_actions(
            sink: &mut TraderSpiSink,
            parked_order_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn notice_history_response(
            sink: &mut TraderSpiSink,
            trading_notice: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn broker_trading_parameters_response(
            sink: &mut TraderSpiSink,
            broker_trading_params: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn broker_trading_algos_response(
            sink: &mut TraderSpiSink,
            broker_trading_algos: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn query_cfmmc_trading_account_token_response(
            sink: &mut TraderSpiSink,
            query_cfmmc_trading_account_token: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn from_bank_to_future_by_bank_report(
            sink: &mut TraderSpiSink,
            rsp_transfer: *const u8,
        );
        unsafe fn from_future_to_bank_by_bank_report(
            sink: &mut TraderSpiSink,
            rsp_transfer: *const u8,
        );
        unsafe fn repeal_from_bank_to_future_by_bank_report(
            sink: &mut TraderSpiSink,
            rsp_repeal: *const u8,
        );
        unsafe fn repeal_from_future_to_bank_by_bank_report(
            sink: &mut TraderSpiSink,
            rsp_repeal: *const u8,
        );
        unsafe fn from_bank_to_future_by_future_report(
            sink: &mut TraderSpiSink,
            rsp_transfer: *const u8,
        );
        unsafe fn from_future_to_bank_by_future_report(
            sink: &mut TraderSpiSink,
            rsp_transfer: *const u8,
        );
        unsafe fn repeal_from_bank_to_future_by_future_manual_report(
            sink: &mut TraderSpiSink,
            rsp_repeal: *const u8,
        );
        unsafe fn repeal_from_future_to_bank_by_future_manual_report(
            sink: &mut TraderSpiSink,
            rsp_repeal: *const u8,
        );
        unsafe fn query_bank_balance_by_future_report(
            sink: &mut TraderSpiSink,
            notify_query_account: *const u8,
        );
        unsafe fn bank_to_future_by_future_error(
            sink: &mut TraderSpiSink,
            req_transfer: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn future_to_bank_by_future_error(
            sink: &mut TraderSpiSink,
            req_transfer: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn repeal_bank_to_future_by_future_manual_error(
            sink: &mut TraderSpiSink,
            req_repeal: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn repeal_future_to_bank_by_future_manual_error(
            sink: &mut TraderSpiSink,
            req_repeal: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn query_bank_balance_by_future_error(
            sink: &mut TraderSpiSink,
            req_query_account: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn repeal_from_bank_to_future_by_future_report(
            sink: &mut TraderSpiSink,
            rsp_repeal: *const u8,
        );
        unsafe fn repeal_from_future_to_bank_by_future_report(
            sink: &mut TraderSpiSink,
            rsp_repeal: *const u8,
        );
        unsafe fn from_bank_to_future_by_future_response(
            sink: &mut TraderSpiSink,
            req_transfer: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn from_future_to_bank_by_future_response(
            sink: &mut TraderSpiSink,
            req_transfer: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn query_bank_account_money_by_future_response(
            sink: &mut TraderSpiSink,
            req_query_account: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn open_account_by_bank_report(sink: &mut TraderSpiSink, open_account: *const u8);
        unsafe fn cancel_account_by_bank_report(
            sink: &mut TraderSpiSink,
            cancel_account: *const u8,
        );
        unsafe fn change_account_by_bank_report(
            sink: &mut TraderSpiSink,
            change_account: *const u8,
        );
        unsafe fn classified_instrument_response(
            sink: &mut TraderSpiSink,
            instrument: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn comb_promotion_parameter_response(
            sink: &mut TraderSpiSink,
            comb_promotion_param: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn risk_settle_investor_position_response(
            sink: &mut TraderSpiSink,
            risk_settle_invest_position: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn risk_settle_product_status_response(
            sink: &mut TraderSpiSink,
            risk_settle_product_status: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spbm_future_parameter_response(
            sink: &mut TraderSpiSink,
            spbm_future_parameter: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spbm_option_parameter_response(
            sink: &mut TraderSpiSink,
            spbm_option_parameter: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spbm_intra_parameter_response(
            sink: &mut TraderSpiSink,
            spbm_intra_parameter: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spbm_inter_parameter_response(
            sink: &mut TraderSpiSink,
            spbm_inter_parameter: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spbm_portfolio_definition_response(
            sink: &mut TraderSpiSink,
            spbm_portf_definition: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spbm_investor_portfolio_definition_response(
            sink: &mut TraderSpiSink,
            spbm_investor_portf_def: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_portfolio_margin_ratio_response(
            sink: &mut TraderSpiSink,
            investor_portf_margin_ratio: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_product_spbm_detail_response(
            sink: &mut TraderSpiSink,
            investor_prod_spbm_detail: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_commodity_spmm_margin_response(
            sink: &mut TraderSpiSink,
            investor_commodity_spmm_margin: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_commodity_group_spmm_margin_response(
            sink: &mut TraderSpiSink,
            investor_commodity_group_spmm_margin: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spmm_instrument_parameter_response(
            sink: &mut TraderSpiSink,
            spmm_inst_param: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spmm_product_parameter_response(
            sink: &mut TraderSpiSink,
            spmm_product_param: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spbm_add_on_inter_parameter_response(
            sink: &mut TraderSpiSink,
            spbm_add_on_inter_parameter: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn rcams_comb_product_info_response(
            sink: &mut TraderSpiSink,
            rcams_comb_product_info: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn rcams_instrument_parameter_response(
            sink: &mut TraderSpiSink,
            rcams_instr_parameter: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn rcams_intra_parameter_response(
            sink: &mut TraderSpiSink,
            rcams_intra_parameter: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn rcams_inter_parameter_response(
            sink: &mut TraderSpiSink,
            rcams_inter_parameter: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn rcams_short_opt_adjust_parameter_response(
            sink: &mut TraderSpiSink,
            rcams_short_opt_adjust_param: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn rcams_investor_comb_position_response(
            sink: &mut TraderSpiSink,
            rcams_investor_comb_position: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_product_rcams_margin_response(
            sink: &mut TraderSpiSink,
            investor_prod_rcams_margin: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn rule_instrument_parameter_response(
            sink: &mut TraderSpiSink,
            rule_instr_parameter: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn rule_intra_parameter_response(
            sink: &mut TraderSpiSink,
            rule_intra_parameter: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn rule_inter_parameter_response(
            sink: &mut TraderSpiSink,
            rule_inter_parameter: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_product_rule_margin_response(
            sink: &mut TraderSpiSink,
            investor_prod_rule_margin: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_portfolio_setting_response(
            sink: &mut TraderSpiSink,
            investor_portf_setting: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn investor_info_commission_record_response(
            sink: &mut TraderSpiSink,
            investor_info_comm_rec: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn comb_leg_response(
            sink: &mut TraderSpiSink,
            comb_leg: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn set_offset_response(
            sink: &mut TraderSpiSink,
            input_offset_setting: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn cancel_offset_setting_response(
            sink: &mut TraderSpiSink,
            input_offset_setting: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn offset_setting_report(sink: &mut TraderSpiSink, offset_setting: *const u8);
        unsafe fn offset_setting_error(
            sink: &mut TraderSpiSink,
            input_offset_setting: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn cancel_offset_setting_error(
            sink: &mut TraderSpiSink,
            cancel_offset_setting: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn offset_settings_response(
            sink: &mut TraderSpiSink,
            offset_setting: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn gen_sms_code_response(
            sink: &mut TraderSpiSink,
            rsp_gen_sms_code: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spd_apply_response(
            sink: &mut TraderSpiSink,
            input_spd_apply: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spd_apply_action_response(
            sink: &mut TraderSpiSink,
            input_spd_apply_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spd_apply_query_response(
            sink: &mut TraderSpiSink,
            spd_apply: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn spd_apply_report(sink: &mut TraderSpiSink, spd_apply: *const u8);
        unsafe fn spd_apply_error(
            sink: &mut TraderSpiSink,
            input_spd_apply: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn spd_apply_action_error(
            sink: &mut TraderSpiSink,
            spd_apply_action: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn hedge_cfm_response(
            sink: &mut TraderSpiSink,
            input_hedge_cfm: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn hedge_cfm_action_response(
            sink: &mut TraderSpiSink,
            input_hedge_cfm_action: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn hedge_cfm_query_response(
            sink: &mut TraderSpiSink,
            hedge_cfm: *const u8,
            rsp_info: *const u8,
            request_id: i32,
            is_last: bool,
        );
        unsafe fn hedge_cfm_report(sink: &mut TraderSpiSink, hedge_cfm: *const u8);
        unsafe fn hedge_cfm_error(
            sink: &mut TraderSpiSink,
            input_hedge_cfm: *const u8,
            rsp_info: *const u8,
        );
        unsafe fn hedge_cfm_action_error(
            sink: &mut TraderSpiSink,
            hedge_cfm_action: *const u8,
            rsp_info: *const u8,
        );
    }
}

#[allow(unused_imports)]
pub use bridge::*;
